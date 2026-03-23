use anyhow::Result;
use postgrest::Postgrest;
use serde_json::Value;
use tracing::info;

use crate::config::AppConfig;

/// Supabase 클라이언트 Wrapper
#[derive(Clone)]
pub struct SupabaseClient {
    client: Postgrest,
}

impl SupabaseClient {
    /// Supabase 클라이언트를 초기화합니다.
    pub fn new(config: &AppConfig) -> Self {
        let client = Postgrest::new(&config.supabase_url)
            .insert_header("apikey", &config.supabase_anon_key)
            .insert_header("Authorization", format!("Bearer {}", config.supabase_anon_key));
            
        Self { client }
    }

    /// 관리자 권한(service_role) 클라이언트를 초기화합니다.
    pub fn admin(config: &AppConfig) -> Self {
        let client = Postgrest::new(&config.supabase_url)
            .insert_header("apikey", &config.supabase_service_role_key)
            .insert_header("Authorization", format!("Bearer {}", config.supabase_service_role_key));
            
        Self { client }
    }

    /// 토폴로지 데이터를 저장합니다.
    pub async fn save_topology(
        &self,
        project_id: &str,
        version: i32,
        topology_data: Value,
    ) -> Result<()> {
        info!("Saving topology version {} for project {}", version, project_id);
        
        let body = serde_json::json!({
            "project_id": project_id,
            "version": version,
            "data": topology_data,
            "is_active": true
        });

        let resp = self.client
            .from("topologies")
            .insert(body.to_string())
            .execute()
            .await
            .map_err(|e| anyhow::anyhow!("Supabase request failed: {}", e))?;

        let status = resp.status();
        if !status.is_success() {
            let err_text = resp.text().await.unwrap_or_default();
            anyhow::bail!("Supabase error ({}): {}", status, err_text);
        }

        Ok(())
    }

    /// 시뮬레이션 결과를 저장합니다.
    pub async fn save_simulation_result(
        &self,
        topology_id: &str,
        request_id: &str,
        result: Value,
    ) -> Result<()> {
        info!("Saving simulation result for request {}", request_id);

        let body = serde_json::json!({
            "topology_id": topology_id,
            "request_id": request_id,
            "success": result["success"],
            "overall_throughput": result["overall_throughput"],
            "overall_efficiency": result["overall_efficiency"],
            "node_results": result["node_results"],
            "impact_chain": result["impact_chain"]
        });

        let resp = self.client
            .from("simulation_results")
            .insert(body.to_string())
            .execute()
            .await
            .map_err(|e| anyhow::anyhow!("Supabase request failed: {}", e))?;

        let status = resp.status();
        if !status.is_success() {
            let err_text = resp.text().await.unwrap_or_default();
            anyhow::bail!("Supabase error ({}): {}", status, err_text);
        }

        Ok(())
    }
}
