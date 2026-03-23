use std::sync::Arc;

use anyhow::Result;
use tracing::{info, warn};
use tracing_subscriber::{EnvFilter, fmt};

mod config;
mod graph;
mod grpc_client;
mod messages;
mod models;
pub mod supabase;
pub mod ws;

#[tokio::main]
async fn main() -> Result<()> {
    // 1. 환경변수 로드 (.env)
    dotenvy::dotenv().ok();

    // 2. 구조화된 로깅 초기화
    fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .init();

    // 3. 설정 로드
    let config = config::AppConfig::from_env()?;
    info!(
        "NEXUS-Flow Backend starting on port {}",
        config.websocket_port
    );

    // 4. gRPC 클라이언트 초기화 (Python AI 엔진 서버 연결)
    let grpc_client = grpc_client::SimulationClient::connect(
        &config.grpc_host,
        config.grpc_port
    ).await?;

    // 5. Supabase 클라이언트 초기화
    let supabase_client = supabase::SupabaseClient::new(&config);

    // 6. 공유 상태 초기화
    let state = Arc::new(ws::AppState::new(grpc_client, supabase_client));

    // 5. WebSocket 서버 시작 (별도 태스크)
    let ws_state = Arc::clone(&state);
    let ws_port = config.websocket_port;
    let ws_handle = tokio::spawn(async move {
        if let Err(e) = ws::start_ws_server(ws_port, ws_state).await {
            tracing::error!("WebSocket server error: {}", e);
        }
    });

    // TODO [Step 2-3]: 그래프 엔진 초기화
    // TODO [Step 2-4]: gRPC 클라이언트 연결
    // TODO [Step 2-5]: Supabase 클라이언트 초기화

    info!("All subsystems initialized. Server is ready.");

    // Graceful shutdown 대기
    tokio::signal::ctrl_c().await?;
    warn!("Shutdown signal received. Cleaning up...");

    // WebSocket 서버 태스크 중단
    ws_handle.abort();

    Ok(())
}
