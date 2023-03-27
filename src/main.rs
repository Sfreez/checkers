fn main() {
    const BOARD_LENGTH: usize = 10;
    const PIECES_NUMBER: usize = 20;

    let mut board = [" "; BOARD_LENGTH*BOARD_LENGTH/2];
    for index in 0..BOARD_LENGTH*BOARD_LENGTH/2 {
        if index < PIECES_NUMBER {
            board[index] = "W"
        } else if index >= BOARD_LENGTH*BOARD_LENGTH/2 - PIECES_NUMBER {
            board[index] = "B"
        }
    }

    for line in 0..BOARD_LENGTH {
        println!("-----------------------------------------");
        for column in 0..BOARD_LENGTH {
            if (line + column) % 2 == 1 {
                print!("| {} ", board[(line * BOARD_LENGTH + column) / 2]);
            } else {
                print!("|   ");
            }
        }
        print!("|");
        println!();
    }
    println!("-----------------------------------------");
}