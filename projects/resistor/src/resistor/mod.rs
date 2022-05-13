use crate::{ResistorError, ResistorColor};

pub struct Resistor {
    pub resistance: f64,
    pub tolerance: f64,
    pub temperature_coefficient: Option<f64>,
}

pub enum ResistorBuilder {
    FourBand {
        first: ResistorColor,
        second: ResistorColor,
        multiplier: ResistorColor,
        tolerance: ResistorColor,
    },
    FiveBand {
        first: ResistorColor,
        second: ResistorColor,
        third: ResistorColor,
        multiplier: ResistorColor,
        tolerance: ResistorColor,
    },
    SixBand {
        first: ResistorColor,
        second: ResistorColor,
        third: ResistorColor,
        multiplier: ResistorColor,
        tolerance: ResistorColor,
        temperature_coefficient: ResistorColor,
    },
}

impl ResistorBuilder {
    pub fn build(&self) -> Result<Resistor, ResistorError> {
        match self {
            ResistorBuilder::FourBand {
                first,
                second,
                multiplier,
                tolerance,
            } => {
                let base = tens_digits(first, second)?;
                let resistance = mul_multiplier(base, multiplier)?;
                Ok(Resistor {
                    resistance,
                    tolerance: tolerance.as_tolerance()?,
                    temperature_coefficient: None,
                })
            }
            ResistorBuilder::FiveBand {
                first,
                second,
                third,
                multiplier,
                tolerance,
            } => {
                let tens = tens_digits(second, third)?;
                let hundreds = add_hundreds_digit(tens, first)?;
                let resistance = mul_multiplier(hundreds, multiplier)?;
                Ok(Resistor {
                    resistance,
                    tolerance: tolerance.as_tolerance()?,
                    temperature_coefficient: None,
                })
            }
            ResistorBuilder::SixBand {
                first,
                second,
                third,
                multiplier,
                tolerance,
                temperature_coefficient,
            } => {
                let first = match first.as_digit() {
                    Some(s) => { s }
                    None => {
                        Err(ResistorError::custom(format!("{} is not a valid hundreds digit", first)))?
                    }
                };
                let second = match second.as_digit() {
                    Some(s) => { s }
                    None => {
                        Err(ResistorError::custom(format!("{} is not a valid tens digit", second)))?
                    }
                };
                let third = match third.as_digit() {
                    Some(s) => { s }
                    None => {
                        Err(ResistorError::custom(format!("{} is not a valid digit", third)))?
                    }
                };
                let multiplier = match multiplier.as_multiplier() {
                    Some(s) => { s }
                    None => {
                        Err(ResistorError::custom(format!("{} is not a valid multiplier", multiplier)))?
                    }
                };
                let resistance = (first * 100.0 + second * 10.0 + third) * multiplier;
                let tolerance = match tolerance.as_tolerance() {
                    Some(s) => { s }
                    None => {
                        Err(ResistorError::custom(format!("{} is not a valid tolerance", tolerance)))?
                    }
                };
                let temperature_coefficient = temperature_coefficient.as_temperature_coefficient()?;
                Ok(Resistor {
                    resistance,
                    tolerance,
                    temperature_coefficient: Some(temperature_coefficient),
                })
            }
        }
    }
}

#[inline]
fn tens_digits(tens: &ResistorColor, digits: &ResistorColor) -> Result<f32, ResistorError> {
    let tens = match tens.as_digit() {
        Some(s) => { s }
        None => {
            Err(ResistorError::custom(format!("{} is not a valid tens digit", tens)))?
        }
    };
    let digits = match digits.as_digit() {
        Some(s) => { s }
        None => {
            Err(ResistorError::custom(format!("{} is not a valid digit", digits)))?
        }
    };
    Ok(digits + tens * 10.0)
}

#[inline]
fn add_hundreds_digit(base: f32, digits: &ResistorColor) -> Result<f32, ResistorError> {
    let digits = match digits.as_digit() {
        Some(s) => { s }
        None => {
            Err(ResistorError::custom(format!("{} is not a valid hundreds digit", digits)))?
        }
    };
    Ok(base + digits * 100.0)
}

#[inline]
fn add_thousands_digit(base: f32, digits: &ResistorColor) -> Result<f32, ResistorError> {
    let digits = match digits.as_digit() {
        Some(s) => { s }
        None => {
            Err(ResistorError::custom(format!("{} is not a valid thousands digit", digits)))?
        }
    };
    Ok(base + digits * 1000.0)
}

#[inline]
fn mul_multiplier(base: f32, multiplier: &ResistorColor) -> Result<f32, ResistorError> {
    let multiplier = match multiplier.as_multiplier() {
        Some(s) => { s }
        None => {
            Err(ResistorError::custom(format!("{} is not a valid multiplier", multiplier)))?
        }
    };
    Ok(base * multiplier)
}