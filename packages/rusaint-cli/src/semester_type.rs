use std::{fmt::Display, ops::Deref, str::FromStr};

use thiserror::Error;

#[derive(Debug, Clone, Copy)]
pub struct SemesterType(rusaint::model::SemesterType);

impl Deref for SemesterType {
    type Target = rusaint::model::SemesterType;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Display for SemesterType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let semester = match self.0 {
            rusaint::model::SemesterType::One => "1학기",
            rusaint::model::SemesterType::Summer => "여름학기",
            rusaint::model::SemesterType::Two => "2학기",
            rusaint::model::SemesterType::Winter => "겨울학기",
        };
        write!(f, "{}", semester)
    }
}

#[derive(Debug, Error)]
pub enum ParseSemesterTypeError {
    #[error("invalid semester type")]
    InvalidSemesterType,
}

impl FromStr for SemesterType {
    type Err = ParseSemesterTypeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "1" | "one" => Ok(SemesterType(rusaint::model::SemesterType::One)),
            "summer" => Ok(SemesterType(rusaint::model::SemesterType::Summer)),
            "2" | "two" => Ok(SemesterType(rusaint::model::SemesterType::Two)),
            "winter" => Ok(SemesterType(rusaint::model::SemesterType::Winter)),
            _ => Err(ParseSemesterTypeError::InvalidSemesterType),
        }
    }
}
