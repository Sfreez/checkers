use regex::Regex;

pub struct WhitesMove {
    captures: bool,
    moves: Vec<u8>
}

impl WhitesMove {

    fn new(captures: bool, moves: Vec<u8>) -> WhitesMove {
        WhitesMove {captures, moves}
    }

    pub fn from(whites_move: &str) -> Result<WhitesMove, ()> {
        let move_regex = Regex::new(r"^\d{1,2}-\d{1,2}$").unwrap();
        let captures_regex = Regex::new(r"^\d{1,2}(x\d{1,2}){1,}$").unwrap();
        if move_regex.is_match(&whites_move) {
            return Ok(WhitesMove::new(
                false,
                whites_move.split("-").map(str::parse::<u8>).collect::<Result<Vec<u8>, _>>().unwrap()))
        } else if captures_regex.is_match(&whites_move) {
            
            return Ok(WhitesMove::new(
                true,
                whites_move.split("x").map(str::parse::<u8>).collect::<Result<Vec<u8>, _>>().unwrap()))
        } else {
            return Err(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_white_move() {
        assert!(WhitesMove::from("12x7").is_ok());
    }
}
