#[derive(Debug)]
pub struct ErrorReport {
    pub message: String,
    pub char: usize,
}

impl ErrorReport {
    pub fn new(message: String, char: usize) -> ErrorReport {
        ErrorReport { message, char }
    }
}
