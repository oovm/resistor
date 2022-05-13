use crate::ResistorError;
use std::fmt::{Display, Formatter};

/// The colors that can be used on a resistor.
#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ResistorColor {
    /// Pink band.
    Pink,
    /// Silver band.
    Silver,
    /// Gold band.
    Gold,
    /// Black band.
    Black,
    /// Brown band.
    Brown,
    /// Red band.
    Red,
    /// Orange band.
    Orange,
    /// Yellow band.
    Yellow,
    /// Green band.
    Green,
    /// Blue band.
    Blue,
    /// Violet band.
    Violet,
    /// Grey band.
    Grey,
    /// White band.
    White,
    /// Empty band.
    Empty,
}

impl Display for ResistorColor {
    /// eg. PK, SR
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ResistorColor::Pink => write!(f, "PK"),
            ResistorColor::Silver => write!(f, "SR"),
            ResistorColor::Gold => write!(f, "GD"),
            ResistorColor::Black => write!(f, "BK"),
            ResistorColor::Brown => write!(f, "BN"),
            ResistorColor::Red => write!(f, "RD"),
            ResistorColor::Orange => write!(f, "OG"),
            ResistorColor::Yellow => write!(f, "YL"),
            ResistorColor::Green => write!(f, "GN"),
            ResistorColor::Blue => write!(f, "BL"),
            ResistorColor::Violet => write!(f, "VT"),
            ResistorColor::Grey => write!(f, "GY"),
            ResistorColor::White => write!(f, "WT"),
            ResistorColor::Empty => write!(f, "  "),
        }
    }
}

impl ResistorColor {
    /// Get the digit value of the color, returns an error if the color is not a valid digit.
    pub fn as_digit(&self, position: &'static str) -> Result<f64, ResistorError> {
        let digit = match self {
            ResistorColor::Pink => None,
            ResistorColor::Silver => None,
            ResistorColor::Gold => None,
            ResistorColor::Black => Some(0.0),
            ResistorColor::Brown => Some(1.0),
            ResistorColor::Red => Some(2.0),
            ResistorColor::Orange => Some(3.0),
            ResistorColor::Yellow => Some(4.0),
            ResistorColor::Green => Some(5.0),
            ResistorColor::Blue => Some(6.0),
            ResistorColor::Violet => Some(7.0),
            ResistorColor::Grey => Some(8.0),
            ResistorColor::White => Some(9.0),
            ResistorColor::Empty => None,
        };
        match digit {
            Some(s) => Ok(s),
            None => Err(ResistorError::custom(format!("{self} is not a valid {position} digit color"))),
        }
    }
    /// Get the multiplier value of the color, returns an error if the color is not a valid multiplier.
    pub fn as_multiplier(&self) -> Result<f64, ResistorError> {
        let multiplier = match self {
            ResistorColor::Pink => Some(0.001),
            ResistorColor::Silver => Some(0.01),
            ResistorColor::Gold => Some(0.1),
            ResistorColor::Black => Some(1.0),
            ResistorColor::Brown => Some(10.0),
            ResistorColor::Red => Some(100.0),
            ResistorColor::Orange => Some(1_000.0),
            ResistorColor::Yellow => Some(10_000.0),
            ResistorColor::Green => Some(100_000.0),
            ResistorColor::Blue => Some(1_000_000.0),
            ResistorColor::Violet => Some(10_000_000.0),
            ResistorColor::Grey => Some(100_000_000.0),
            ResistorColor::White => Some(1_000_000_000.0),
            ResistorColor::Empty => None,
        };
        match multiplier {
            Some(s) => Ok(s),
            None => Err(ResistorError::custom(format!("{} is not a valid multiplier color", self))),
        }
    }
    /// Get the tolerance value of the color, returns an error if the color is not a valid tolerance.
    pub fn as_tolerance(&self) -> Result<f64, ResistorError> {
        let tolerance = match self {
            ResistorColor::Pink => None,
            ResistorColor::Silver => Some(10.0),
            ResistorColor::Gold => Some(5.0),
            ResistorColor::Black => None,
            ResistorColor::Brown => Some(1.0),
            ResistorColor::Red => Some(2.0),
            ResistorColor::Orange => Some(0.05),
            ResistorColor::Yellow => Some(0.02),
            ResistorColor::Green => Some(0.5),
            ResistorColor::Blue => Some(0.25),
            ResistorColor::Violet => Some(0.1),
            ResistorColor::Grey => Some(0.01),
            ResistorColor::White => None,
            ResistorColor::Empty => Some(20.0),
        };
        match tolerance {
            Some(s) => Ok(s),
            None => Err(ResistorError::custom(format!("{} is not a valid tolerance color", self))),
        }
    }
    /// Get the tolerance letter of the color, returns an error if the color is not a valid tolerance.
    pub fn as_tolerance_letter(&self) -> Option<char> {
        match self {
            ResistorColor::Pink => None,
            ResistorColor::Silver => Some('J'),
            ResistorColor::Gold => Some('K'),
            ResistorColor::Black => None,
            ResistorColor::Brown => Some('F'),
            ResistorColor::Red => Some('G'),
            ResistorColor::Orange => Some('D'),
            ResistorColor::Yellow => Some('C'),
            ResistorColor::Green => Some('B'),
            ResistorColor::Blue => Some('A'),
            ResistorColor::Violet => Some('V'),
            ResistorColor::Grey => Some('S'),
            ResistorColor::White => None,
            ResistorColor::Empty => Some('M'),
        }
    }
    /// Get the temperature coefficient value of the color, returns an error if the color is not a valid temperature coefficient.
    pub fn as_temperature_coefficient(&self) -> Result<f64, ResistorError> {
        let temperature_coefficient = match self {
            ResistorColor::Pink => None,
            ResistorColor::Silver => None,
            ResistorColor::Gold => None,
            ResistorColor::Black => Some(250.0),
            ResistorColor::Brown => Some(100.0),
            ResistorColor::Red => Some(50.0),
            ResistorColor::Orange => Some(15.0),
            ResistorColor::Yellow => Some(25.0),
            ResistorColor::Green => Some(20.0),
            ResistorColor::Blue => Some(10.0),
            ResistorColor::Violet => Some(5.0),
            ResistorColor::Grey => Some(1.0),
            ResistorColor::White => None,
            ResistorColor::Empty => None,
        };
        match temperature_coefficient {
            Some(s) => Ok(s),
            None => Err(ResistorError::custom(format!("{} is not a valid temperature coefficient color", self))),
        }
    }
    /// Get the temperature coefficient letter of the color, returns an error if the color is not a valid temperature coefficient.
    pub fn as_temperature_letter(&self) -> Option<char> {
        match self {
            ResistorColor::Pink => None,
            ResistorColor::Silver => None,
            ResistorColor::Gold => None,
            ResistorColor::Black => Some('Z'),
            ResistorColor::Brown => Some('Y'),
            ResistorColor::Red => Some('R'),
            ResistorColor::Orange => Some('P'),
            ResistorColor::Yellow => Some('Q'),
            ResistorColor::Green => Some('Z'),
            ResistorColor::Blue => Some('M'),
            ResistorColor::Violet => Some('L'),
            ResistorColor::Grey => Some('K'),
            ResistorColor::White => None,
            ResistorColor::Empty => None,
        }
    }
}
