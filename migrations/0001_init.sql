-- migrations/0001_init.sql
-- Schéma initial. Cf. doc d'archi §4.

CREATE EXTENSION IF NOT EXISTS pgcrypto;
CREATE EXTENSION IF NOT EXISTS vector;
CREATE EXTENSION IF NOT EXISTS pg_trgm;

-- ─────────────────────────────────────────────────────────────────────────
-- OFFRES
-- ─────────────────────────────────────────────────────────────────────────
CREATE TABLE offres (
    id              UUID        PRIMARY KEY DEFAULT gen_random_uuid(),
    slug            TEXT        NOT NULL UNIQUE,
    source_url      TEXT        NOT NULL,
    source_host     TEXT        NOT NULL,
    source_hash     BYTEA       NOT NULL,
    entreprise      TEXT        NOT NULL,
    intitule        TEXT        NOT NULL,
    localisation    TEXT,
    contrat         TEXT,
    raw_html        TEXT,
    raw_text        TEXT        NOT NULL,
    structured      JSONB       NOT NULL,
    embedding       vector(1024),
    scraped_at      TIMESTAMPTZ NOT NULL DEFAULT now(),
    last_seen_at    TIMESTAMPTZ NOT NULL DEFAULT now(),
    closed_at       TIMESTAMPTZ,
    UNIQUE (source_host, source_hash)
);

CREATE INDEX offres_entreprise_trgm ON offres USING gin (entreprise gin_trgm_ops);
CREATE INDEX offres_intitule_trgm   ON offres USING gin (intitule gin_trgm_ops);
CREATE INDEX offres_structured_gin  ON offres USING gin (structured jsonb_path_ops);
CREATE INDEX offres_embedding_hnsw  ON offres
    USING hnsw (embedding vector_cosine_ops)
    WITH (m = 16, ef_construction = 64);

-- ─────────────────────────────────────────────────────────────────────────
-- PROFILS
-- ─────────────────────────────────────────────────────────────────────────
CREATE TABLE profils (
    id              UUID        PRIMARY KEY DEFAULT gen_random_uuid(),
    label           TEXT        NOT NULL,
    content         JSONB       NOT NULL,
    is_active       BOOLEAN     NOT NULL DEFAULT false,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT now()
);
CREATE UNIQUE INDEX profils_one_active ON profils (is_active) WHERE is_active = true;

-- ─────────────────────────────────────────────────────────────────────────
-- CHUNKS
-- ─────────────────────────────────────────────────────────────────────────
CREATE TYPE chunk_kind AS ENUM (
    'experience', 'projet', 'formation', 'competence', 'phrase_lettre'
);

CREATE TABLE chunks (
    id              UUID        PRIMARY KEY DEFAULT gen_random_uuid(),
    profil_id       UUID        NOT NULL REFERENCES profils(id) ON DELETE CASCADE,
    kind            chunk_kind  NOT NULL,
    titre           TEXT        NOT NULL,
    content         TEXT        NOT NULL,
    metadata        JSONB       NOT NULL DEFAULT '{}'::jsonb,
    embedding       vector(1024) NOT NULL,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT now()
);
CREATE INDEX chunks_profil    ON chunks(profil_id);
CREATE INDEX chunks_kind      ON chunks(kind);
CREATE INDEX chunks_embedding ON chunks USING hnsw (embedding vector_cosine_ops);

-- ─────────────────────────────────────────────────────────────────────────
-- INSTANCES
-- ─────────────────────────────────────────────────────────────────────────
CREATE TYPE instance_status AS ENUM (
    'draft', 'generating', 'ready', 'sent', 'archived', 'failed'
);

CREATE TABLE instances (
    id                UUID        PRIMARY KEY DEFAULT gen_random_uuid(),
    slug              TEXT        NOT NULL UNIQUE,
    offre_id          UUID        NOT NULL REFERENCES offres(id) ON DELETE RESTRICT,
    profil_id         UUID        NOT NULL REFERENCES profils(id) ON DELETE RESTRICT,
    status            instance_status NOT NULL DEFAULT 'draft',
    resume_json       JSONB,
    cover_letter_json JSONB,
    notes             JSONB       NOT NULL DEFAULT '{}'::jsonb,
    created_at        TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at        TIMESTAMPTZ NOT NULL DEFAULT now(),
    sent_at           TIMESTAMPTZ
);
CREATE INDEX instances_offre  ON instances(offre_id);
CREATE INDEX instances_status ON instances(status);

-- ─────────────────────────────────────────────────────────────────────────
-- LLM_CALLS
-- ─────────────────────────────────────────────────────────────────────────
CREATE TABLE llm_calls (
    id              UUID        PRIMARY KEY DEFAULT gen_random_uuid(),
    instance_id     UUID        REFERENCES instances(id) ON DELETE CASCADE,
    purpose         TEXT        NOT NULL,
    provider        TEXT        NOT NULL,
    model           TEXT        NOT NULL,
    prompt_hash     BYTEA       NOT NULL,
    prompt          TEXT        NOT NULL,
    response        TEXT,
    tokens_in       INTEGER,
    tokens_out      INTEGER,
    cost_usd        NUMERIC(10,6),
    latency_ms      INTEGER,
    error           TEXT,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT now()
);
CREATE INDEX llm_calls_instance ON llm_calls(instance_id);
CREATE INDEX llm_calls_hash     ON llm_calls(prompt_hash);

-- ─────────────────────────────────────────────────────────────────────────
-- VUE COÛTS — utile dès le jour 1 pour suivre les dépenses LLM
-- ─────────────────────────────────────────────────────────────────────────
CREATE VIEW v_llm_costs_daily AS
SELECT
    date_trunc('day', created_at) AS jour,
    provider,
    model,
    purpose,
    COUNT(*)              AS nb_calls,
    SUM(tokens_in)        AS tokens_in,
    SUM(tokens_out)       AS tokens_out,
    SUM(cost_usd)         AS cost_usd,
    AVG(latency_ms)::INT  AS avg_latency_ms
FROM llm_calls
GROUP BY 1, 2, 3, 4
ORDER BY 1 DESC;
