use serde::{Deserialize, Serialize};

// ============================================================
//  SSOT JSON 스키마에 대응하는 Rust 데이터 모델
//  schema/nexus_flow_schema.json과 1:1 매핑
// ============================================================

/// 최상위 토폴로지 구조
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Topology {
    pub version: String,
    pub project: Project,
    pub nodes: Vec<Node>,
    pub edges: Vec<Edge>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub simulation_config: Option<SimulationConfig>,
}

/// 프로젝트 메타데이터
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
}

/// 공정 노드
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Node {
    pub id: String,
    #[serde(rename = "type")]
    pub node_type: NodeType,
    pub position: Position,
    pub data: NodeData,
}

/// 노드 타입 열거형
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum NodeType {
    Source,
    Assembler,
    Inspector,
    Buffer,
    Sink,
}

/// 2D 좌표 (React Flow 레이아웃)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Position {
    pub x: f64,
    pub y: f64,
}

/// 노드 데이터 (라벨, 파라미터, 상태)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeData {
    pub label: String,
    pub parameters: NodeParameters,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<NodeState>,
}

/// 노드 파라미터 (확률적 특성)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeParameters {
    #[serde(default)]
    pub cycle_time: f64,
    #[serde(default)]
    pub failure_rate: f64,
    #[serde(default = "default_capacity")]
    pub processing_capacity: i32,
}

fn default_capacity() -> i32 {
    1
}

/// 노드 상태
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeState {
    pub status: NodeStatus,
    #[serde(default)]
    pub current_utilization: f64,
}

/// 노드 상태 열거형
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum NodeStatus {
    Idle,
    Processing,
    Blocked,
    Failed,
}

/// 공정 에지 (노드 간 연결)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Edge {
    pub id: String,
    pub source: String,
    pub target: String,
    #[serde(rename = "type")]
    pub edge_type: EdgeType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<EdgeData>,
}

/// 에지 타입
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum EdgeType {
    MaterialFlow,
    DataFlow,
}

/// 에지 데이터
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EdgeData {
    #[serde(default)]
    pub latency: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bandwidth: Option<f64>,
}

/// 시뮬레이션 설정
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimulationConfig {
    pub duration_seconds: i32,
    #[serde(default = "default_time_step")]
    pub time_step: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub random_seed: Option<i32>,
}

fn default_time_step() -> f64 {
    1.0
}
