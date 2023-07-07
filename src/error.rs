use std::{error::Error, fmt::Display};

/// charのパースに関するエラー
#[derive(Debug)]
pub struct ParseCharError(pub char);

impl Display for ParseCharError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ParseCharError: '{}' cannot parse.", self.0)
    }
}

impl Error for ParseCharError {}

/// パース全般に関するエラー
#[derive(Debug)]
pub enum ParseError {
    ParseCharError(ParseCharError),
}

impl From<ParseCharError> for ParseError {
    fn from(value: ParseCharError) -> Self {
        ParseError::ParseCharError(value)
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let e_string = match self {
            e @ ParseError::ParseCharError(_) => e.to_string(),
        };
        write!(f, "{e_string}")
    }
}

impl Error for ParseError {}

/// データ構造の変換に伴うエラー
#[derive(Debug)]
pub struct ConversionStructureError(pub String);

impl Display for ConversionStructureError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ConversionStructureError: {}", self.0)
    }
}

impl Error for ConversionStructureError {}

/// カスタムエラー
#[derive(Debug)]
pub struct CustomError(String);

impl CustomError {
    pub fn new<T: Display>(msg: T) -> Self {
        Self(msg.to_string())
    }
}

impl Display for CustomError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "CustomError: {}", self.0)
    }
}

impl Error for CustomError {}
