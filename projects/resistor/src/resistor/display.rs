use super::*;

impl Debug for Resistor {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Resistor")
            .field("resistance", &Ohms(self.resistance))
            .field("tolerance", &Percent(self.tolerance))
            .field("temperature_coefficient", &self.temperature_coefficient)
            .finish()
    }
}

impl Display for Resistor {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Resistor({}Ω ± {}%)", self.resistance, self.tolerance)
    }
}

struct Ohms(f64);

struct Percent(f64);

impl Debug for Percent {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}%", self.0)
    }
}

impl Debug for Ohms {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}Ω", self.0)
    }
}
