#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RuntimeError {
    InvalidSnapshot(String),
    AlreadyRegistered(String),
    NotRegistered(String),
    CursorExpired,
    ProviderBoundary(String),
}
impl std::fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidSnapshot(message) => write!(f, "invalid snapshot: {message}"),
            Self::AlreadyRegistered(id) => write!(f, "provider {id} already registered"),
            Self::NotRegistered(id) => write!(f, "provider {id} is not registered"),
            Self::ProviderBoundary(id) => {
                write!(f, "snapshot contains records outside provider {id}")
            }
            Self::CursorExpired => write!(f, "cursor unavailable; reconcile with a snapshot"),
        }
    }
}
impl std::error::Error for RuntimeError {}
