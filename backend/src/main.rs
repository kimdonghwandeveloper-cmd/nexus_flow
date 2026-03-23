use anyhow::Result;
use tracing::{info, warn};
use tracing_subscriber::{EnvFilter, fmt};

mod config;
mod models;

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

    // 4. 서브시스템 초기화 (향후 확장 지점)
    // TODO [Step 2-2]: WebSocket 서버 시작
    // TODO [Step 2-3]: 그래프 엔진 초기화
    // TODO [Step 2-4]: gRPC 클라이언트 연결
    // TODO [Step 2-5]: Supabase 클라이언트 초기화

    info!("All subsystems initialized. Server is ready.");

    // Graceful shutdown 대기
    tokio::signal::ctrl_c().await?;
    warn!("Shutdown signal received. Cleaning up...");

    Ok(())
}
