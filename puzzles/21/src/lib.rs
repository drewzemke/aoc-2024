use common::point::Point;

pub mod puzzle21a;
pub mod puzzle21b;

#[derive(Debug, Clone, Copy)]
pub enum NumpadButton {
    Number(u32),
    Act,
}

impl From<char> for NumpadButton {
    fn from(c: char) -> Self {
        if let Some(n) = c.to_digit(10) {
            Self::Number(n)
        } else {
            Self::Act
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum DirpadButton {
    Up,
    Down,
    Left,
    Right,
    Act,
}

impl From<DirpadButton> for char {
    fn from(value: DirpadButton) -> Self {
        match value {
            DirpadButton::Up => '^',
            DirpadButton::Down => 'v',
            DirpadButton::Left => '<',
            DirpadButton::Right => '>',
            DirpadButton::Act => 'A',
        }
    }
}

impl NumpadButton {
    /// The numpad looks like this:
    /// ```
    /// +---+---+---+
    /// | 7 | 8 | 9 |
    /// +---+---+---+
    /// | 4 | 5 | 6 |
    /// +---+---+---+
    /// | 1 | 2 | 3 |
    /// +---+---+---+
    ///     | 0 | A |
    ///     +---+---+
    /// ```
    fn coords(&self) -> Point {
        match self {
            Self::Number(0) => (3, 1).into(),
            Self::Number(1) => (2, 0).into(),
            Self::Number(2) => (2, 1).into(),
            Self::Number(3) => (2, 2).into(),
            Self::Number(4) => (1, 0).into(),
            Self::Number(5) => (1, 1).into(),
            Self::Number(6) => (1, 2).into(),
            Self::Number(7) => (0, 0).into(),
            Self::Number(8) => (0, 1).into(),
            Self::Number(9) => (0, 2).into(),
            Self::Act => (3, 2).into(),
            _ => unreachable!(),
        }
    }

    fn dir_buttons_to_press(from: &Self, to: &Self) -> Vec<DirpadButton> {
        let from = from.coords();
        let to = to.coords();
        let diff = to - from;

        // move vertically, then horizontally
        let row = diff.row;
        let v_button = if row > 0 {
            DirpadButton::Down
        } else {
            DirpadButton::Up
        };
        let v_buttons = vec![v_button; row.unsigned_abs() as usize];

        let col = diff.col;
        let h_button = if col > 0 {
            DirpadButton::Right
        } else {
            DirpadButton::Left
        };
        let h_buttons = vec![h_button; col.unsigned_abs() as usize];

        let mut result = vec![];

        // strats:
        // - group by direction
        // - prefer left first
        // - then down first
        // - then up-right over right-up
        if diff.col < 0 {
            // move left first, unless doing so would cause us to hit the blank
            if from.row == 3 && (from.col == -diff.col) {
                result.extend(v_buttons);
                result.extend(h_buttons);
            } else {
                result.extend(h_buttons);
                result.extend(v_buttons);
            }
        } else if diff.row > 0 {
            // move down first, unless doing so would cause us to hit the blank
            if from.col == 0 && (from.row == 3 - diff.row) {
                result.extend(h_buttons);
                result.extend(v_buttons);
            } else {
                result.extend(v_buttons);
                result.extend(h_buttons);
            }
        } else {
            result.extend(v_buttons);
            result.extend(h_buttons);
        }

        result.push(DirpadButton::Act);
        result
    }

    /// Computes the sequence of dirpad buttons required to press a given
    /// sequence of numpad buttons, where there are a given number of layers
    /// of dirpad button-based controllers on top of the numpad button.
    ///
    /// `layers`=0  would be as if you were directly pressing numpad buttons,
    /// `layers`=1  is using dirpad controlling the numpad controller
    /// `layers`=2  is using dirpad controlling a dirpad controlling the numpad controller
    /// ... and so on
    pub fn seq_to_press(seq: &[Self], layers: usize) -> Vec<DirpadButton> {
        let mut cursor = Self::Act;
        let dirpad_buttons = seq
            .iter()
            .flat_map(|numpad_button| {
                let dir_buttons = Self::dir_buttons_to_press(&cursor, numpad_button);
                cursor = *numpad_button;
                dir_buttons
            })
            .collect::<Vec<_>>();

        // ignoring the case where layers=0, pass this list to the DirpadButton impl
        DirpadButton::seq_to_press(&dirpad_buttons, layers - 1)
    }
}

impl DirpadButton {
    /// The dirpad looks like this:
    /// ```
    ///     +---+---+
    ///     | ^ | A |
    /// +---+---+---+
    /// | < | v | > |
    /// +---+---+---+
    /// ```
    fn dir_buttons_to_press(from: &Self, to: &Self) -> Vec<Self> {
        let mut moves = match (from, to) {
            (Self::Up, Self::Down) => vec![Self::Down],
            (Self::Up, Self::Left) => vec![Self::Down, Self::Left],
            (Self::Up, Self::Right) => vec![Self::Down, Self::Right],
            (Self::Up, Self::Act) => vec![Self::Right],
            (Self::Down, Self::Up) => vec![Self::Up],
            (Self::Down, Self::Left) => vec![Self::Left],
            (Self::Down, Self::Right) => vec![Self::Right],
            (Self::Down, Self::Act) => vec![Self::Up, Self::Right],
            (Self::Left, Self::Up) => vec![Self::Right, Self::Up],
            (Self::Left, Self::Down) => vec![Self::Right],
            (Self::Left, Self::Right) => vec![Self::Right, Self::Right],
            (Self::Left, Self::Act) => vec![Self::Right, Self::Right, Self::Up],
            (Self::Right, Self::Up) => vec![Self::Left, Self::Up],
            (Self::Right, Self::Down) => vec![Self::Left],
            (Self::Right, Self::Left) => vec![Self::Left, Self::Left],
            (Self::Right, Self::Act) => vec![Self::Up],
            (Self::Act, Self::Up) => vec![Self::Left],
            (Self::Act, Self::Down) => vec![Self::Left, Self::Down],
            (Self::Act, Self::Left) => vec![Self::Down, Self::Left, Self::Left],
            (Self::Act, Self::Right) => vec![Self::Down],
            _ => vec![],
        };
        moves.push(Self::Act);
        moves
    }

    pub fn seq_to_string(seq: &[Self]) -> String {
        seq.iter()
            .map(|b| {
                let c: char = (*b).into();
                c
            })
            .collect()
    }

    fn seq_to_press(seq: &[Self], layers: usize) -> Vec<DirpadButton> {
        if layers == 0 {
            return seq.to_vec();
        }

        let mut cursor = Self::Act;
        let dirpad_buttons = seq
            .iter()
            .flat_map(|numpad_button| {
                let dir_buttons = Self::dir_buttons_to_press(&cursor, numpad_button);
                cursor = *numpad_button;
                dir_buttons
            })
            .collect::<Vec<_>>();

        Self::seq_to_press(&dirpad_buttons, layers - 1)
    }
}
