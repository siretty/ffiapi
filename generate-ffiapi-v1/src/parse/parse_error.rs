#[derive(Debug)]
pub enum ParseError {
    GrammarFailsToMatch(Box<dyn std::error::Error>),
}

impl std::error::Error for ParseError {}

impl core::fmt::Display for ParseError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        use core::fmt::Display;
        match self {
            ParseError::GrammarFailsToMatch(err) => Display::fmt(err, f),
        }
    }
}
