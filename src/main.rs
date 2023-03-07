use regex::Regex;

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

fn main() {
    print_board();
}

fn print_board() {
  let mut board = ["   ";50];
  for (i, el) in board.iter_mut().enumerate() {
    if i < 20 {
      *el = " B ";
    } else if i >= 30 {
      *el = " W ";
    }
  }
    let mut board_str = String::from(BOARD_WITH_PLACEHOLDERS);
    let regex = Regex::new(r"(\*)(\d{2})").unwrap();
    for captures in regex.captures_iter(BOARD_WITH_PLACEHOLDERS) {
        let index = captures[2].trim_start_matches('0').parse::<usize>().unwrap() - 1;
        board_str = board_str.replace(&captures[0], board.get(index).unwrap());
    }
    println!("{}", board_str);
}