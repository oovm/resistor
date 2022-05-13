use crate::{ResistorColor, ResistorError};
use std::fmt::{Debug, Display, Formatter};

mod display;

/// A resistor.
#[derive(Copy, Clone)]
pub struct Resistor {
    /// The resistance of the resistor in ohms.
    pub resistance: f64,
    /// The tolerance of the resistor in percent.
    pub tolerance: f64,
    /// The temperature coefficient of the resistor in ppm/K.
    pub temperature_coefficient: Option<f64>,
}

/// A builder for a resistor.
#[derive(Copy, Clone, Debug)]
pub enum ResistorBuilder {
    /// A four band resistor.
    FourBand {
        /// The 1st band, tens digit.
        first: ResistorColor,
        /// The 2nd band, ones digit.
        second: ResistorColor,
        /// The 3rd band, multiplier.
        multiplier: ResistorColor,
        /// The 4th band, tolerance.
        tolerance: ResistorColor,
    },
    /// A five band resistor.
    FiveBand {
        /// The 1st band, hundreds digit.
        first: ResistorColor,
        /// The 2nd band, tens digit.
        second: ResistorColor,
        /// The 3rd band, ones digit.
        third: ResistorColor,
        /// The 4th band, multiplier.
        multiplier: ResistorColor,
        /// The 5th band, tolerance.
        tolerance: ResistorColor,
    },
    /// A six band resistor.
    SixBand {
        /// The 1st band, hundreds digit.
        first: ResistorColor,
        /// The 2nd band, tens digit.
        second: ResistorColor,
        /// The 3rd band, ones digit.
        third: ResistorColor,
        /// The 4th band, multiplier.
        multiplier: ResistorColor,
        /// The 5th band, tolerance.
        tolerance: ResistorColor,
        /// The 6th band, temperature coefficient.
        temperature_coefficient: ResistorColor,
    },
}

impl ResistorBuilder {
    /// Build the resistor.
    pub fn build(&self) -> Result<Resistor, ResistorError> {
        match self {
            ResistorBuilder::FourBand { first, second, multiplier, tolerance } => {
                let tens = first.as_digit("tens")?;
                let digit = second.as_digit("ones")?;
                let base = tens * 10.0 + digit;
                let multiplier = multiplier.as_multiplier()?;
                Ok(Resistor {
                    resistance: base * multiplier,
                    tolerance: tolerance.as_tolerance()?,
                    temperature_coefficient: None,
                })
            }
            ResistorBuilder::FiveBand { first, second, third, multiplier, tolerance } => {
                let hundreds = first.as_digit("hundreds")?;
                let tens = second.as_digit("tens")?;
                let digit = third.as_digit("ones")?;
                let base = hundreds * 100.0 + tens * 10.0 + digit;
                let multiplier = multiplier.as_multiplier()?;
                Ok(Resistor {
                    resistance: base * multiplier,
                    tolerance: tolerance.as_tolerance()?,
                    temperature_coefficient: None,
                })
            }
            ResistorBuilder::SixBand { first, second, third, multiplier, tolerance, temperature_coefficient } => {
                let thousands = first.as_digit("thousands")?;
                let hundreds = second.as_digit("hundreds")?;
                let tens = third.as_digit("tens")?;
                let digit = multiplier.as_digit("ones")?;
                let base = thousands * 1_000.0 + hundreds * 100.0 + tens * 10.0 + digit;
                let multiplier = temperature_coefficient.as_multiplier()?;
                Ok(Resistor {
                    resistance: base * multiplier,
                    tolerance: tolerance.as_tolerance()?,
                    temperature_coefficient: None,
                })
            }
        }
    }
}
