use crate::board::{Colour, Figure, Square};
use core::fmt;
use std::convert::TryFrom;

#[derive(PartialEq, Debug)]
pub struct Move {
    pub from: Square,
    pub to: Square,
}

type GameBoard = [[Option<Figure>; 8]; 8];

pub struct GameState {
    board: GameBoard,
    turn: Colour,
    current_move: u16,
    move_log: Vec<Move>,
}

impl GameState {
    pub fn create() -> GameState {
        let mut game = GameState {
            board: [[None; 8]; 8],
            current_move: 0,
            turn: Colour::White,
            move_log: Vec::new(),
        };
        game.populate_field();
        game
    }

    fn populate_field(&mut self) {
        let mut vec_white = vec![];
        let mut vec_red = vec![];
        for j in 0..3 {
            for i in 0..8 {
                if i % 2 == j % 2 {
                    vec_white.push((j, i));
                } else {
                    vec_red.push(( 7 - j,i))
                }
            }
        }
        dbg!(&vec_white);
        for (x, y) in vec_white {
            self.board[x][y] = Some(Figure::create(Colour::White));
        }
        for (x, y) in vec_red {
            self.board[x][y] = Some(Figure::create(Colour::Red));
        }
    }

    pub fn handle_move(&mut self, game_move: Move) -> Result<(), String> {
        let allowed_moves = get_legal_moves(&self.board, &self.turn);
        if !allowed_moves.contains(&game_move) {
            dbg!(&allowed_moves);
            return Err("Move not allowed!".to_string());
        };
        let Square(fx, fy) = game_move.from;
        let Square(tx, ty) = game_move.to;
        if let Some(_t) = self.board[tx][ty] {
            return Err("The target field already has a figure on it!".to_string())
        };
        let piece = match self.board[fx][fy] {
            Some(piece) => piece,
            None => return Err("There is no piece on this field!".to_string()),
        };
        if to_i8(fx) - to_i8(fx) != 1 {
            let x_m: i8 = to_i8(fx) - (to_i8(fx) - to_i8(fy)) / 2;
            let y_m: i8 = to_i8(fy) - (to_i8(fy) - to_i8(ty)) / 2;
            self.board[usize::try_from(x_m).unwrap()][usize::try_from(y_m).unwrap()];
        }

        self.board[tx][ty] = Some(piece);
        self.board[fx][fy] = None;

        self.current_move += 1;
        if (ty == 0 && piece.colour == Colour::Red) || (ty == 7 && piece.colour == Colour::White) {
            self.board[tx][ty].unwrap().crowned = true;
            true
        } else {
            false
        };
        self.current_move += 1;
        if self.turn == Colour::White {
            self.turn = Colour::Red;
        } else {
            self.turn = Colour::White;
        }
        self.move_log.push(game_move);
        Ok(())
    }
}

impl fmt::Display for GameState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut horizontal_limiter = String::from("+");
        for _ in 0..self.board.len() {
            horizontal_limiter.push('-');
            horizontal_limiter.push('-');
        }
        horizontal_limiter.push_str("+\n");
        write!(f, "{}", horizontal_limiter)?;
        for row in self.board.iter() {
            write!(f, "|")?;
            for field in row.iter() {
                match field {
                    Some(figure) => match figure.colour {
                        Colour::White => {
                            write!(f, "x ")?;
                        }
                        Colour::Red => {
                            write!(f, "o ")?;
                        }
                    },
                    None => {
                        write!(f, "  ")?;
                    }
                }
            }
            write!(f, "|")?;
            writeln!(f)?;
        }
        write!(f, "{}", horizontal_limiter)
    }
}

fn get_crown(piece: Figure, square: Square) -> bool {
    (square.1 == 0 && piece.colour == Colour::Red)
        || (square.1 == 7 && piece.colour == Colour::White)
}

fn to_i8(i: usize) -> i8 {
    i8::try_from(i).unwrap()
}

fn get_legal_moves(board: &GameBoard, turn: &Colour) -> Vec<Move> {
    let mut moves: Vec<Move> = vec![];
    for col in 0..8 {
        for row in 0..8 {
            if let Some(piece) = board[col][row] {
                if piece.colour == *turn {
                    moves.append(&mut moves_from_field(board, Square(col, row)))
                }
            }
        }
    }
    moves
}

fn moves_from_field(board: &GameBoard, field: Square) -> Vec<Move> {
    if let Some(piece) = board[field.0][field.1] {
        let mut jumps = field
            .jump_targets()
            .into_iter()
            .filter(|j| legal_jump(board, &piece, &field, &j))
            .map(|ref j| Move {
                from: field,
                to: *j,
            })
            .collect::<Vec<Move>>();
        let mut regular_moves = field
            .move_targets()
            .into_iter()
            .filter(|m| legal_move(board, &piece, &field, &m))
            .map(|ref m| Move {
                from: field,
                to: *m,
            })
            .collect::<Vec<Move>>();
        jumps.append(&mut regular_moves);
        jumps
    } else {
        vec![]
    }
}

fn jumped_figure(board: &GameBoard, from: &Square, to: &Square) -> Option<Figure> {
    let Square(fx, fy) = *from;
    let Square(tx, ty) = *to;
    let x_m: i8 = to_i8(fx) - (to_i8(fx) - to_i8(tx)) / 2;
    let y_m: i8 = to_i8(fy) - (to_i8(fy) - to_i8(ty)) / 2;
    board[usize::try_from(x_m).unwrap()][usize::try_from(y_m).unwrap()]
}

fn legal_jump(board: &GameBoard, piece: &Figure, from: &Square, to: &Square) -> bool {
    match jumped_figure(board, from, to) {
        Some(p) if p.colour != piece.colour => true,
        _ => false,
    }
}

fn legal_move(board: &GameBoard, piece: &Figure, from: &Square, to: &Square) -> bool {
    let Square(tx, ty) = *to;
    if let Some(_piece) = board[tx][ty] {
        false
    } else {
        let Square(_fx, fy) = *from;
        ty > fy && piece.colour == Colour::Red && piece.crowned
            || piece.colour == Colour::White && ty < fy && piece.crowned
            || ty < fy && piece.colour == Colour::Red
            || ty > fy && piece.colour == Colour::White
    }
}

#[cfg(test)]
mod test {
    use super::super::board::Figure;
    use super::*;

    #[test]
    fn move_targets() {
        let mut game = GameState::create();
        let field1 = Square(3, 0);
        let targets = field1.move_targets();
        assert_eq!(targets, [Square(4, 1), Square(2, 1)]);

        let field2 = Square(5, 5);
        let targets2 = field2.move_targets();
        assert_eq!(
            targets2,
            [Square(6, 4), Square(6, 6), Square(4, 4), Square(4, 6)]
        );
    }
}
