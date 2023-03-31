pub mod checkers;

fn main() {
    const BOARD_LENGTH: usize = 10;
    const TROOPS_SIZE: usize = 20;

    let mut board = checkers::Checkers::new(BOARD_LENGTH);
    board.initialize(TROOPS_SIZE);
    print!("{}", board.to_string());
}