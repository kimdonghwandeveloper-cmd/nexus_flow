pub mod simulation {
    tonic::include_proto!("nexus.simulation");
}

use anyhow::{Context, Result};
use simulation::nexus_simulation_service_client::NexusSimulationServiceClient;
pub use simulation::{
    Node as ProtoNode, NodeParameters as ProtoNodeParameters, NodeState as ProtoNodeState,
    ParameterChange as ProtoParameterChange, SimulationRequest as ProtoSimulationRequest,
    SimulationResponse as ProtoSimulationResponse, Topology as ProtoTopology,
    Edge as ProtoEdge, SimulationConfig as ProtoSimulationConfig,
};
use tonic::transport::Channel;
use tracing::info;

use crate::models::{self, Topology};

/// Python AI 엔진과 통신하는 gRPC 클라이언트
#[derive(Clone)]
pub struct SimulationClient {
    client: NexusSimulationServiceClient<Channel>,
}

impl SimulationClient {
    /// gRPC 서버에 연결합니다.
    pub async fn connect(host: &str, port: u16) -> Result<Self> {
        let addr = format!("http://{}:{}", host, port);
        info!("Connecting to AI Engine gRPC at {}", addr);
        
        let client = NexusSimulationServiceClient::connect(addr)
            .await
            .context("Failed to connect to AI Engine gRPC server")?;
            
        Ok(Self { client })
    }

    /// 단일 시뮬레이션을 실행합니다.
    pub async fn run_simulation(
        &self,
        request_id: String,
        topology: &Topology,
        parameter_changes: Vec<crate::messages::ParameterUpdate>,
    ) -> Result<ProtoSimulationResponse> {
        let proto_topology = self.map_topology_to_proto(topology);
        let proto_changes = parameter_changes
            .into_iter()
            .map(|c| ProtoParameterChange {
                node_id: c.node_id,
                param_name: c.param_name,
                new_value: c.new_value,
            })
            .collect();

        let request = ProtoSimulationRequest {
            request_id,
            topology: Some(proto_topology),
            parameter_changes: proto_changes,
        };

        let response = self.client.clone().run_simulation(request).await?;
        Ok(response.into_inner())
    }

    /// 토폴로지 모델을 Protobuf 메시지로 변환합니다.
    fn map_topology_to_proto(&self, topo: &Topology) -> ProtoTopology {
        ProtoTopology {
            nodes: topo
                .nodes
                .iter()
                .map(|n| ProtoNode {
                    id: n.id.clone(),
                    r#type: format!("{:?}", n.node_type).to_lowercase(),
                    label: n.data.label.clone(),
                    parameters: Some(ProtoNodeParameters {
                        cycle_time: n.data.parameters.cycle_time,
                        failure_rate: n.data.parameters.failure_rate,
                        processing_capacity: n.data.parameters.processing_capacity,
                    }),
                    state: n.data.state.as_ref().map(|s| ProtoNodeState {
                        status: format!("{:?}", s.status).to_lowercase(),
                        current_utilization: s.current_utilization,
                    }),
                })
                .collect(),
            edges: topo
                .edges
                .iter()
                .map(|e| ProtoEdge {
                    id: e.id.clone(),
                    source: e.source.clone(),
                    target: e.target.clone(),
                    r#type: format!("{:?}", e.edge_type).to_lowercase(),
                    latency: e.data.as_ref().map(|d| d.latency).unwrap_or(0.0),
                })
                .collect(),
            config: topo.simulation_config.as_ref().map(|c| ProtoSimulationConfig {
                duration_seconds: c.duration_seconds,
                time_step: c.time_step,
                random_seed: c.random_seed.unwrap_or(0),
            }),
        }
    }
}
