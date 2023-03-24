use regex::Regex;
use std::io;

pub mod checkers;
pub mod checkers_move;

const BOARD_WITH_PLACEHOLDERS: &str = "
        1       2       3       4       5   
  -----------------------------------------
  |   |*01|   |*02|   |*03|   |*04|   |*05| 5
  -----------------------------------------
6 |*06|   |*07|   |*08|   |*09|   |*10|   |
  -----------------------------------------
  |   |*11|   |*12|   |*13|   |*14|   |*15|15
  -----------------------------------------
16|*16|   |*17|   |*18|   |*19|   |*20|   |
  -----------------------------------------
  |   |*21|   |*22|   |*23|   |*24|   |*25|25
  -----------------------------------------
26|*26|   |*27|   |*28|   |*29|   |*30|   |
  -----------------------------------------
  |   |*31|   |*32|   |*33|   |*34|   |*35|35
  -----------------------------------------
36|*36|   |*37|   |*38|   |*39|   |*40|   |
  -----------------------------------------
  |   |*41|   |*42|   |*43|   |*44|   |*45|45
  -----------------------------------------
46|*46|   |*47|   |*48|   |*49|   |*50|   |
  -----------------------------------------
    46      47      48      49      50";

#[derive(Clone, Copy)]
enum CaseValue {
    EMPTY,
    BLACK,
    WHITE,
}

fn main() {
    let board = initialize_board();
    print_board(board);
    println!("Whites to play :");
    let mut player_input = String::new();
    
    match io::stdin().read_line(&mut player_input) {
        Ok(_) => {
            player_input = player_input.trim().to_string();
            match checkers::WhitesMove::from(&player_input) {
                Ok(_) => (),
                Err(_) => println!("Invalid move format.")
            }
        }
        Err(error) => println!("error: {error}"),
    }
}

fn initialize_board() -> [CaseValue; 50] {
    let mut board: [CaseValue; 50] = [CaseValue::EMPTY; 50];
    for (index, case_value) in board.iter_mut().enumerate() {
        if index < 20 {
            *case_value = CaseValue::BLACK;
        } else if index >= 30 {
            *case_value = CaseValue::WHITE;
        } else {
            *case_value = CaseValue::EMPTY;
        }
    }
    board
}

fn print_board(board: [CaseValue; 50]) {
    let mut board_str = String::from(BOARD_WITH_PLACEHOLDERS);
    let regex = Regex::new(r"(\*)(\d{2})").unwrap();
    for captures in regex.captures_iter(BOARD_WITH_PLACEHOLDERS) {
        let index = captures[2]
            .trim_start_matches('0')
            .parse::<usize>()
            .unwrap()
            - 1;
        match board.get(index).unwrap() {
            CaseValue::BLACK => board_str = board_str.replace(&captures[0], " B "),
            CaseValue::WHITE => board_str = board_str.replace(&captures[0], " W "),
            CaseValue::EMPTY => board_str = board_str.replace(&captures[0], "   "),
        }
    }
    println!("{}", board_str);
}
