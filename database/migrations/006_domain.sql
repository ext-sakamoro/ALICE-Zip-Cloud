-- ALICE Zip Cloud: Domain-specific tables
CREATE TABLE IF NOT EXISTS compression_jobs (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES auth.users(id),
    algorithm TEXT NOT NULL DEFAULT 'hybrid-auto' CHECK (algorithm IN ('procedural', 'lzma', 'hybrid-auto', 'terrain', 'texture', 'log-structured')),
    input_bytes BIGINT NOT NULL DEFAULT 0,
    output_bytes BIGINT NOT NULL DEFAULT 0,
    compression_ratio DOUBLE PRECISION NOT NULL DEFAULT 1.0,
    pattern_type TEXT CHECK (pattern_type IN ('terrain', 'texture', 'log', 'generic', 'procedural')),
    elapsed_ms BIGINT NOT NULL DEFAULT 0,
    status TEXT NOT NULL DEFAULT 'completed' CHECK (status IN ('queued', 'compressing', 'completed', 'failed')),
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TABLE IF NOT EXISTS pattern_analyses (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    job_id UUID NOT NULL REFERENCES compression_jobs(id) ON DELETE CASCADE,
    pattern_type TEXT NOT NULL CHECK (pattern_type IN ('terrain', 'texture', 'log', 'generic', 'procedural')),
    confidence DOUBLE PRECISION NOT NULL DEFAULT 0.0,
    estimated_ratio DOUBLE PRECISION NOT NULL DEFAULT 1.0,
    seed_parameters JSONB NOT NULL DEFAULT '{}',
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TABLE IF NOT EXISTS batch_jobs (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES auth.users(id),
    total_files INTEGER NOT NULL DEFAULT 0,
    completed_files INTEGER NOT NULL DEFAULT 0,
    total_input_bytes BIGINT NOT NULL DEFAULT 0,
    total_output_bytes BIGINT NOT NULL DEFAULT 0,
    avg_ratio DOUBLE PRECISION NOT NULL DEFAULT 1.0,
    status TEXT NOT NULL DEFAULT 'running' CHECK (status IN ('running', 'completed', 'failed', 'cancelled')),
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX idx_compression_jobs_user ON compression_jobs(user_id, created_at);
CREATE INDEX idx_pattern_analyses_job ON pattern_analyses(job_id);
CREATE INDEX idx_batch_jobs_user ON batch_jobs(user_id, created_at);
