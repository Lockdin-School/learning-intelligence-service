-- Add migration script here
CREATE TABLE concepts_lookup
(
    concept_id UUID PRIMARY KEY,
    topic_id   UUID NOT NULL,
    subject_id UUID NOT NULL
);