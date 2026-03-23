import { useState, useCallback, useMemo } from 'react';
import ReactFlow, { 
  addEdge, 
  Background, 
  Controls, 
  applyEdgeChanges, 
  applyNodeChanges,
  Panel,
  type Node,
  type Edge,
  type NodeChange,
  type EdgeChange,
  type Connection,
  type OnConnect,
} from 'reactflow';
import 'reactflow/dist/style.css';
import { Activity, Play, Database, Share2, Layers } from 'lucide-react';

import { nodeTypes } from './components/CustomNodes';
import { Sidebar } from './components/Sidebar';
import { useSocket } from './hooks/useSocket';

const initialNodes: Node[] = [
  {
    id: 'node-1',
    type: 'assembler',
    data: { 
      label: 'Main Assembler',
      parameters: { cycle_time: 12.5, processing_capacity: 100 }
    },
    position: { x: 250, y: 100 },
  },
  {
    id: 'node-2',
    type: 'inspector',
    data: { 
      label: 'Quality Check',
      parameters: { failure_rate: 0.05 }
    },
    position: { x: 250, y: 300 },
  },
];

const initialEdges: Edge[] = [
  { id: 'e1-2', source: 'node-1', target: 'node-2', animated: true },
];

function App() {
  const [nodes, setNodes] = useState<Node[]>(initialNodes);
  const [edges, setEdges] = useState<Edge[]>(initialEdges);
  const [selectedNodeId, setSelectedNodeId] = useState<string | null>(null);

  const { isConnected, sendMessage } = useSocket('ws://localhost:8080');

  const onNodesChange = useCallback(
    (changes: NodeChange[]) => setNodes((nds) => applyNodeChanges(changes, nds)),
    []
  );
  const onEdgesChange = useCallback(
    (changes: EdgeChange[]) => setEdges((eds) => applyEdgeChanges(changes, eds)),
    []
  );
  const onConnect: OnConnect = useCallback(
    (params: Connection) => setEdges((eds) => addEdge(params, eds)),
    []
  );

  const onNodeClick = useCallback((_: React.MouseEvent, node: Node) => {
    setSelectedNodeId(node.id);
  }, []);

  const onPaneClick = useCallback(() => {
    setSelectedNodeId(null);
  }, []);

  const updateNodeParameter = useCallback((nodeId: string, param: string, value: number) => {
    setNodes((nds) =>
      nds.map((node) => {
        if (node.id === nodeId) {
          return {
            ...node,
            data: {
              ...node.data,
              parameters: {
                ...node.data.parameters,
                [param]: value,
              },
            },
          };
        }
        return node;
      })
    );

    // 백엔드에 매개변수 업데이트 전송
    sendMessage({
      event: 'UpdateParameter',
      payload: {
        node_id: nodeId,
        param_name: param,
        new_value: value,
      },
    });
  }, [sendMessage]);

  const selectedNode = useMemo(() => 
    nodes.find(n => n.id === selectedNodeId) || null, 
  [nodes, selectedNodeId]);

  const runSimulation = () => {
    if (!isConnected) return;
    sendMessage({
      event: 'RunSimulation',
      payload: {
        request_id: `sim-${Date.now()}`,
        parameter_changes: [] // 현재 상태 기반
      }
    });
  };

  return (
    <div style={{ width: '100vw', height: '100vh', display: 'flex', flexDirection: 'column' }}>
      <header className="glass-panel" style={{ 
        height: '64px', 
        margin: '12px', 
        display: 'flex', 
        alignItems: 'center', 
        justifyContent: 'space-between',
        padding: '0 24px',
        zIndex: 10
      }}>
        <div style={{ display: 'flex', alignItems: 'center', gap: '12px' }}>
          <Activity size={24} color={isConnected ? "#3b82f6" : "#f87171"} className={isConnected ? "status-active" : ""} />
          <h1 style={{ fontSize: '1.25rem', fontWeight: 600, letterSpacing: '-0.025em' }}>
            NEXUS-Flow <span style={{ color: 'var(--text-secondary)', fontWeight: 400 }}>| Simulation Studio</span>
          </h1>
        </div>
        
        <div style={{ display: 'flex', gap: '12px' }}>
          <button 
            onClick={runSimulation}
            className="glass-card" 
            style={{ 
              padding: '8px 16px', 
              display: 'flex', 
              alignItems: 'center', 
              gap: '8px', 
              cursor: 'pointer', 
              background: 'var(--accent-blue)', 
              border: 'none',
              opacity: isConnected ? 1 : 0.5
            }}
          >
            <Play size={16} fill="white" /> Run Simulation
          </button>
        </div>
      </header>

      <div style={{ flex: 1, display: 'flex', overflow: 'hidden' }}>
        <main style={{ flex: 1, position: 'relative' }}>
          <ReactFlow
            nodes={nodes}
            edges={edges}
            onNodesChange={onNodesChange}
            onEdgesChange={onEdgesChange}
            onConnect={onConnect}
            onNodeClick={onNodeClick}
            onPaneClick={onPaneClick}
            nodeTypes={nodeTypes}
            fitView
          >
            <Background color="#333" gap={20} />
            <Controls />
            
            <Panel position="bottom-left" style={{ display: 'flex', gap: '12px', padding: '12px' }}>
              <div className="glass-card" style={{ padding: '10px' }}><Database size={20} /></div>
              <div className="glass-card" style={{ padding: '10px' }}><Share2 size={20} /></div>
              <div className="glass-card" style={{ padding: '10px' }}><Layers size={20} /></div>
            </Panel>
          </ReactFlow>
        </main>

        <Sidebar 
          selectedNode={selectedNode} 
          onUpdate={updateNodeParameter} 
        />
      </div>
    </div>
  );
}

export default App;
