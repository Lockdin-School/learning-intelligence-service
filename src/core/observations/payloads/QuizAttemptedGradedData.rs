use uuid::Uuid;

pub struct QuizAttemptGradedData {
    pub quiz_id: Uuid,
    pub attempt_id: Uuid,

    pub subject_id: Uuid,
    pub topic_id: Uuid,

    pub score: i32,
    pub max_score: i32,
    pub percentage: f64,

    pub items: Vec<QuizItemObservation>,
}

pub struct QuizItemObservation {
    pub question_id: Uuid,

    pub correct: bool,

    pub score: i32,
    pub max_score: i32,

    pub concept_ids: Vec<Uuid>,
}
