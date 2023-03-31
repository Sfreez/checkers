pub struct Checkers<'a> {
    board_length: usize,
    board: Vec<&'a str>
}

impl Checkers<'_> {
    pub fn new(board_length: usize) -> Checkers<'static>{
        assert!(board_length % 2 == 0);
        Checkers{board_length, board: vec![" "; board_length*board_length/2]}
    }

    pub fn initialize(&mut self, troops_size: usize) {
        assert!(troops_size <= self.board_length * self.board_length / 4);
        assert!(troops_size % (self.board_length / 2) == 0);

        for index in 0..self.board_length*self.board_length/2 {
            if index < troops_size {
                self.board[index] = "W";
            } else if index >= self.board_length*self.board_length/2 - troops_size {
                self.board[index] = "B";
            }
        }
    }

    pub fn to_string(&self) -> String {
        let mut board_string = String::new();
        let line_separator = format!("{}\n", "-".repeat(self.board_length * 4 + 1));
        for line in 0..self.board_length {
            board_string.push_str(&line_separator);
            for column in 0..self.board_length {
                board_string.push_str("| ");
                if (line + column) % 2 == 1 {
                    board_string.push_str(self.board[(line * self.board_length + column) / 2]);
                    board_string.push_str(" ");
                } else {
                    board_string.push_str("  ");
                }
            }
            board_string.push_str("|\n");
        }
        board_string.push_str(&line_separator);
        board_string
    }
}