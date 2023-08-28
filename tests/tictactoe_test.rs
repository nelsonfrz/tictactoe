use tictactoe::tictactoe::{FieldState, TicTacToe};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initialization() {
        let game = TicTacToe::new();
        for row in game.clone_board().iter() {
            for &cell in row.iter() {
                assert_eq!(cell, FieldState::None);
            }
        }
        assert_eq!(game.next_player(), FieldState::Cross);
    }

    #[test]
    fn test_valid_move() {
        let mut game = TicTacToe::new();
        assert_eq!(game.play_move(0, 0), Ok(()));
        assert_eq!(game.next_player(), FieldState::Circle);
    }

    #[test]
    fn test_invalid_move() {
        let mut game = TicTacToe::new();
        game.play_move(0, 0).unwrap();
        assert_eq!(game.play_move(0, 0), Err("Move position already taken."));
        assert_eq!(game.play_move(3, 0), Err("Move position out of bounds."));
    }

    #[test]
    fn test_clone_board() {
        let game = TicTacToe::new();
        let cloned_board = game.clone_board();
        for (original_row, cloned_row) in game.clone_board().iter().zip(cloned_board.iter()) {
            for (&original_cell, &cloned_cell) in original_row.iter().zip(cloned_row.iter()) {
                assert_eq!(original_cell, cloned_cell);
            }
        }
    }

    #[test]
    fn test_winner_with_cross_row() {
        let mut game = TicTacToe::new();
        game.play_move(0, 0).unwrap();
        game.play_move(1, 0).unwrap();
        game.play_move(1, 1).unwrap();
        game.play_move(2, 0).unwrap();
        game.play_move(2, 2).unwrap();
        assert_eq!(game.winner(), FieldState::Cross);
    }

    #[test]
    fn test_winner_with_circle_row() {
        let mut game = TicTacToe::new();
        game.play_move(0, 0).unwrap();
        game.play_move(0, 1).unwrap();
        game.play_move(1, 0).unwrap();
        game.play_move(1, 1).unwrap();
        game.play_move(2, 2).unwrap();
        game.play_move(2, 1).unwrap();
        assert_eq!(game.winner(), FieldState::Circle);
    }

    #[test]
    fn test_winner_with_cross_column() {
        let mut game = TicTacToe::new();
        game.play_move(0, 0).unwrap();
        game.play_move(0, 1).unwrap();
        game.play_move(1, 0).unwrap();
        game.play_move(0, 2).unwrap();
        game.play_move(2, 0).unwrap();
        assert_eq!(game.winner(), FieldState::Cross);
    }

    #[test]
    fn test_winner_with_circle_column() {
        let mut game = TicTacToe::new();
        game.play_move(0, 0).unwrap();
        game.play_move(0, 1).unwrap();
        game.play_move(1, 0).unwrap();
        game.play_move(1, 1).unwrap();
        game.play_move(2, 2).unwrap();
        game.play_move(2, 1).unwrap();
        assert_eq!(game.winner(), FieldState::Circle);
    }

    #[test]
    fn test_winner_with_cross_diagonal() {
        let mut game = TicTacToe::new();
        game.play_move(0, 0).unwrap();
        game.play_move(0, 1).unwrap();
        game.play_move(1, 1).unwrap();
        game.play_move(0, 2).unwrap();
        game.play_move(2, 2).unwrap();
        assert_eq!(game.winner(), FieldState::Cross);
    }

    #[test]
    fn test_winner_with_circle_diagonal() {
        let mut game = TicTacToe::new();
        game.play_move(0, 0).unwrap();
        game.play_move(0, 2).unwrap();
        game.play_move(1, 0).unwrap();
        game.play_move(1, 1).unwrap();
        game.play_move(0, 1).unwrap();
        game.play_move(2, 0).unwrap();
        assert_eq!(game.winner(), FieldState::Circle);
    }
}
