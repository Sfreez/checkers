#[derive(PartialEq, Debug)]
pub struct Move {
    moves: Vec<u8>,
}

impl Move {
    fn new(positions: Vec<u8>) -> Result<Move, String> {
        if positions.len() < 2 {
            return Err(String::from("Invalid move"));
        }
        if !positions
            .windows(2)
            .all(|w| Self::are_on_the_same_diagonal(w[0].into(), w[1].into()))
        {
            return Err(String::from("Invalid move"));
        }
        return Ok(Move { moves: positions });
    }

    fn are_on_the_same_diagonal(position1: i16, position2: i16) -> bool {
        let line_position1 = (position1 - 1) / 5;
        let line_position2 = (position2 - 1) / 5;
        let column_position1 = ((position1 - 1) % 5 + 1) * 2 - line_position1 % 2;
        let column_position2 = ((position2 - 1) % 5 + 1) * 2 - line_position2 % 2;
        return (line_position1 - line_position2).abs()
            == (column_position1 - column_position2).abs();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn at_least_two_postions_are_needed_to_form_a_move() {
        assert_eq!(Err(String::from("Invalid move")), Move::new(vec![]));
        assert_eq!(Err(String::from("Invalid move")), Move::new(vec![1]));
    }

    #[test]
    fn two_postions_on_the_same_diagonal_can_form_a_move() {
        assert_eq!(
            Move {
                moves: vec![21, 43]
            },
            Move::new(vec![21, 43]).unwrap()
        );
    }

    #[test]
    fn two_postions_on_different_diagonals_cannot_form_a_move() {
        assert_eq!(Err(String::from("Invalid move")), Move::new(vec![18, 19]));
    }

    #[test]
    fn multiple_postions_with_consecutive_positions_on_the_same_diagonal_can_form_a_move() {
        assert_eq!(
            Move {
                moves: vec![10, 19, 8]
            },
            Move::new(vec![10, 19, 8]).unwrap()
        );
    }

    #[test]
    fn multiple_postions_with_consecutive_positions_on_different_diagonals_cannot_form_a_move() {
        assert_eq!(
            Err(String::from("Invalid move")),
            Move::new(vec![10, 19, 17])
        );
    }
}
