use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum PngdefryError {
    Msg(String),
}

impl Display for PngdefryError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            PngdefryError::Msg(msg) => write!(f, "{}", msg),
        }
    }
}

impl std::error::Error for PngdefryError {}

unsafe impl Send for PngdefryError {}

unsafe impl Sync for PngdefryError {}

