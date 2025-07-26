#[derive(Debug)]
pub struct Error(String);

impl Error {
    pub fn new(message: &str) -> Self {
        Self(message.to_string())
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for Error {}
