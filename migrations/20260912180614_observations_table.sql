-- Add migration script here
CREATE TYPE observation_type AS ENUM (
    'quiz_attempt_graded',
    'lesson_completed',
    'video_watched',
    'question_answered',
    'quiz_attempt_started',
    'assignment_submitted',
    'live_session_attended'
    );

CREATE TABLE observations (
                              id UUID PRIMARY KEY,

                              event_type observation_type NOT NULL,
                              version INTEGER NOT NULL DEFAULT 1,

                              learner_id UUID NOT NULL,

                              occurred_at TIMESTAMPTZ NOT NULL,

                              source_service VARCHAR(100) NOT NULL,
                              source_event_id UUID NOT NULL,

                              data JSONB NOT NULL,

                              created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

                              CONSTRAINT observations_source_event_unique
                                  UNIQUE (source_service, source_event_id)
);

CREATE INDEX idx_observations_learner_id
    ON observations (learner_id);

CREATE INDEX idx_observations_event_type
    ON observations (event_type);

CREATE INDEX idx_observations_occurred_at
    ON observations (occurred_at);

CREATE INDEX idx_observations_learner_occurred_at
    ON observations (learner_id, occurred_at);
