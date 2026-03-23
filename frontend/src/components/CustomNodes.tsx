import { memo } from 'react';
import { Handle, Position, type NodeProps } from 'reactflow';
import { CheckCircle2, Box } from 'lucide-react';

export const AssemblerNode = memo(({ data }: NodeProps) => {
  return (
    <div className="glass-card status-active" style={{ padding: '12px', minWidth: '180px' }}>
      <Handle type="target" position={Position.Top} />
      <div style={{ display: 'flex', alignItems: 'center', gap: '8px', marginBottom: '8px' }}>
        <Box size={16} color="var(--accent-blue)" />
        <span style={{ fontWeight: 600, fontSize: '0.875rem' }}>{data.label}</span>
      </div>
      <div style={{ fontSize: '0.75rem', color: 'var(--text-secondary)' }}>
        <div style={{ display: 'flex', justifyContent: 'space-between' }}>
          <span>Cycle Time:</span>
          <span style={{ color: '#fff' }}>{data.parameters?.cycle_time}s</span>
        </div>
        <div style={{ display: 'flex', justifyContent: 'space-between' }}>
          <span>Load:</span>
          <span style={{ color: 'var(--accent-cyan)' }}>82%</span>
        </div>
      </div>
      <Handle type="source" position={Position.Bottom} />
    </div>
  );
});

export const InspectorNode = memo(({ data }: NodeProps) => {
  return (
    <div className="glass-card" style={{ padding: '12px', minWidth: '180px', borderLeft: '4px solid var(--accent-purple)' }}>
      <Handle type="target" position={Position.Top} />
      <div style={{ display: 'flex', alignItems: 'center', gap: '8px', marginBottom: '8px' }}>
        <CheckCircle2 size={16} color="var(--accent-purple)" />
        <span style={{ fontWeight: 600, fontSize: '0.875rem' }}>{data.label}</span>
      </div>
      <div style={{ fontSize: '0.75rem', color: 'var(--text-secondary)' }}>
        <div style={{ display: 'flex', justifyContent: 'space-between' }}>
          <span>Failure Rate:</span>
          <span style={{ color: '#f87171' }}>{(data.parameters?.failure_rate * 100).toFixed(1)}%</span>
        </div>
      </div>
      <Handle type="source" position={Position.Bottom} />
    </div>
  );
});

export const nodeTypes = {
  assembler: AssemblerNode,
  inspector: InspectorNode,
};
