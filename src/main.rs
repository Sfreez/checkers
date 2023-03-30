fn main() {
    const BOARD_LENGTH: usize = 10;
    const TROOPS_SIZE: usize = 20;

    assert!(BOARD_LENGTH % 2 == 0);
    assert!(TROOPS_SIZE <= BOARD_LENGTH * BOARD_LENGTH / 4);
    assert!(TROOPS_SIZE % (BOARD_LENGTH / 2) == 0);

    let board = initialize_board(BOARD_LENGTH, TROOPS_SIZE);

    print_board(board, BOARD_LENGTH);
}

fn print_board(board: Vec<&str>, board_length: usize) {

    let line_separator = "-".repeat(board_length * 4 + 1);
    for line in 0..board_length {
        println!("{}", line_separator);
        for column in 0..board_length {
            if (line + column) % 2 == 1 {
                print!("| {} ", board[(line * board_length + column) / 2]);
            } else {
                print!("|   ");
            }
        }
        println!("|");
    }
    println!("{}", line_separator);
}

fn initialize_board(board_length: usize, troops_size: usize) -> Vec<&'static str> {
    let mut board = vec![" "; board_length*board_length/2];
    for index in 0..board_length*board_length/2 {
        if index < troops_size {
            board[index] = "W"
        } else if index >= board_length*board_length/2 - troops_size {
            board[index] = "B"
        }
    }
    board
}