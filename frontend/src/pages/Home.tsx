import { useState, useEffect } from 'react';
import { useNavigate } from 'react-router-dom';
import { Home as HomeIcon, Folder, Heart, Settings, Plus, Loader2 } from 'lucide-react';
import { supabase } from '../lib/supabase';

// Mock data fallback
const mockProjects = [
  { id: 'p1', title: '워크스페이스 디자인', img: 'https://images.unsplash.com/photo-1593640408182-31c70c8268f5?auto=format&fit=crop&w=400&q=80' },
  { id: 'p2', title: '크리에이티브 스튜디오', img: 'https://images.unsplash.com/photo-1497215728101-856f4ea42174?auto=format&fit=crop&w=400&q=80' },
  { id: 'p3', title: '미니멀 오피스', img: 'https://images.unsplash.com/photo-1497366216548-37526070297c?auto=format&fit=crop&w=400&q=80' },
  { id: 'p4', title: '건축 설계', img: 'https://images.unsplash.com/photo-1503387762-592deb58ef4e?auto=format&fit=crop&w=400&q=80' },
  { id: 'p5', title: '웹 디자인 목업', img: 'https://images.unsplash.com/photo-1498050108023-c5249f4df085?auto=format&fit=crop&w=400&q=80' },
  { id: 'p6', title: '기하학적 패턴', img: 'https://images.unsplash.com/photo-1550684848-fac1c5b4e853?auto=format&fit=crop&w=400&q=80' }
];

export function Home() {
  const navigate = useNavigate();
  const [projects, setProjects] = useState<any[]>([]);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    fetchProjects();
  }, []);

  const fetchProjects = async () => {
    try {
      const { data, error } = await supabase.from('projects').select('*').order('created_at', { ascending: false });
      
      if (error || !data || data.length === 0) {
        console.warn('Failed to fetch from Supabase (or no data), using mock data.', error?.message);
        setProjects(mockProjects);
      } else {
        // Map DB models to component models, providing random mockup images based on index
        const mapped = data.map((p: any, i: number) => ({
          id: p.id,
          title: p.name,
          img: mockProjects[i % mockProjects.length].img
        }));
        setProjects(mapped);
      }
    } catch (e) {
      console.error('Exception fetching projects:', e);
      setProjects(mockProjects);
    } finally {
      setLoading(false);
    }
  };

  const handleCreateProject = async () => {
    try {
      // Attempt to inserting to Supabase. RLS might block this without auth.
      const { data, error } = await supabase
        .from('projects')
        .insert({ name: '새 프로젝트' })
        .select()
        .single();
        
      if (!error && data) {
        navigate(`/project/${data.id}`);
      } else {
        console.warn('Could not create project in Supabase (probably RLS or Auth), redirecting to new-local');
        navigate('/project/new-local');
      }
    } catch (e) {
      navigate('/project/new-local');
    }
  };

  return (
    <div style={{ display: 'flex', width: '100vw', height: '100vh', backgroundColor: '#ffffff', color: '#111827' }}>
      {/* Left Sidebar Menu */}
      <div style={{ 
        width: '64px', 
        backgroundColor: '#000000', 
        display: 'flex', 
        flexDirection: 'column', 
        alignItems: 'center', 
        padding: '32px 0',
        zIndex: 50
      }}>
        <div style={{ display: 'flex', flexDirection: 'column', gap: '36px', flex: 1 }}>
          <div style={{ 
            padding: '12px', 
            borderRadius: '12px', 
            cursor: 'pointer',
            border: '1px solid rgba(255,255,255,0.2)' 
          }}>
            <HomeIcon size={24} color="#ffffff" />
          </div>
          
          <div style={{ padding: '8px', cursor: 'pointer', transition: 'opacity 0.2s', opacity: 0.8 }}>
            <Folder size={24} color="#ffffff" />
          </div>
          
          <div style={{ padding: '8px', cursor: 'pointer', transition: 'opacity 0.2s', opacity: 0.8 }}>
            <Heart size={24} color="#ffffff" />
          </div>
        </div>
        
        <div style={{ padding: '8px', cursor: 'pointer', transition: 'opacity 0.2s', opacity: 0.8 }}>
          <Settings size={24} color="#ffffff" />
        </div>
      </div>

      {/* Main Content Area */}
      <div style={{ flex: 1, padding: '56px 72px', overflowY: 'auto' }}>
        <h1 style={{ 
          fontSize: '28px', 
          fontWeight: 700, 
          marginBottom: '40px', 
          color: '#111827',
          letterSpacing: '-0.02em',
          display: 'flex',
          alignItems: 'center',
          gap: '12px'
        }}>
          내 프로젝트
          {loading && <Loader2 size={24} color="#a1a1aa" style={{ animation: 'spin 1.5s linear infinite' }} />}
        </h1>

        <div style={{ 
          display: 'grid', 
          gridTemplateColumns: 'repeat(auto-fill, minmax(280px, 1fr))', 
          gap: '32px 24px' 
        }}>
          {/* Create New Project Card */}
          <div 
            onClick={handleCreateProject}
            style={{ display: 'flex', flexDirection: 'column', gap: '12px', cursor: 'pointer' }}
          >
            <div style={{ 
              backgroundColor: '#0a0a0c', 
              borderRadius: '12px', 
              aspectRatio: '1.4 / 1',
              display: 'flex', 
              flexDirection: 'column',
              justifyContent: 'center', 
              alignItems: 'center', 
              boxShadow: '0 4px 12px rgba(0,0,0,0.05)',
              transition: 'transform 0.2s, box-shadow 0.2s'
            }}
            onMouseOver={(e) => {
              e.currentTarget.style.transform = 'translateY(-4px)';
              e.currentTarget.style.boxShadow = '0 12px 24px rgba(0,0,0,0.1)';
            }}
            onMouseOut={(e) => {
              e.currentTarget.style.transform = 'none';
              e.currentTarget.style.boxShadow = '0 4px 12px rgba(0,0,0,0.05)';
            }}
            >
              <Plus size={40} color="#ffffff" />
            </div>
            <span style={{ fontSize: '15px', fontWeight: 500, color: '#374151' }}>프로젝트 추가하기</span>
          </div>

          {/* Project Cards */}
          {projects.map(proj => (
            <div 
              key={proj.id}
              onClick={() => navigate(`/project/${proj.id}`)}
              style={{ display: 'flex', flexDirection: 'column', gap: '12px', cursor: 'pointer' }}
            >
              <div style={{ 
                borderRadius: '12px', 
                overflow: 'hidden', 
                aspectRatio: '1.4 / 1',
                backgroundColor: '#f3f4f6',
                boxShadow: '0 4px 12px rgba(0,0,0,0.05)',
                transition: 'transform 0.2s, box-shadow 0.2s'
              }}
              onMouseOver={(e) => {
                e.currentTarget.style.transform = 'translateY(-4px)';
                e.currentTarget.style.boxShadow = '0 12px 24px rgba(0,0,0,0.1)';
              }}
              onMouseOut={(e) => {
                e.currentTarget.style.transform = 'none';
                e.currentTarget.style.boxShadow = '0 4px 12px rgba(0,0,0,0.05)';
              }}
              >
                <img 
                  src={proj.img} 
                  alt={proj.title} 
                  style={{ width: '100%', height: '100%', objectFit: 'cover' }} 
                />
              </div>
              <span style={{ fontSize: '15px', fontWeight: 500, color: '#374151' }}>{proj.title}</span>
            </div>
          ))}
        </div>
      </div>
    </div>
  );
}
