#[derive(Debug)]
pub enum TransformationError {
    InvalidData(serde_json::Error),
    UnsupportedObservationType,
}

impl std::fmt::Display for TransformationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidData(error) => {
                write!(f, "Invalid observation data: {}", error)
            }
            Self::UnsupportedObservationType => {
                write!(f, "Unsupported observation type")
            }
        }
    }
}
