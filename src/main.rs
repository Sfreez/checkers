fn main() {
    const BOARD_LENGTH: usize = 10;
    const PIECES_NUMBER: usize = 20;

    assert!(BOARD_LENGTH % 2 == 0);
    assert!(PIECES_NUMBER <= BOARD_LENGTH * BOARD_LENGTH / 4);
    assert!(PIECES_NUMBER % (BOARD_LENGTH / 2) == 0);

    let mut board = [" "; BOARD_LENGTH*BOARD_LENGTH/2];
    for index in 0..BOARD_LENGTH*BOARD_LENGTH/2 {
        if index < PIECES_NUMBER {
            board[index] = "W"
        } else if index >= BOARD_LENGTH*BOARD_LENGTH/2 - PIECES_NUMBER {
            board[index] = "B"
        }
    }

    let line_separator = "-".repeat(BOARD_LENGTH * 4 + 1);
    for line in 0..BOARD_LENGTH {
        println!("{}", line_separator);
        for column in 0..BOARD_LENGTH {
            if (line + column) % 2 == 1 {
                print!("| {} ", board[(line * BOARD_LENGTH + column) / 2]);
            } else {
                print!("|   ");
            }
        }
        println!("|");
    }
    println!("{}", line_separator);
}