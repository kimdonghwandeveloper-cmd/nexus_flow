use anyhow::{Context, Result};
use std::env;

/// 애플리케이션 전역 설정
#[derive(Debug, Clone)]
pub struct AppConfig {
    // Supabase
    pub supabase_url: String,
    pub supabase_anon_key: String,
    pub supabase_service_role_key: String,

    // Server
    pub websocket_port: u16,

    // gRPC (Python AI Engine)
    pub grpc_host: String,
    pub grpc_port: u16,
}

impl AppConfig {
    /// 환경변수에서 설정을 로드합니다.
    pub fn from_env() -> Result<Self> {
        Ok(Self {
            supabase_url: env::var("SUPABASE_URL")
                .context("SUPABASE_URL must be set")?,
            supabase_anon_key: env::var("SUPABASE_ANON_KEY")
                .context("SUPABASE_ANON_KEY must be set")?,
            supabase_service_role_key: env::var("SUPABASE_SERVICE_ROLE_KEY")
                .context("SUPABASE_SERVICE_ROLE_KEY must be set")?,
            websocket_port: env::var("WEBSOCKET_PORT")
                .unwrap_or_else(|_| "8081".to_string())
                .parse()
                .context("WEBSOCKET_PORT must be a valid port number")?,
            grpc_host: env::var("GRPC_HOST")
                .unwrap_or_else(|_| "127.0.0.1".to_string()),
            grpc_port: env::var("GRPC_PORT")
                .unwrap_or_else(|_| "50051".to_string())
                .parse()
                .context("GRPC_PORT must be a valid port number")?,
        })
    }
}
