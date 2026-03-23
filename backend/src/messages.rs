use serde::{Deserialize, Serialize};

use crate::models::Topology;

// ============================================================
//  WebSocket JSON 메시지 프로토콜
//  Frontend <-> Rust Backend 간 양방향 통신 규격
// ============================================================

/// Frontend → Backend 인바운드 메시지
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "action", content = "payload")]
#[serde(rename_all = "snake_case")]
pub enum ClientMessage {
    /// 토폴로지 전체 동기화 (초기 로드 또는 전체 업데이트)
    SyncTopology {
        project_id: String,
        topology: Topology,
    },

    /// 단일 노드 파라미터 변경 (Debounced)
    UpdateParameter(ParameterUpdate),

    /// What-If 시뮬레이션 실행 요청
    RunSimulation(SimulationRequest),

    /// 현재 서버 상태 조회
    Ping,
}

/// Backend → Frontend 아웃바운드 메시지
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "event", content = "payload")]
#[serde(rename_all = "snake_case")]
pub enum ServerMessage {
    /// 토폴로지 동기화 확인 응답
    TopologySynced(SyncAck),

    /// 파라미터 업데이트 확인
    ParameterUpdated(ParameterUpdateAck),

    /// 시뮬레이션 결과 수신
    SimulationResult(SimulationResultPayload),

    /// 에러 발생
    Error(ErrorPayload),

    /// Ping 응답
    Pong,
}

// ---- Payload 구조체들 ----

/// 파라미터 변경 요청
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParameterUpdate {
    pub node_id: String,
    pub param_name: String,
    pub new_value: f64,
}

/// 시뮬레이션 실행 요청
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimulationRequest {
    pub request_id: String,
    pub parameter_changes: Vec<ParameterUpdate>,
}

/// 동기화 확인 응답
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncAck {
    pub node_count: usize,
    pub edge_count: usize,
    pub message: String,
}

/// 파라미터 업데이트 확인 응답
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParameterUpdateAck {
    pub node_id: String,
    pub param_name: String,
    pub accepted: bool,
}

/// 시뮬레이션 결과
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimulationResultPayload {
    pub request_id: String,
    pub success: bool,
    pub overall_throughput: Option<f64>,
    pub overall_efficiency: Option<f64>,
    pub node_results: Vec<serde_json::Value>, // 향후 구체적 타입으로 교체
    pub impact_chain: Vec<serde_json::Value>,  // 향후 구체적 타입으로 교체
}

/// 에러 페이로드
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorPayload {
    pub code: String,
    pub message: String,
}
