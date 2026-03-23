use std::net::SocketAddr;
use std::sync::Arc;

use anyhow::Result;
use futures_util::{SinkExt, StreamExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::RwLock;
use tokio_tungstenite::accept_async;
use tokio_tungstenite::tungstenite::Message;
use tracing::{error, info, warn};

use crate::grpc_client::SimulationClient;
use crate::messages::{
    ClientMessage, ErrorPayload, ParameterUpdateAck, ServerMessage, SimulationResultPayload,
    SyncAck,
};
use crate::models::Topology;

/// 공유 애플리케이션 상태 (스레드 안전)
pub struct AppState {
    pub topology: RwLock<Option<Topology>>,
    pub grpc_client: SimulationClient,
}

impl AppState {
    pub fn new(grpc_client: SimulationClient) -> Self {
        Self {
            topology: RwLock::new(None),
            grpc_client,
        }
    }
}

/// WebSocket 서버 시작
pub async fn start_ws_server(port: u16, state: Arc<AppState>) -> Result<()> {
    let addr = format!("0.0.0.0:{}", port);
    let listener = TcpListener::bind(&addr).await?;
    info!("WebSocket server listening on ws://{}", addr);

    while let Ok((stream, peer_addr)) = listener.accept().await {
        let state = Arc::clone(&state);
        tokio::spawn(async move {
            if let Err(e) = handle_connection(stream, peer_addr, state).await {
                error!("Connection error from {}: {}", peer_addr, e);
            }
        });
    }

    Ok(())
}

/// 개별 WebSocket 연결 처리
async fn handle_connection(
    stream: TcpStream,
    peer_addr: SocketAddr,
    state: Arc<AppState>,
) -> Result<()> {
    let ws_stream = accept_async(stream).await?;
    info!("New WebSocket connection from: {}", peer_addr);

    let (mut ws_sender, mut ws_receiver) = ws_stream.split();

    while let Some(msg) = ws_receiver.next().await {
        let msg = match msg {
            Ok(m) => m,
            Err(e) => {
                warn!("WebSocket receive error from {}: {}", peer_addr, e);
                break;
            }
        };

        match msg {
            Message::Text(text) => {
                let response = process_message(&text, &state).await;
                let response_json = serde_json::to_string(&response)
                    .unwrap_or_else(|_| r#"{"event":"error","payload":{"code":"SERIALIZE_ERROR","message":"Failed to serialize response"}}"#.to_string());
                if let Err(e) = ws_sender.send(Message::Text(response_json.into())).await {
                    error!("Failed to send response to {}: {}", peer_addr, e);
                    break;
                }
            }
            Message::Close(_) => {
                info!("Client {} disconnected", peer_addr);
                break;
            }
            Message::Ping(data) => {
                if let Err(e) = ws_sender.send(Message::Pong(data)).await {
                    error!("Failed to send pong to {}: {}", peer_addr, e);
                    break;
                }
            }
            _ => {} // Binary, Pong 등은 무시
        }
    }

    info!("Connection closed: {}", peer_addr);
    Ok(())
}

/// 인바운드 JSON 메시지를 파싱하고 비즈니스 로직을 실행
async fn process_message(text: &str, state: &AppState) -> ServerMessage {
    // 1. JSON 파싱
    let client_msg: ClientMessage = match serde_json::from_str(text) {
        Ok(msg) => msg,
        Err(e) => {
            warn!("Invalid JSON received: {}", e);
            return ServerMessage::Error(ErrorPayload {
                code: "PARSE_ERROR".to_string(),
                message: format!("Invalid JSON: {}", e),
            });
        }
    };

    // 2. 메시지 타입별 처리
    match client_msg {
        ClientMessage::SyncTopology(topology) => {
            let node_count = topology.nodes.len();
            let edge_count = topology.edges.len();
            info!(
                "Topology synced: {} nodes, {} edges",
                node_count, edge_count
            );

            // 상태에 토폴로지 저장
            let mut topo_state = state.topology.write().await;
            *topo_state = Some(topology);

            ServerMessage::TopologySynced(SyncAck {
                node_count,
                edge_count,
                message: "Topology synchronized successfully".to_string(),
            })
        }

        ClientMessage::UpdateParameter(update) => {
            info!(
                "Parameter update: node={}, param={}, value={}",
                update.node_id, update.param_name, update.new_value
            );

            // 현재 토폴로지에서 해당 노드 파라미터 업데이트
            let mut topo_state = state.topology.write().await;
            if let Some(ref mut topology) = *topo_state {
                if let Some(node) = topology.nodes.iter_mut().find(|n| n.id == update.node_id) {
                    let accepted = match update.param_name.as_str() {
                        "cycle_time" => {
                            node.data.parameters.cycle_time = update.new_value;
                            true
                        }
                        "failure_rate" => {
                            node.data.parameters.failure_rate = update.new_value;
                            true
                        }
                        "processing_capacity" => {
                            node.data.parameters.processing_capacity = update.new_value as i32;
                            true
                        }
                        _ => false,
                    };

                    ServerMessage::ParameterUpdated(ParameterUpdateAck {
                        node_id: update.node_id,
                        param_name: update.param_name,
                        accepted,
                    })
                } else {
                    ServerMessage::Error(ErrorPayload {
                        code: "NODE_NOT_FOUND".to_string(),
                        message: format!("Node '{}' not found in topology", update.node_id),
                    })
                }
            } else {
                ServerMessage::Error(ErrorPayload {
                    code: "NO_TOPOLOGY".to_string(),
                    message: "No topology loaded. Send SyncTopology first.".to_string(),
                })
            }
        }

        ClientMessage::RunSimulation(request) => {
            info!("Simulation requested: id={}", request.request_id);

            // gRPC로 Python AI 엔진에 전달
            let topo_state = state.topology.read().await;
            if let Some(ref topology) = *topo_state {
                match state
                    .grpc_client
                    .run_simulation(
                        request.request_id.clone(),
                        topology,
                        request.parameter_changes,
                    )
                    .await
                {
                    Ok(resp) => ServerMessage::SimulationResult(SimulationResultPayload {
                        request_id: resp.request_id,
                        success: resp.success,
                        overall_throughput: Some(resp.overall_throughput),
                        overall_efficiency: Some(resp.overall_efficiency),
                        node_results: vec![], // TODO: 구체적 매핑
                        impact_chain: vec![], // TODO: 구체적 매핑
                    }),
                    Err(e) => ServerMessage::Error(ErrorPayload {
                        code: "GRPC_ERROR".to_string(),
                        message: format!("AI Engine communication failed: {}", e),
                    }),
                }
            } else {
                ServerMessage::Error(ErrorPayload {
                    code: "NO_TOPOLOGY".to_string(),
                    message: "No topology loaded. Send SyncTopology first.".to_string(),
                })
            }
        }

        ClientMessage::Ping => ServerMessage::Pong,
    }
}
