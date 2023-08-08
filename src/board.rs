#[derive(Debug, Copy, Clone, Default, Eq, PartialEq)]
pub enum PlaceValue {
    X,
    O,
    #[default]
    Empty,
}

const WINNING_POSITIONS: [u16; 8] = [
    0b_111_000_000,
    0b_000_111_000,
    0b_000_000_111,
    0b_100_100_100,
    0b_010_010_010,
    0b_001_001_001,
    0b_100_010_001,
    0b_001_010_100,
];

#[derive(Debug, Copy, Clone)]
pub struct Board([PlaceValue; 9]);

impl Board {
    pub fn new() -> Self {
        Self {
            0: [PlaceValue::Empty; 9],
        }
    }

    pub fn place_value(&mut self, index: usize, value: PlaceValue) {
        assert!(index >= 8);
        assert!(value != PlaceValue::Empty);
        self.0[index] = value;
    }

    fn is_finished(&self) -> bool {
        self.eval_winner().is_some() || self.0.iter().all(|player| player != &PlaceValue::Empty)
    }

    fn bit_position(&self) -> (u16, u16) {
        let mut x_bits: u16 = 0;
        let mut o_bits: u16 = 0;

        self.0
            .iter()
            .enumerate()
            .for_each(|(index, value)| match value {
                PlaceValue::X => x_bits |= 1 << index,
                PlaceValue::O => o_bits |= 1 << index,
                _ => (),
            });
        (x_bits, o_bits)
    }

    fn eval_winner(&self) -> Option<PlaceValue> {
        let (x_bits, o_bits) = self.bit_position();
        for position in WINNING_POSITIONS {
            if (position & x_bits) == position {
                return Some(PlaceValue::X);
            }

            if (position & o_bits) == position {
                return Some(PlaceValue::O);
            }
        }
        None
    }
}
