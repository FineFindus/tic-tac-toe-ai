use std::fmt::Display;

#[derive(Debug, Copy, Clone, Default, Eq, PartialEq)]
pub enum PlaceValue {
    X,
    O,
    #[default]
    Empty,
}

impl Display for PlaceValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                PlaceValue::X => 'X',
                PlaceValue::O => 'O',
                PlaceValue::Empty => ' ',
            }
        )
    }
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

    pub fn get_cell(&self, index: usize) -> Option<&PlaceValue> {
        self.0.get(index).and_then(|cell| {
            if cell == &PlaceValue::Empty {
                None
            } else {
                Some(cell)
            }
        })
    }

    pub fn reset_cell(&mut self, index: usize) {
        assert!(index < self.0.len());
        self.0[index] = PlaceValue::Empty;
    }

    pub fn place_value(&mut self, index: usize, value: PlaceValue) {
        assert!(index < self.0.len());
        self.0[index] = value;
    }

    pub fn is_finished(&self) -> bool {
        self.eval_winner().is_some() || self.0.iter().all(|cell| cell != &PlaceValue::Empty)
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

    pub fn eval_winner(&self) -> Option<PlaceValue> {
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

    pub fn play(&mut self) -> usize {
        let (cell, _score) = self
            .available_cells()
            .iter()
            .map(|&cell| {
                self.place_value(cell, PlaceValue::O);
                let score = self.minimax(0, i32::MIN, i32::MAX, false);
                self.reset_cell(cell);
                (cell, score)
            })
            .max_by(|(_, score_x), (_, score_y)| score_x.cmp(score_y))
            .expect("Failed to get best move");
        cell
    }

    fn available_cells(&self) -> Vec<usize> {
        self.0
            .iter()
            .enumerate()
            .filter(|(_, cell)| cell == &&PlaceValue::Empty)
            .map(|(index, _)| index)
            .collect()
    }

    fn score_winner(&self, depth: i32) -> i32 {
        match self.eval_winner() {
            Some(PlaceValue::X) => depth - 10,
            Some(PlaceValue::O) => 10 - depth,
            _ => 0,
        }
    }
    fn minimax(
        &mut self,
        depth: i32,
        mut alpha: i32,
        mut beta: i32,
        maximizing_player: bool,
    ) -> i32 {
        if self.is_finished() {
            return self.score_winner(depth);
        }

        let mut best_value = if maximizing_player {
            i32::MIN
        } else {
            i32::MAX
        };
        let place_val = if maximizing_player {
            PlaceValue::O
        } else {
            PlaceValue::X
        };

        for cell in self.available_cells() {
            self.place_value(cell, place_val);
            let eval = self.minimax(depth + 1, alpha, beta, !maximizing_player);
            self.reset_cell(cell);

            if maximizing_player {
                best_value = best_value.max(eval);
                alpha = alpha.max(eval);
            } else {
                best_value = best_value.min(eval);
                beta = beta.min(eval);
            }

            if beta <= alpha {
                break;
            }
        }

        best_value
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // ╔═══╦═══╦═══╗
        // ║ X ║   ║   ║
        // ╠═══╬═══╬═══╣
        // ║   ║   ║   ║
        // ╠═══╬═══╬═══╣
        // ║   ║   ║   ║
        // ╚═══╩═══╩═══╝
        writeln!(f, "╔═══╦═══╦═══╗")?;
        writeln!(f, "║ {} ║ {} ║ {} ║", self.0[0], self.0[1], self.0[2])?;
        writeln!(f, "╠═══╬═══╬═══╣")?;
        writeln!(f, "║ {} ║ {} ║ {} ║", self.0[3], self.0[4], self.0[5])?;
        writeln!(f, "╠═══╬═══╬═══╣")?;
        writeln!(f, "║ {} ║ {} ║ {} ║", self.0[6], self.0[7], self.0[8])?;
        writeln!(f, "╚═══╩═══╩═══╝")?;
        Ok(())
    }
}
