use regex::Regex;

pub fn validate_whites_move_format(whites_move: String) -> Result<(), ()> {
    let regex = Regex::new(r"^\d{1,2}-\d{1,2}$|^\d{1,2}(x\d{1,2}){1,}$").unwrap();
    match regex.is_match(&whites_move) {
        true => return Ok(()),
        false => Err(()),
    }
    // TODO : fix
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valide_white_move() {
        let whites_move = String::from("12x7");

        assert!(validate_whites_move_format(whites_move).is_ok());
    }
}
