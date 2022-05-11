#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ResistorColor {
    Silver,
    Gold,
    Black,
    Brown,
    Red,
    Orange,
    Yellow,
    Green,
    Blue,
    Violet,
    Grey,
    White,
    Empty,
}

impl ResistorColor {
    pub fn as_multiplier(&self) -> Option<f32> {
        match self {
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
        }
    }
}
