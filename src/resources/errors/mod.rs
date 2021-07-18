#[derive(Debug)]
pub struct Internal(pub String);

impl<E: std::error::Error> From<E> for Internal {
    fn from(err: E) -> Self {
        Self(err.to_string())
    }
}

impl ToString for Internal {
    fn to_string(&self) -> String {
        format!("INTERNAL({})", self.0)
    }
}
