-- init.sql
-- Schéma complet et unifié. Source de vérité unique pour la structure de la base.

CREATE EXTENSION IF NOT EXISTS pgcrypto;
CREATE EXTENSION IF NOT EXISTS vector;
CREATE EXTENSION IF NOT EXISTS pg_trgm;

DROP TABLE IF EXISTS llm_calls CASCADE;
DROP TABLE IF EXISTS messages CASCADE;
DROP TABLE IF EXISTS instances CASCADE;
DROP TABLE IF EXISTS chunks CASCADE;
DROP TABLE IF EXISTS profils CASCADE;
DROP TABLE IF EXISTS offres CASCADE;
DROP TYPE IF EXISTS chunk_kind CASCADE;
DROP TYPE IF EXISTS instance_status CASCADE;
DROP VIEW IF EXISTS v_llm_costs_daily CASCADE;

-- ─────────────────────────────────────────────────────────────────────────
-- OFFRES
-- ─────────────────────────────────────────────────────────────────────────
CREATE TABLE IF NOT EXISTS offres (
    id              UUID        PRIMARY KEY DEFAULT gen_random_uuid(),
    slug            TEXT        NOT NULL UNIQUE,
    source_url      TEXT        NOT NULL,
    source_host     TEXT        NOT NULL,
    source_hash     BYTEA       NOT NULL,
    entreprise      TEXT        NOT NULL,
    intitule        TEXT        NOT NULL,
    localisation    TEXT,
    contrat         TEXT,
    categorie       TEXT,
    raw_html        TEXT,
    raw_text        TEXT        NOT NULL,
    structured      JSONB       NOT NULL,
    embedding       vector(1024),
    scraped_at      TIMESTAMPTZ NOT NULL DEFAULT now(),
    last_seen_at    TIMESTAMPTZ NOT NULL DEFAULT now(),
    closed_at       TIMESTAMPTZ,
    UNIQUE (source_host, source_hash)
);

CREATE INDEX IF NOT EXISTS offres_entreprise_trgm ON offres USING gin (entreprise gin_trgm_ops);
CREATE INDEX IF NOT EXISTS offres_intitule_trgm   ON offres USING gin (intitule gin_trgm_ops);
CREATE INDEX IF NOT EXISTS offres_structured_gin  ON offres USING gin (structured jsonb_path_ops);
CREATE INDEX IF NOT EXISTS offres_embedding_hnsw  ON offres
    USING hnsw (embedding vector_cosine_ops)
    WITH (m = 16, ef_construction = 64);

-- ─────────────────────────────────────────────────────────────────────────
-- PROFILS
-- ─────────────────────────────────────────────────────────────────────────
CREATE TABLE IF NOT EXISTS profils (
    id              UUID        PRIMARY KEY DEFAULT gen_random_uuid(),
    label           TEXT        NOT NULL,
    content         JSONB       NOT NULL,
    is_active       BOOLEAN     NOT NULL DEFAULT false,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT now()
);
CREATE UNIQUE INDEX IF NOT EXISTS profils_one_active ON profils (is_active) WHERE is_active = true;

DO $$ BEGIN
    IF NOT EXISTS (SELECT 1 FROM pg_type WHERE typname = 'chunk_kind') THEN
        CREATE TYPE chunk_kind AS ENUM (
            'experience', 'projet', 'formation', 'competence', 'phrase_lettre'
        );
    END IF;
END $$;

CREATE TABLE IF NOT EXISTS chunks (
    id              UUID        PRIMARY KEY DEFAULT gen_random_uuid(),
    profil_id       UUID        NOT NULL REFERENCES profils(id) ON DELETE CASCADE,
    kind            chunk_kind  NOT NULL,
    titre           TEXT        NOT NULL,
    content         TEXT        NOT NULL,
    metadata        JSONB       NOT NULL DEFAULT '{}'::jsonb,
    embedding       vector(1024) NOT NULL,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT now()
);
CREATE INDEX IF NOT EXISTS chunks_profil    ON chunks(profil_id);
CREATE INDEX IF NOT EXISTS chunks_kind      ON chunks(kind);
CREATE INDEX IF NOT EXISTS chunks_embedding ON chunks USING hnsw (embedding vector_cosine_ops);

-- ─────────────────────────────────────────────────────────────────────────
-- INSTANCES
-- ─────────────────────────────────────────────────────────────────────────
DO $$ BEGIN
    IF NOT EXISTS (SELECT 1 FROM pg_type WHERE typname = 'instance_status') THEN
        CREATE TYPE instance_status AS ENUM (
            'draft', 'generating', 'ready', 'sent', 'archived', 'failed'
        );
    END IF;
END $$;

CREATE TABLE IF NOT EXISTS instances (
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
CREATE INDEX IF NOT EXISTS instances_offre  ON instances(offre_id);
CREATE INDEX IF NOT EXISTS instances_status ON instances(status);

-- ─────────────────────────────────────────────────────────────────────────
-- MESSAGES (CHAT)
-- ─────────────────────────────────────────────────────────────────────────
CREATE TABLE IF NOT EXISTS messages (
    id              UUID        PRIMARY KEY DEFAULT gen_random_uuid(),
    instance_id     UUID        NOT NULL REFERENCES instances(id) ON DELETE CASCADE,
    role            TEXT        NOT NULL, -- 'user' ou 'assistant'
    content         TEXT        NOT NULL,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT now()
);
CREATE INDEX IF NOT EXISTS messages_instance ON messages(instance_id);

-- ─────────────────────────────────────────────────────────────────────────
-- LLM_CALLS
-- ─────────────────────────────────────────────────────────────────────────
CREATE TABLE IF NOT EXISTS llm_calls (
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
CREATE INDEX IF NOT EXISTS llm_calls_instance ON llm_calls(instance_id);
CREATE INDEX IF NOT EXISTS llm_calls_hash     ON llm_calls(prompt_hash);

-- ─────────────────────────────────────────────────────────────────────────
-- VUE COÛTS
-- ─────────────────────────────────────────────────────────────────────────
CREATE OR REPLACE VIEW v_llm_costs_daily AS
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
