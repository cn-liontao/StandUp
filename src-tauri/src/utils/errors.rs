use log::SetLoggerError;

pub struct StandingError {
    pub message: String,
}

#[derive(Debug)]
pub struct ParsingError {
    pub message: String,
}

impl From<SetLoggerError> for ParsingError {
    fn from(value: SetLoggerError) -> Self {
        ParsingError::init_str(value.to_string())
    }
}

impl ParsingError {
    pub(crate) fn init(error_msg: &str) -> ParsingError {
        ParsingError { message: error_msg.to_string() }
    }
    pub(crate) fn init_str(error_msg: String) -> ParsingError {
        ParsingError { message: error_msg }
    }
}