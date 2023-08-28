use std::io;
mod tictactoe;

fn main() -> io::Result<()> {
    let mut game = tictactoe::TicTacToe::new();

    while !matches!(game.next_player(), tictactoe::Player::None) {
        let mut first_line = String::new();
        let mut second_line = String::new();

        println!("Next player: {}", game.next_player());

        println!("Enter x position:");
        io::stdin().read_line(&mut first_line)?;
        let x = first_line.get(0..1).unwrap().parse::<usize>().unwrap();

        println!("Enter y position:");
        io::stdin().read_line(&mut second_line)?;
        let y = second_line.get(0..1).unwrap().parse::<usize>().unwrap();

        game.play_move(x, y).unwrap();
        tictactoe::print_tictactoe_board(&game.clone_board());
    }

    println!("The winner is {}", game.winner());

    Ok(())
}
