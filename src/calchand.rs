use super::dice;

fn sum_faces(hand: &dice::Dice, face: dice::DieFace) -> i16 {
    let sum = hand
        .dice
        .iter()
        .filter(|x| **x == face)
        .sum::<dice::DieFace>();
    sum as i16
}

fn sort_faces(hand: &dice::Dice) -> Vec<usize> {
    let range = 1..dice::Dice::NUMBER_OF_DICE + 2;

    let r: Vec<usize> = range
        .map(|face| {
            let face_count: Vec<&i8> = hand
                .dice
                .iter()
                .filter(|f| **f == face as dice::DieFace)
                .collect();
            face_count.len()
        })
        .collect();
    r
}

pub fn calc_ace(hand: &dice::Dice) -> i16 {
    sum_faces(hand, 1)
}

pub fn calc_two(hand: &dice::Dice) -> i16 {
    sum_faces(hand, 2)
}

pub fn calc_three(hand: &dice::Dice) -> i16 {
    sum_faces(hand, 3)
}

pub fn calc_four(hand: &dice::Dice) -> i16 {
    sum_faces(hand, 4)
}

pub fn calc_five(hand: &dice::Dice) -> i16 {
    sum_faces(hand, 5)
}

pub fn calc_six(hand: &dice::Dice) -> i16 {
    sum_faces(hand, 6)
}

pub fn calc_3k(hand: &dice::Dice) -> i16 {
    let faces_count = sort_faces(hand);
    println!("hand {:?}", hand);

    println!("faces_count {:?}", faces_count);
    let tmp: Vec<&usize> = faces_count.iter().filter(|f| **f >= 3).collect();
    println!("tmp {:?}", tmp);
    let sum: i16 = hand.dice.iter().map(|f| *f as i16).sum();
    println!("sum {:?}", sum);

    let test = tmp.len() >= 1;
    println!("test {:?}", test);

    let val = if test { sum } else { 0 };
    println!("val {:?}", val);
    val
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
    fn test_3k_five_aces() {
        let test_dice: Vec<dice::DieFace> = vec![1, 1, 1, 1, 1];
        let hand = dice::Dice::roll_fake(test_dice);

        let scorecard = scorecard::get_new_scorecard_data();
        let score = (scorecard.three_kind.calc)(&hand);
        assert_eq!(score, 5);
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
