use crate::{ResistorError, ResistorColor};

pub struct Resistor {
    pub resistance: f32,
    pub tolerance: f32,
    pub temperature_coefficient: Option<f32>,
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
                let first = match first.as_digit() {
                    Some(s) => {s},
                    None => {
                        Err(ResistorError::custom(format!("{} is not a valid tens digit", first)))?
                    }
                };
                let second = match second.as_digit() {
                    Some(s) => {s},
                    None => {
                        Err(ResistorError::custom(format!("{} is not a valid digit", second)))?
                    }
                };
                let multiplier = match multiplier.as_multiplier() {
                    Some(s) => {s},
                    None => {
                        Err(ResistorError::custom(format!("{} is not a valid multiplier", multiplier)))?
                    }
                };
                let resistance = (first * 10.0 + second) * multiplier;
                let tolerance = match tolerance.as_tolerance() {
                    Some(s) => {s},
                    None => {
                        Err(ResistorError::custom(format!("{} is not a valid tolerance", tolerance)))?
                    }
                };
                Ok(Resistor {
                    resistance,
                    tolerance,
                    temperature_coefficient: None,
                })
            },
            ResistorBuilder::FiveBand {
                first,
                second,
                third,
                multiplier,
                tolerance,
            } => {
                let first = match first.as_digit() {
                    Some(s) => {s},
                    None => {
                        Err(ResistorError::custom(format!("{} is not a valid hundreds digit", first)))?
                    }
                };
                let second = match second.as_digit() {
                    Some(s) => {s},
                    None => {
                        Err(ResistorError::custom(format!("{} is not a valid tens digit", second)))?
                    }
                };
                let third = match third.as_digit() {
                    Some(s) => {s},
                    None => {
                        Err(ResistorError::custom(format!("{} is not a valid digit", third)))?
                    }
                };
                let multiplier = match multiplier.as_multiplier() {
                    Some(s) => {s},
                    None => {
                        Err(ResistorError::custom(format!("{} is not a valid multiplier", multiplier)))?
                    }
                };
                let resistance = (first * 100.0 + second * 10.0 + third) * multiplier;
                let tolerance = match tolerance.as_tolerance() {
                    Some(s) => {s},
                    None => {
                        Err(ResistorError::custom(format!("{} is not a valid tolerance", tolerance)))?
                    }
                };
                Ok(Resistor {
                    resistance,
                    tolerance,
                    temperature_coefficient: None,
                })
            },
            ResistorBuilder::SixBand {
                first,
                second,
                third,
                multiplier,
                tolerance,
                temperature_coefficient,
            } => {
                let first = match first.as_digit() {
                    Some(s) => {s},
                    None => {
                        Err(ResistorError::custom(format!("{} is not a valid hundreds digit", first)))?
                    }
                };
                let second = match second.as_digit() {
                    Some(s) => {s},
                    None => {
                        Err(ResistorError::custom(format!("{} is not a valid tens digit", second)))?
                    }
                };
                let third = match third.as_digit() {
                    Some(s) => {s},
                    None => {
                        Err(ResistorError::custom(format!("{} is not a valid digit", third)))?
                    }
                };
                let multiplier = match multiplier.as_multiplier() {
                    Some(s) => {s},
                    None => {
                        Err(ResistorError::custom(format!("{} is not a valid multiplier", multiplier)))?
                    }
                };
                let resistance = (first * 100.0 + second * 10.0 + third) * multiplier;
                let tolerance = match tolerance.as_tolerance() {
                    Some(s) => {s},
                    None => {
                        Err(ResistorError::custom(format!("{} is not a valid tolerance", tolerance)))?
                    }
                };
                let temperature_coefficient = match temperature_coefficient.as_temperature_coefficient() {
                    Some(s) => {s},
                    None => {
                        Err(ResistorError::custom(format!("{} is not a valid temperature coefficient", temperature_coefficient)))?
                    }
                };
                Ok(Resistor {
                    resistance,
                    tolerance,
                    temperature_coefficient: Some(temperature_coefficient),
                })
            },
        }
    }
}

