-- ============================================================
--  NEXUS-Flow Supabase Database Schema
--  Initial Migration: Projects, Topologies, Simulation Results
-- ============================================================

-- Enable UUID generation
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- ---------------------------------------------------------
--  1. projects: 시뮬레이션 프로젝트 메타데이터
-- ---------------------------------------------------------
CREATE TABLE projects (
    id          UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name        TEXT NOT NULL,
    description TEXT DEFAULT '',
    created_at  TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at  TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    created_by  UUID REFERENCES auth.users(id) ON DELETE SET NULL
);

-- 프로젝트 수정 시 updated_at 자동 갱신 트리거
CREATE OR REPLACE FUNCTION update_modified_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER trg_projects_updated_at
    BEFORE UPDATE ON projects
    FOR EACH ROW
    EXECUTE FUNCTION update_modified_column();

-- ---------------------------------------------------------
--  2. topologies: JSONB 기반 토폴로지 데이터 (버전 관리)
-- ---------------------------------------------------------
CREATE TABLE topologies (
    id          UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    project_id  UUID NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
    version     INT NOT NULL DEFAULT 1,
    data        JSONB NOT NULL,               -- SSOT JSON 전체 (nodes, edges, simulation_config)
    snapshot_label TEXT DEFAULT '',            -- 사용자 지정 스냅샷 이름 (예: "baseline", "what-if-01")
    is_active   BOOLEAN NOT NULL DEFAULT TRUE, -- 현재 활성 토폴로지 여부
    created_at  TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    -- 한 프로젝트 내 같은 버전 중복 방지
    UNIQUE (project_id, version)
);

-- JSONB 검색 성능을 위한 GIN 인덱스
CREATE INDEX idx_topologies_data ON topologies USING GIN (data);
CREATE INDEX idx_topologies_project_active ON topologies (project_id, is_active);

-- ---------------------------------------------------------
--  3. simulation_results: What-If 시뮬레이션 결과 캐싱
-- ---------------------------------------------------------
CREATE TABLE simulation_results (
    id              UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    topology_id     UUID NOT NULL REFERENCES topologies(id) ON DELETE CASCADE,
    request_id      TEXT NOT NULL,              -- gRPC SimulationRequest.request_id 매칭
    parameter_changes JSONB DEFAULT '[]',       -- 변경된 파라미터 목록 스냅샷
    node_results    JSONB NOT NULL DEFAULT '[]', -- NodeResult 배열
    impact_chain    JSONB NOT NULL DEFAULT '[]', -- ImpactChainLink 배열
    overall_throughput DOUBLE PRECISION,         -- 전체 예측 처리량
    overall_efficiency DOUBLE PRECISION,         -- 전체 예측 효율 (%)
    success         BOOLEAN NOT NULL DEFAULT TRUE,
    error_message   TEXT DEFAULT '',
    executed_at     TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_sim_results_topology ON simulation_results (topology_id);
CREATE INDEX idx_sim_results_request  ON simulation_results (request_id);

-- ---------------------------------------------------------
--  4. RLS (Row Level Security) 정책
-- ---------------------------------------------------------

-- projects 테이블 RLS
ALTER TABLE projects ENABLE ROW LEVEL SECURITY;

CREATE POLICY "Users can view own projects"
    ON projects FOR SELECT
    USING (auth.uid() = created_by);

CREATE POLICY "Users can create projects"
    ON projects FOR INSERT
    WITH CHECK (auth.uid() = created_by);

CREATE POLICY "Users can update own projects"
    ON projects FOR UPDATE
    USING (auth.uid() = created_by);

CREATE POLICY "Users can delete own projects"
    ON projects FOR DELETE
    USING (auth.uid() = created_by);

-- topologies 테이블 RLS (프로젝트 소유자만 접근)
ALTER TABLE topologies ENABLE ROW LEVEL SECURITY;

CREATE POLICY "Users can access topologies of own projects"
    ON topologies FOR ALL
    USING (
        EXISTS (
            SELECT 1 FROM projects
            WHERE projects.id = topologies.project_id
            AND projects.created_by = auth.uid()
        )
    );

-- simulation_results 테이블 RLS (프로젝트 소유자만 접근)
ALTER TABLE simulation_results ENABLE ROW LEVEL SECURITY;

CREATE POLICY "Users can access simulation results of own topologies"
    ON simulation_results FOR ALL
    USING (
        EXISTS (
            SELECT 1 FROM topologies
            JOIN projects ON projects.id = topologies.project_id
            WHERE topologies.id = simulation_results.topology_id
            AND projects.created_by = auth.uid()
        )
    );
