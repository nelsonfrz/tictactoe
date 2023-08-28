#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FieldState {
    None,
    Cross,
    Circle,
}

impl std::fmt::Display for FieldState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                FieldState::Circle => "O",
                FieldState::Cross => "X",
                FieldState::None => " ",
            }
        )
    }
}

const BOARD_SIZE: usize = 3;
pub type Board = [[FieldState; BOARD_SIZE]; BOARD_SIZE];
pub type Player = FieldState;

pub struct TicTacToe {
    board: Board,
    next_field_state: FieldState,
}

impl TicTacToe {
    pub fn new() -> Self {
        TicTacToe {
            board: [[FieldState::None; BOARD_SIZE]; BOARD_SIZE],
            next_field_state: FieldState::Cross,
        }
    }

    pub fn next_player(&self) -> Player {
        self.next_field_state
    }

    pub fn play_move(&mut self, x: usize, y: usize) -> Result<(), &'static str> {
        if x >= BOARD_SIZE || y >= BOARD_SIZE {
            return Err("Move position out of bounds.");
        }

        let field = &self.board[y][x];
        match field {
            FieldState::None => {
                self.board[y][x] = self.next_player();
                self.next_field_state = if let FieldState::None = self.winner() {
                    match self.next_field_state {
                        FieldState::Circle => FieldState::Cross,
                        FieldState::Cross => FieldState::Circle,
                        FieldState::None => FieldState::None,
                    }
                } else {
                    FieldState::None
                };
                Ok(())
            }
            _ => Err("Move position already taken."),
        }
    }

    pub fn clone_board(&self) -> Board {
        self.board
    }

    pub fn winner(&self) -> FieldState {
        let check_winner = |field: FieldState| -> bool {
            self.board
                .iter()
                .any(|row| row.iter().all(|&cell| cell == field))
                || (0..BOARD_SIZE).any(|i| self.board.iter().all(|row| row[i] == field))
                || self
                    .board
                    .iter()
                    .enumerate()
                    .all(|(i, row)| row[i] == field)
                || self
                    .board
                    .iter()
                    .enumerate()
                    .all(|(i, row)| row[BOARD_SIZE - 1 - i] == field)
        };

        if check_winner(FieldState::Circle) {
            FieldState::Circle
        } else if check_winner(FieldState::Cross) {
            FieldState::Cross
        } else {
            FieldState::None
        }
    }
}

pub fn print_tictactoe_board(board: &Board) {
    for row in board.iter() {
        for (i, field) in row.iter().enumerate() {
            print!("|{}", field);
            if i == BOARD_SIZE - 1 {
                print!("|");
            }
        }
        println!();
    }
    println!();
}
