use regex::Regex;

pub fn validate_whites_move_format(whites_moves: String) -> Result<(), ()> {
    let regex = Regex::new(r"^\d{1,2}-\d{1,2}$|^\d{1,2}(x\d{1,2}){1,}$").unwrap();
    match regex.is_match(&whites_moves) {
        true => return Ok(()),
        false => Err(()),
    }
    // TODO : fix
}
