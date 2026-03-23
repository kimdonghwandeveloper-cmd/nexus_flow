import React, { useState, useEffect } from 'react';
import { Settings, Save, RefreshCw, ChevronRight } from 'lucide-react';

interface SidebarProps {
  selectedNode: any;
  onUpdate: (nodeId: string, param: string, value: number) => void;
}

export const Sidebar: React.FC<SidebarProps> = ({ selectedNode, onUpdate }) => {
  if (!selectedNode) {
    return (
      <aside className="glass-panel" style={{ width: '320px', margin: '12px', padding: '24px', display: 'flex', flexDirection: 'column', alignItems: 'center', justifyContent: 'center', color: 'var(--text-secondary)' }}>
        <Settings size={48} style={{ marginBottom: '16px', opacity: 0.5 }} />
        <p>Select a node to edit parameters</p>
      </aside>
    );
  }

  const params = selectedNode.data.parameters || {};

  return (
    <aside className="glass-panel" style={{ width: '320px', margin: '12px', padding: '24px', display: 'flex', flexDirection: 'column', gap: '20px' }}>
      <div style={{ display: 'flex', alignItems: 'center', gap: '8px', borderBottom: '1px solid var(--border-color)', paddingBottom: '12px' }}>
        <Settings size={20} color="var(--accent-blue)" />
        <h2 style={{ fontSize: '1rem', fontWeight: 600 }}>Node Settings</h2>
      </div>

      <div>
        <label style={{ display: 'block', fontSize: '0.75rem', color: 'var(--text-secondary)', marginBottom: '4px' }}>Node ID</label>
        <div style={{ fontSize: '0.875rem', fontWeight: 500 }}>{selectedNode.id}</div>
      </div>

      <div style={{ display: 'flex', flexDirection: 'column', gap: '16px' }}>
        {Object.entries(params).map(([key, value]) => (
          <div key={key}>
            <label style={{ display: 'block', fontSize: '0.75rem', color: 'var(--text-secondary)', marginBottom: '6px' }}>
              {key.split('_').map(word => word.charAt(0).toUpperCase() + word.slice(1)).join(' ')}
            </label>
            <input 
              type="number" 
              value={value as number}
              onChange={(e) => onUpdate(selectedNode.id, key, parseFloat(e.target.value))}
              style={{
                width: '100%',
                background: 'rgba(255, 255, 255, 0.05)',
                border: '1px solid var(--border-color)',
                borderRadius: '8px',
                padding: '8px 12px',
                color: '#fff',
                fontSize: '0.875rem',
                outline: 'none'
              }}
            />
          </div>
        ))}
      </div>

      <div style={{ marginTop: 'auto', display: 'flex', flexDirection: 'column', gap: '8px' }}>
        <button className="glass-card" style={{ padding: '10px', display: 'flex', alignItems: 'center', justifyContent: 'center', gap: '8px', cursor: 'pointer', background: 'rgba(59, 130, 246, 0.1)', border: 'none', color: 'var(--accent-blue)' }}>
          <Save size={16} /> Save Changes
        </button>
        <button className="glass-card" style={{ padding: '10px', display: 'flex', alignItems: 'center', justifyContent: 'center', gap: '8px', cursor: 'pointer', background: 'transparent', border: 'none', color: 'var(--text-secondary)' }}>
          <RefreshCw size={16} /> Reset Default
        </button>
      </div>
    </aside>
  );
};
