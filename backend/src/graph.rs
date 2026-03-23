use std::collections::{HashMap, HashSet, VecDeque};

use crate::models::{Edge, Node, Topology};

// ============================================================
//  그래프 엔진: BFS/DFS 순회 및 의존성 체인 분석
//  SSOT Topology를 인접 리스트(Adjacency List)로 변환하여 분석
// ============================================================

/// 인접 리스트 기반 방향 그래프
#[derive(Debug, Clone)]
pub struct ProcessGraph {
    /// node_id → Node 참조용 인덱스
    node_map: HashMap<String, usize>,
    /// 인덱스 → Node 데이터
    nodes: Vec<Node>,
    /// 정방향 인접 리스트: node_index → [(target_index, edge)]
    adj_forward: Vec<Vec<(usize, Edge)>>,
    /// 역방향 인접 리스트: node_index → [(source_index, edge)]
    adj_backward: Vec<Vec<(usize, Edge)>>,
}

/// BFS/DFS 순회 결과
#[derive(Debug, Clone)]
pub struct TraversalResult {
    /// 방문 순서대로 정렬된 node_id 목록
    pub visited_order: Vec<String>,
    /// 방문된 노드 수
    pub visit_count: usize,
}

/// 의존성 체인 분석 결과
#[derive(Debug, Clone)]
pub struct DependencyChain {
    /// 특정 노드에 영향을 주는 상류(upstream) 노드 ID 목록
    pub upstream: Vec<String>,
    /// 특정 노드가 영향을 미치는 하류(downstream) 노드 ID 목록
    pub downstream: Vec<String>,
}

/// 병목 분석 결과
#[derive(Debug, Clone)]
pub struct BottleneckAnalysis {
    pub node_id: String,
    pub label: String,
    /// 병목 점수 (0.0 ~ 1.0): 하류 의존 노드 수 기반
    pub bottleneck_score: f64,
    /// 해당 노드에 의존하는 하류 노드 수
    pub downstream_count: usize,
}

/// 토폴로지 정렬 결과 (위상 정렬)
#[derive(Debug, Clone)]
pub struct TopologicalOrder {
    /// 위상 정렬된 node_id 목록 (순환이 없을 때)
    pub order: Vec<String>,
    /// 순환(Cycle) 포함 여부
    pub has_cycle: bool,
}

impl ProcessGraph {
    /// Topology JSON 구조에서 그래프를 생성합니다.
    pub fn from_topology(topology: &Topology) -> Self {
        let mut node_map = HashMap::new();
        let mut nodes = Vec::new();

        for (idx, node) in topology.nodes.iter().enumerate() {
            node_map.insert(node.id.clone(), idx);
            nodes.push(node.clone());
        }

        let n = nodes.len();
        let mut adj_forward = vec![Vec::new(); n];
        let mut adj_backward = vec![Vec::new(); n];

        for edge in &topology.edges {
            if let (Some(&src_idx), Some(&tgt_idx)) =
                (node_map.get(&edge.source), node_map.get(&edge.target))
            {
                adj_forward[src_idx].push((tgt_idx, edge.clone()));
                adj_backward[tgt_idx].push((src_idx, edge.clone()));
            }
        }

        Self {
            node_map,
            nodes,
            adj_forward,
            adj_backward,
        }
    }

    /// 노드 수 반환
    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }

    /// 에지 수 반환
    pub fn edge_count(&self) -> usize {
        self.adj_forward.iter().map(|v| v.len()).sum()
    }

    // =====================================================
    //  BFS (Breadth-First Search) — 레벨 단위 순회
    // =====================================================

    /// 특정 노드에서 시작하는 정방향 BFS 순회
    pub fn bfs_forward(&self, start_node_id: &str) -> Option<TraversalResult> {
        let &start_idx = self.node_map.get(start_node_id)?;
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        let mut order = Vec::new();

        visited.insert(start_idx);
        queue.push_back(start_idx);

        while let Some(current) = queue.pop_front() {
            order.push(self.nodes[current].id.clone());

            for &(next_idx, _) in &self.adj_forward[current] {
                if visited.insert(next_idx) {
                    queue.push_back(next_idx);
                }
            }
        }

        Some(TraversalResult {
            visit_count: order.len(),
            visited_order: order,
        })
    }

    /// 특정 노드에서 시작하는 역방향 BFS (상류 탐색)
    pub fn bfs_backward(&self, start_node_id: &str) -> Option<TraversalResult> {
        let &start_idx = self.node_map.get(start_node_id)?;
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        let mut order = Vec::new();

        visited.insert(start_idx);
        queue.push_back(start_idx);

        while let Some(current) = queue.pop_front() {
            order.push(self.nodes[current].id.clone());

            for &(prev_idx, _) in &self.adj_backward[current] {
                if visited.insert(prev_idx) {
                    queue.push_back(prev_idx);
                }
            }
        }

        Some(TraversalResult {
            visit_count: order.len(),
            visited_order: order,
        })
    }

    // =====================================================
    //  DFS (Depth-First Search) — 깊이 우선 순회
    // =====================================================

    /// 특정 노드에서 시작하는 정방향 DFS 순회
    pub fn dfs_forward(&self, start_node_id: &str) -> Option<TraversalResult> {
        let &start_idx = self.node_map.get(start_node_id)?;
        let mut visited = HashSet::new();
        let mut order = Vec::new();

        self.dfs_recursive(start_idx, &self.adj_forward, &mut visited, &mut order);

        Some(TraversalResult {
            visit_count: order.len(),
            visited_order: order,
        })
    }

    /// 특정 노드에서 시작하는 역방향 DFS (상류 깊이 탐색)
    pub fn dfs_backward(&self, start_node_id: &str) -> Option<TraversalResult> {
        let &start_idx = self.node_map.get(start_node_id)?;
        let mut visited = HashSet::new();
        let mut order = Vec::new();

        self.dfs_recursive(start_idx, &self.adj_backward, &mut visited, &mut order);

        Some(TraversalResult {
            visit_count: order.len(),
            visited_order: order,
        })
    }

    /// DFS 재귀 헬퍼
    fn dfs_recursive(
        &self,
        current: usize,
        adj: &[Vec<(usize, Edge)>],
        visited: &mut HashSet<usize>,
        order: &mut Vec<String>,
    ) {
        visited.insert(current);
        order.push(self.nodes[current].id.clone());

        for &(next_idx, _) in &adj[current] {
            if !visited.contains(&next_idx) {
                self.dfs_recursive(next_idx, adj, visited, order);
            }
        }
    }

    // =====================================================
    //  의존성 체인 분석
    // =====================================================

    /// 특정 노드의 상류/하류 의존성 체인을 분석합니다.
    pub fn analyze_dependency_chain(&self, node_id: &str) -> Option<DependencyChain> {
        // 상류: 역방향 BFS (자기 자신 제외)
        let upstream_result = self.bfs_backward(node_id)?;
        let upstream: Vec<String> = upstream_result
            .visited_order
            .into_iter()
            .filter(|id| id != node_id)
            .collect();

        // 하류: 정방향 BFS (자기 자신 제외)
        let downstream_result = self.bfs_forward(node_id)?;
        let downstream: Vec<String> = downstream_result
            .visited_order
            .into_iter()
            .filter(|id| id != node_id)
            .collect();

        Some(DependencyChain {
            upstream,
            downstream,
        })
    }

    // =====================================================
    //  위상 정렬 (Topological Sort) — Kahn's Algorithm
    // =====================================================

    /// DAG(Directed Acyclic Graph) 기준 위상 정렬을 수행합니다.
    /// 순환이 감지되면 `has_cycle = true`를 반환합니다.
    pub fn topological_sort(&self) -> TopologicalOrder {
        let n = self.nodes.len();
        let mut in_degree = vec![0usize; n];

        // 진입 차수 계산
        for edges in &self.adj_forward {
            for &(target, _) in edges {
                in_degree[target] += 1;
            }
        }

        // 진입 차수가 0인 노드를 큐에 삽입
        let mut queue: VecDeque<usize> = VecDeque::new();
        for (idx, &deg) in in_degree.iter().enumerate() {
            if deg == 0 {
                queue.push_back(idx);
            }
        }

        let mut order = Vec::new();

        while let Some(current) = queue.pop_front() {
            order.push(self.nodes[current].id.clone());

            for &(next_idx, _) in &self.adj_forward[current] {
                in_degree[next_idx] -= 1;
                if in_degree[next_idx] == 0 {
                    queue.push_back(next_idx);
                }
            }
        }

        let has_cycle = order.len() != n;
        TopologicalOrder { order, has_cycle }
    }

    // =====================================================
    //  병목 분석
    // =====================================================

    /// 모든 노드에 대한 병목 분석을 수행합니다.
    /// 하류 의존 노드 수가 많을수록 병목 점수가 높습니다.
    pub fn analyze_bottlenecks(&self) -> Vec<BottleneckAnalysis> {
        let total = self.nodes.len();
        if total <= 1 {
            return Vec::new();
        }

        let max_possible = total - 1; // 자기 자신 제외 최대 하류 수

        let mut results: Vec<BottleneckAnalysis> = self
            .nodes
            .iter()
            .map(|node| {
                let downstream_count = self
                    .bfs_forward(&node.id)
                    .map(|r| r.visit_count.saturating_sub(1)) // 자기 자신 제외
                    .unwrap_or(0);

                BottleneckAnalysis {
                    node_id: node.id.clone(),
                    label: node.data.label.clone(),
                    bottleneck_score: downstream_count as f64 / max_possible as f64,
                    downstream_count,
                }
            })
            .collect();

        // 병목 점수 내림차순 정렬
        results.sort_by(|a, b| {
            b.bottleneck_score
                .partial_cmp(&a.bottleneck_score)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        results
    }
}

// ============================================================
//  단위 테스트
// ============================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::*;

    /// 테스트용 간단한 토폴로지 생성
    /// Source → Assembler → Inspector → Sink
    fn create_test_topology() -> Topology {
        Topology {
            version: "1.0.0".to_string(),
            project: Project {
                name: "Test".to_string(),
                description: None,
                created_at: None,
            },
            nodes: vec![
                Node {
                    id: "src".to_string(),
                    node_type: NodeType::Source,
                    position: Position { x: 0.0, y: 0.0 },
                    data: NodeData {
                        label: "Source".to_string(),
                        parameters: NodeParameters {
                            cycle_time: 2.0,
                            failure_rate: 0.0,
                            processing_capacity: 10,
                        },
                        state: None,
                    },
                },
                Node {
                    id: "asm".to_string(),
                    node_type: NodeType::Assembler,
                    position: Position { x: 100.0, y: 0.0 },
                    data: NodeData {
                        label: "Assembler".to_string(),
                        parameters: NodeParameters {
                            cycle_time: 15.0,
                            failure_rate: 0.05,
                            processing_capacity: 1,
                        },
                        state: None,
                    },
                },
                Node {
                    id: "ins".to_string(),
                    node_type: NodeType::Inspector,
                    position: Position { x: 200.0, y: 0.0 },
                    data: NodeData {
                        label: "Inspector".to_string(),
                        parameters: NodeParameters {
                            cycle_time: 5.0,
                            failure_rate: 0.01,
                            processing_capacity: 2,
                        },
                        state: None,
                    },
                },
                Node {
                    id: "snk".to_string(),
                    node_type: NodeType::Sink,
                    position: Position { x: 300.0, y: 0.0 },
                    data: NodeData {
                        label: "Sink".to_string(),
                        parameters: NodeParameters {
                            cycle_time: 0.0,
                            failure_rate: 0.0,
                            processing_capacity: 100,
                        },
                        state: None,
                    },
                },
            ],
            edges: vec![
                Edge {
                    id: "e1".to_string(),
                    source: "src".to_string(),
                    target: "asm".to_string(),
                    edge_type: EdgeType::MaterialFlow,
                    data: None,
                },
                Edge {
                    id: "e2".to_string(),
                    source: "asm".to_string(),
                    target: "ins".to_string(),
                    edge_type: EdgeType::MaterialFlow,
                    data: None,
                },
                Edge {
                    id: "e3".to_string(),
                    source: "ins".to_string(),
                    target: "snk".to_string(),
                    edge_type: EdgeType::MaterialFlow,
                    data: None,
                },
            ],
            simulation_config: None,
        }
    }

    #[test]
    fn test_graph_construction() {
        let topo = create_test_topology();
        let graph = ProcessGraph::from_topology(&topo);
        assert_eq!(graph.node_count(), 4);
        assert_eq!(graph.edge_count(), 3);
    }

    #[test]
    fn test_bfs_forward() {
        let topo = create_test_topology();
        let graph = ProcessGraph::from_topology(&topo);
        let result = graph.bfs_forward("src").unwrap();
        assert_eq!(result.visit_count, 4);
        assert_eq!(result.visited_order, vec!["src", "asm", "ins", "snk"]);
    }

    #[test]
    fn test_bfs_backward() {
        let topo = create_test_topology();
        let graph = ProcessGraph::from_topology(&topo);
        let result = graph.bfs_backward("snk").unwrap();
        assert_eq!(result.visit_count, 4);
        assert_eq!(result.visited_order, vec!["snk", "ins", "asm", "src"]);
    }

    #[test]
    fn test_dfs_forward() {
        let topo = create_test_topology();
        let graph = ProcessGraph::from_topology(&topo);
        let result = graph.dfs_forward("src").unwrap();
        assert_eq!(result.visit_count, 4);
        assert_eq!(result.visited_order, vec!["src", "asm", "ins", "snk"]);
    }

    #[test]
    fn test_dependency_chain() {
        let topo = create_test_topology();
        let graph = ProcessGraph::from_topology(&topo);
        let chain = graph.analyze_dependency_chain("asm").unwrap();
        assert_eq!(chain.upstream, vec!["src"]);
        assert_eq!(chain.downstream, vec!["ins", "snk"]);
    }

    #[test]
    fn test_topological_sort() {
        let topo = create_test_topology();
        let graph = ProcessGraph::from_topology(&topo);
        let result = graph.topological_sort();
        assert!(!result.has_cycle);
        assert_eq!(result.order, vec!["src", "asm", "ins", "snk"]);
    }

    #[test]
    fn test_bottleneck_analysis() {
        let topo = create_test_topology();
        let graph = ProcessGraph::from_topology(&topo);
        let bottlenecks = graph.analyze_bottlenecks();

        // Source 노드가 가장 높은 병목 점수 (하류 3개 — 전체의 100%)
        assert_eq!(bottlenecks[0].node_id, "src");
        assert_eq!(bottlenecks[0].downstream_count, 3);
        assert!((bottlenecks[0].bottleneck_score - 1.0).abs() < f64::EPSILON);

        // Sink 노드는 하류가 없으므로 점수 0
        let sink = bottlenecks.iter().find(|b| b.node_id == "snk").unwrap();
        assert_eq!(sink.downstream_count, 0);
        assert!((sink.bottleneck_score - 0.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_invalid_node_returns_none() {
        let topo = create_test_topology();
        let graph = ProcessGraph::from_topology(&topo);
        assert!(graph.bfs_forward("nonexistent").is_none());
        assert!(graph.analyze_dependency_chain("nonexistent").is_none());
    }
}
