#[derive(Debug)]
pub struct ParseError {
    message: String,
}

impl ParseError {
    pub fn new<E: std::fmt::Debug>(err: nom::Err<E>) -> ParseError {
        ParseError { message: err.to_string() }
    }
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for ParseError {}
