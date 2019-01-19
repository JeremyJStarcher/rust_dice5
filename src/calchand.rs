use super::dice;

struct DiceSort {

}

fn sum_faces(dice: &dice::Dice, face: dice::DieFace) -> i16 {
    let sum = dice.dice.iter().filter(|x| **x == face).sum::<dice::DieFace>();
    sum as i16
}

fn sort_faces(dice: &dice::Dice, face: dice::DieFace) -> i16 {
    let sum = dice.dice.iter().filter(|x| **x == face).sum::<dice::DieFace>();
    sum as i16
}
pub fn calc_ace(dice: &dice::Dice) -> i16 {
    sum_faces(dice, 1)
}

pub fn calc_two(dice: &dice::Dice) -> i16 {
    sum_faces(dice, 2)
}

pub fn calc_three(dice: &dice::Dice) -> i16 {
    sum_faces(dice, 3)
}

pub fn calc_four(dice: &dice::Dice) -> i16 {
    sum_faces(dice, 4)
}

pub fn calc_five(dice: &dice::Dice) -> i16 {
    sum_faces(dice, 5)
}

pub fn calc_six(dice: &dice::Dice) -> i16 {
    sum_faces(dice, 6)
}

#[cfg(test)]
mod tests {
    use super::super::scorecard;
    use super::*;

    #[test]
    fn is_test() {
        assert_eq!(true, true);
    }

    #[test]
    fn test_ace_all_aces() {
        let test_dice: Vec<dice::DieFace> = vec![1, 1, 1, 1, 1];
        let hand = dice::Dice::roll_fake(test_dice);

        let scorecard = scorecard::get_new_scorecard_data();
        let score = (scorecard.ace.calc)(&hand);
        assert_eq!(score, 1 * 5);
    }
    #[test]
    fn test_two_all_twos() {
        let test_dice: Vec<dice::DieFace> = vec![2, 2, 2, 2, 2];
        let hand = dice::Dice::roll_fake(test_dice);

        let scorecard = scorecard::get_new_scorecard_data();
        let score = (scorecard.two.calc)(&hand);
        assert_eq!(score, 2 * 5);
    }

    #[test]
    fn test_thre_all_threes() {
        let test_dice: Vec<dice::DieFace> = vec![3, 3, 3, 3, 3];
        let hand = dice::Dice::roll_fake(test_dice);

        let scorecard = scorecard::get_new_scorecard_data();
        let score = (scorecard.three.calc)(&hand);
        assert_eq!(score, 3 * 5);
    }

    #[test]
    fn test_four_all_fours() {
        let test_dice: Vec<dice::DieFace> = vec![4, 4, 4, 4, 4];
        let hand = dice::Dice::roll_fake(test_dice);

        let scorecard = scorecard::get_new_scorecard_data();
        let score = (scorecard.four.calc)(&hand);
        assert_eq!(score, 4 * 5);
    }

    #[test]
    fn test_fives_all_aces() {
        let test_dice: Vec<dice::DieFace> = vec![5, 5, 5, 5, 5];
        let hand = dice::Dice::roll_fake(test_dice);

        let scorecard = scorecard::get_new_scorecard_data();
        let score = (scorecard.five.calc)(&hand);
        assert_eq!(score, 5 * 5);
    }

    #[test]
    fn test_six_all_sixes() {
        let test_dice: Vec<dice::DieFace> = vec![6, 6, 6, 6, 6];
        let hand = dice::Dice::roll_fake(test_dice);

        let scorecard = scorecard::get_new_scorecard_data();
        let score = (scorecard.six.calc)(&hand);
        assert_eq!(score, 6 * 5);
    }

    #[test]
    fn test_3k_three_sixes() {
        let test_dice: Vec<dice::DieFace> = vec![6, 6, 6, 1, 2];
        let hand = dice::Dice::roll_fake(test_dice);

        let scorecard = scorecard::get_new_scorecard_data();
        let score = (scorecard.three_kind.calc)(&hand);
        assert_eq!(score, (6 * 3) + 1 + 2);
    }

    #[test]
    fn test_aces_all_zeros() {
        let test_dice: Vec<dice::DieFace> = vec![0, 0, 0, 0, 0];
        let hand = dice::Dice::roll_fake(test_dice);

        let scorecard = scorecard::get_new_scorecard_data();
        let score = (scorecard.ace.calc)(&hand);
        assert_eq!(score, 0);
    }
}
