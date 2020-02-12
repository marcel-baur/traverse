#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Figure {
    pub colour: Colour,
    pub crowned: bool,
}

impl Figure {
    pub fn create(colour: Colour) -> Figure {
        Figure {
            colour,
            crowned: false,
        }
    }
    pub fn crown(piece: Figure) -> Figure {
        Figure {
            colour: piece.colour,
            crowned: true,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Square(pub usize, pub usize);

impl Square {
    pub fn jump_targets(&self) -> Vec<Square> {
        let mut targets = vec![];
        let Square(x, y) = *self;
        if x < 6 && y > 1 {
            targets.push(Square(x + 2, y - 2));
        }
        if x < 6 && y < 6 {
            targets.push(Square(x + 2, y + 2));
        }

        if x > 1 && y > 1 {
            targets.push(Square(x - 2, y - 2));
        }
        if x > 1 && y < 6 {
            targets.push(Square(x - 2, y + 2));
        }
        targets
    }
    pub fn move_targets(&self) -> Vec<Square> {
        let mut targets = vec![];
        let Square(x, y) = *self;
        if x < 7 && y > 0 {
            targets.push(Square(x + 1, y - 1));
        }
        if x < 7 && y < 7 {
            targets.push(Square(x + 1, y + 1));
        }

        if x > 0 && y > 0 {
            targets.push(Square(x - 1, y - 1));
        }
        if x > 0 && y < 7 {
            targets.push(Square(x - 1, y + 1));
        }
        targets
    }
}
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Colour {
    Red,
    White,
}
