-- Add migration script here
-- current_progress: one row per (student, concept), always the latest state.
-- Acts as a materialized cache over progress_history — never write to it
-- except via the upsert, and it should always be fully rebuildable from history.
CREATE TABLE current_progress
(
    student_id         UUID        NOT NULL,
    concept_id         UUID        NOT NULL,
    questions_answered INTEGER     NOT NULL DEFAULT 0
        CHECK (questions_answered >= 0),
    questions_correct  INTEGER     NOT NULL DEFAULT 0
        CHECK (questions_correct >= 0
            AND questions_correct <= questions_answered),
    accuracy           NUMERIC(5, 4) GENERATED ALWAYS AS (
        CASE
            WHEN questions_answered = 0 THEN 0
            ELSE questions_correct::numeric / questions_answered
            END
        ) STORED,
    updated_at         TIMESTAMPTZ NOT NULL DEFAULT now(),
    PRIMARY KEY (student_id, concept_id)
);

CREATE INDEX idx_current_progress_concept ON current_progress (concept_id);

-- progress_history: append-only, one row per calculated cumulative state.
-- Never updated or deleted — this is the source of truth current_progress
-- is derived from, and what the trend queries read against.
CREATE TABLE progress_history
(
    id                 BIGSERIAL PRIMARY KEY,
    student_id         UUID        NOT NULL,
    concept_id         UUID        NOT NULL,
    observation_id     UUID        NOT NULL REFERENCES observations (id),
    questions_answered INTEGER     NOT NULL CHECK (questions_answered >= 0),
    questions_correct  INTEGER     NOT NULL
        CHECK (questions_correct >= 0
            AND questions_correct <= questions_answered),
    accuracy           NUMERIC(5, 4) GENERATED ALWAYS AS (
        CASE
            WHEN questions_answered = 0 THEN 0
            ELSE questions_correct::numeric / questions_answered
            END
        ) STORED,
    created_at         TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX idx_progress_history_lookup
    ON progress_history (student_id, concept_id, created_at DESC);