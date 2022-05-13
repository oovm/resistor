Resistor
========

resistor is a library for calculating the resistance of a resistor based on the color bands.


```rust
use resistor::{ResistorBuilder, ResistorColor};
#[test]
fn test_red4() {
    let resistor = ResistorBuilder::FourBand {
        first: ResistorColor::Red,
        second: ResistorColor::Red,
        multiplier: ResistorColor::Red,
        tolerance: ResistorColor::Red,
    }
    .build()
    .unwrap();
    assert_eq!(resistor.resistance, 2200.0);
    assert_eq!(resistor.to_string(), "Resistor(2200Ω ± 2%)");
}
```