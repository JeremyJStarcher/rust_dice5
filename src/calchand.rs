use super::hand;
use super::hand::Dice;
use super::hand::DieFace;

const VALUE_SMALL_STRAIGHT: i16 = 30;
const VALUE_LARGE_STRAIGHT: i16 = 40;
const VALUE_FULL_HOUSE: i16 = 25;
const VALUE_DICE5: i16 = 50;

fn sum_faces(hand: &Dice, face: DieFace) -> i16 {
    let sum = hand.dice.iter().filter(|x| **x == face).sum::<DieFace>();
    sum as i16
}

fn sort_faces(hand: &Dice) -> Vec<usize> {
    let range = 1..hand::Dice::NUMBER_OF_DICE + 2;

    let r: Vec<usize> = range
        .map(|face| {
            let face_count: Vec<&i8> = hand
                .dice
                .iter()
                .filter(|f| **f == face as DieFace)
                .collect();
            face_count.len()
        })
        .collect();
    r
}

pub fn hand_to_string(hand: &Dice) -> String {
    let faces_count = sort_faces(hand);
    let piles_of_at_least_one: Vec<String> = faces_count
        .iter()
        .map(|f| match *f >= 1 {
            true => "+".to_string(),
            false => "-".to_string(),
        })
        .collect();

    piles_of_at_least_one.join("")
}

fn sum_all_dice(hand: &Dice) -> i16 {
    hand.dice.iter().map(|f| *f as i16).sum()
}

pub fn is_dice5(hand: &Dice) -> bool {
    let faces_count = sort_faces(hand);
    let piles_of_at_least_five: Vec<&usize> = faces_count.iter().filter(|f| **f >= 5).collect();
    piles_of_at_least_five.len() >= 1
}

pub fn calc_ace(hand: &Dice, _special_dice5: bool) -> i16 {
    sum_faces(hand, 1)
}

pub fn calc_two(hand: &Dice, _special_dice5: bool) -> i16 {
    sum_faces(hand, 2)
}

pub fn calc_three(hand: &Dice, _special_dice5: bool) -> i16 {
    sum_faces(hand, 3)
}

pub fn calc_four(hand: &Dice, _special_dice5: bool) -> i16 {
    sum_faces(hand, 4)
}

pub fn calc_five(hand: &Dice, _special_dice5: bool) -> i16 {
    sum_faces(hand, 5)
}

pub fn calc_six(hand: &Dice, _special_dice5: bool) -> i16 {
    sum_faces(hand, 6)
}

pub fn calc_3k(hand: &Dice, _special_dice5: bool) -> i16 {
    let faces_count = sort_faces(hand);
    let piles_of_at_least_three: Vec<&usize> = faces_count.iter().filter(|f| **f >= 3).collect();
    match piles_of_at_least_three.len() >= 1 {
        true => sum_all_dice(hand),
        false => 0,
    }
}

pub fn calc_4k(hand: &Dice, _special_dice5: bool) -> i16 {
    let faces_count = sort_faces(hand);
    let piles_of_at_least_four: Vec<&usize> = faces_count.iter().filter(|f| **f >= 4).collect();
    match piles_of_at_least_four.len() >= 1 {
        true => sum_all_dice(hand),
        false => 0,
    }
}

pub fn calc_ss(hand: &Dice, special_dice5: bool) -> i16 {
    if special_dice5 {
        if is_dice5(hand) {
            return VALUE_SMALL_STRAIGHT;
        } else {
            panic!("Illegal call");
        }
    }

    let str = hand_to_string(hand);

    match str.contains("++++") {
        true => VALUE_SMALL_STRAIGHT,
        false => 0,
    }
}

pub fn calc_ls(hand: &Dice, special_dice5: bool) -> i16 {
    if special_dice5 {
        if is_dice5(hand) {
            return VALUE_LARGE_STRAIGHT;
        } else {
            panic!("Illegal call");
        }
    }

    let str = hand_to_string(hand);

    match str.contains("+++++") {
        true => VALUE_LARGE_STRAIGHT,
        false => 0,
    }
}

pub fn calc_dice5(hand: &Dice, _special_dice5: bool) -> i16 {
    match is_dice5(hand) {
        true => VALUE_DICE5,
        false => 0,
    }
}

pub fn calc_fh(hand: &Dice, special_dice5: bool) -> i16 {
    if special_dice5 {
        if is_dice5(hand) {
            return VALUE_FULL_HOUSE;
        } else {
            panic!("Illegal call");
        }
    }

    let faces_count = sort_faces(hand);
    let piles_of_at_exactly_3: Vec<&usize> = faces_count.iter().filter(|f| **f == 3).collect();
    let piles_of_at_exactly_2: Vec<&usize> = faces_count.iter().filter(|f| **f == 2).collect();

    match piles_of_at_exactly_3.len() >= 1 && piles_of_at_exactly_2.len() >= 1 {
        true => VALUE_FULL_HOUSE,
        false => 0,
    }
}

pub fn calc_chance(hand: &Dice, _special_dice5: bool) -> i16 {
    sum_all_dice(hand)
}

#[cfg(test)]
mod tests {
    use super::super::scorecard;
    use super::*;
    use scorecard::LineId as L;

    #[test]
    fn test_ace_all_aces() {
        let test_dice: Vec<DieFace> = vec![1, 1, 1, 1, 1];
        let hand = Dice::roll_fake(test_dice);

        let scorecard = scorecard::get_new_scorecard_data();
        let score = (scorecard.get_line_by_id(L::Ace).calc)(&hand, false);
        assert_eq!(score, 1 * 5);
    }

    #[test]
    fn test_ace_two_aces() {
        let test_dice: Vec<DieFace> = vec![1, 1, 2, 3, 2];
        let hand = Dice::roll_fake(test_dice);

        let scorecard = scorecard::get_new_scorecard_data();
        let score = (scorecard.get_line_by_id(L::Ace).calc)(&hand, false);
        assert_eq!(score, 1 * 2);
    }

    #[test]
    fn test_two_all_twos() {
        let test_dice: Vec<DieFace> = vec![2, 2, 2, 2, 2];
        let hand = Dice::roll_fake(test_dice);

        let scorecard = scorecard::get_new_scorecard_data();
        let score = (scorecard.get_line_by_id(L::Two).calc)(&hand, false);
        assert_eq!(score, 2 * 5);
    }

    #[test]
    fn test_two_two_twos() {
        let test_dice: Vec<DieFace> = vec![2, 2, 3, 4, 5];
        let hand = Dice::roll_fake(test_dice);

        let scorecard = scorecard::get_new_scorecard_data();
        let score = (scorecard.get_line_by_id(L::Two).calc)(&hand, false);
        assert_eq!(score, 2 * 2);
    }

    #[test]
    fn test_three_all_threes() {
        let test_dice: Vec<DieFace> = vec![3, 3, 3, 3, 3];
        let hand = Dice::roll_fake(test_dice);

        let scorecard = scorecard::get_new_scorecard_data();
        let score = (scorecard.get_line_by_id(L::Three).calc)(&hand, false);
        assert_eq!(score, 3 * 5);
    }

    #[test]
    fn test_three_four_threes() {
        let test_dice: Vec<DieFace> = vec![3, 3, 3, 3, 1];
        let hand = Dice::roll_fake(test_dice);

        let scorecard = scorecard::get_new_scorecard_data();
        let score = (scorecard.get_line_by_id(L::Three).calc)(&hand, false);
        assert_eq!(score, 3 * 4);
    }

    #[test]
    fn test_four_all_fours() {
        let test_dice: Vec<DieFace> = vec![4, 4, 4, 4, 4];
        let hand = Dice::roll_fake(test_dice);

        let scorecard = scorecard::get_new_scorecard_data();
        let score = (scorecard.get_line_by_id(L::Four).calc)(&hand, false);
        assert_eq!(score, 4 * 5);
    }

    #[test]
    fn test_four_zero_fours() {
        let test_dice: Vec<DieFace> = vec![1, 2, 3, 5, 5];
        let hand = Dice::roll_fake(test_dice);

        let scorecard = scorecard::get_new_scorecard_data();
        let score = (scorecard.get_line_by_id(L::Four).calc)(&hand, false);
        assert_eq!(score, 4 * 0);
    }

    #[test]
    fn test_fives_all_aces() {
        let test_dice: Vec<DieFace> = vec![5, 5, 5, 5, 5];
        let hand = Dice::roll_fake(test_dice);

        let scorecard = scorecard::get_new_scorecard_data();
        let score = (scorecard.get_line_by_id(L::Five).calc)(&hand, false);
        assert_eq!(score, 5 * 5);
    }

    #[test]
    fn test_six_all_sixes() {
        let test_dice: Vec<DieFace> = vec![6, 6, 6, 6, 6];
        let hand = Dice::roll_fake(test_dice);

        let scorecard = scorecard::get_new_scorecard_data();
        let score = (scorecard.get_line_by_id(L::Six).calc)(&hand, false);
        assert_eq!(score, 6 * 5);
    }

    #[test]
    fn test_3k_three_sixes() {
        let test_dice: Vec<DieFace> = vec![6, 6, 6, 1, 2];
        let hand = Dice::roll_fake(test_dice);

        let scorecard = scorecard::get_new_scorecard_data();
        let score = (scorecard.get_line_by_id(L::ThreeKind).calc)(&hand, false);
        assert_eq!(score, (6 * 3) + 1 + 2);
    }

    #[test]
    fn test_3k_five_aces() {
        let test_dice: Vec<DieFace> = vec![1, 1, 1, 1, 1];
        let hand = Dice::roll_fake(test_dice);

        let scorecard = scorecard::get_new_scorecard_data();
        let score = (scorecard.get_line_by_id(L::ThreeKind).calc)(&hand, false);
        assert_eq!(score, 5);
    }

    #[test]
    fn test_3k_four_threes() {
        let test_dice: Vec<DieFace> = vec![3, 3, 3, 3, 1];
        let hand = Dice::roll_fake(test_dice);

        let scorecard = scorecard::get_new_scorecard_data();
        let score = (scorecard.get_line_by_id(L::ThreeKind).calc)(&hand, false);
        assert_eq!(score, 3 * 4 + 1);
    }

    #[test]
    fn test_3k_no_3k() {
        let test_dice: Vec<DieFace> = vec![1, 1, 2, 2, 3];
        let hand = Dice::roll_fake(test_dice);

        let scorecard = scorecard::get_new_scorecard_data();
        let score = (scorecard.get_line_by_id(L::ThreeKind).calc)(&hand, false);
        assert_eq!(score, 0);
    }

    #[test]
    fn test_4k_four_sixes() {
        let test_dice: Vec<DieFace> = vec![6, 6, 6, 6, 2];
        let hand = Dice::roll_fake(test_dice);

        let scorecard = scorecard::get_new_scorecard_data();
        let score = (scorecard.get_line_by_id(L::FourKind).calc)(&hand, false);
        assert_eq!(score, (6 * 4) + 2);
    }

    #[test]
    fn test_4k_five_aces() {
        let test_dice: Vec<DieFace> = vec![1, 1, 1, 1, 1];
        let hand = Dice::roll_fake(test_dice);

        let scorecard = scorecard::get_new_scorecard_data();
        let score = (scorecard.get_line_by_id(L::FourKind).calc)(&hand, false);
        assert_eq!(score, 5);
    }

    #[test]
    fn test_4k_no_4k() {
        let test_dice: Vec<DieFace> = vec![1, 1, 1, 2, 2];
        let hand = Dice::roll_fake(test_dice);

        let scorecard = scorecard::get_new_scorecard_data();
        let score = (scorecard.get_line_by_id(L::FourKind).calc)(&hand, false);
        assert_eq!(score, 0);
    }

    #[test]
    fn test_small_straight_low_straight() {
        let test_dice: Vec<DieFace> = vec![1, 4, 3, 2, 1];
        let hand = Dice::roll_fake(test_dice);

        let scorecard = scorecard::get_new_scorecard_data();
        let score = (scorecard.get_line_by_id(L::SmallStraight).calc)(&hand, false);
        assert_eq!(score, VALUE_SMALL_STRAIGHT);
    }

    #[test]
    fn test_small_straight_mid_straight() {
        let test_dice: Vec<DieFace> = vec![2, 4, 3, 2, 5];
        let hand = Dice::roll_fake(test_dice);

        let scorecard = scorecard::get_new_scorecard_data();
        let score = (scorecard.get_line_by_id(L::SmallStraight).calc)(&hand, false);
        assert_eq!(score, VALUE_SMALL_STRAIGHT);
    }

    #[test]
    fn test_small_straight_high_straight() {
        let test_dice: Vec<DieFace> = vec![6, 3, 4, 6, 5];
        let hand = Dice::roll_fake(test_dice);

        let scorecard = scorecard::get_new_scorecard_data();
        let score = (scorecard.get_line_by_id(L::SmallStraight).calc)(&hand, false);
        assert_eq!(score, VALUE_SMALL_STRAIGHT);
    }

    #[test]
    fn test_small_straight_large_straight() {
        let test_dice: Vec<DieFace> = vec![6, 3, 4, 2, 5];
        let hand = Dice::roll_fake(test_dice);

        let scorecard = scorecard::get_new_scorecard_data();
        let score = (scorecard.get_line_by_id(L::SmallStraight).calc)(&hand, false);
        assert_eq!(score, VALUE_SMALL_STRAIGHT);
    }

    #[test]
    fn test_small_straight_no_straight() {
        let test_dice: Vec<DieFace> = vec![6, 3, 3, 2, 5];
        let hand = Dice::roll_fake(test_dice);

        let scorecard = scorecard::get_new_scorecard_data();
        let score = (scorecard.get_line_by_id(L::SmallStraight).calc)(&hand, false);
        assert_eq!(score, 0);
    }

    #[test]
    fn test_small_straight_dice5_special() {
        let test_dice: Vec<DieFace> = vec![3, 3, 3, 3, 3];
        let hand = Dice::roll_fake(test_dice);

        let scorecard = scorecard::get_new_scorecard_data();
        let score = (scorecard.get_line_by_id(L::SmallStraight).calc)(&hand, true);
        assert_eq!(score, VALUE_SMALL_STRAIGHT);
    }

    #[test]
    fn test_large_straight_high_straight() {
        let test_dice: Vec<DieFace> = vec![6, 3, 4, 2, 5];
        let hand = Dice::roll_fake(test_dice);

        let scorecard = scorecard::get_new_scorecard_data();
        let score = (scorecard.get_line_by_id(L::LargeStraight).calc)(&hand, false);
        assert_eq!(score, VALUE_LARGE_STRAIGHT);
    }

    #[test]
    fn test_large_straight_no_straight() {
        let test_dice: Vec<DieFace> = vec![6, 2, 2, 2, 5];
        let hand = Dice::roll_fake(test_dice);

        let scorecard = scorecard::get_new_scorecard_data();
        let score = (scorecard.get_line_by_id(L::LargeStraight).calc)(&hand, false);
        assert_eq!(score, 0);
    }

    #[test]
    fn test_large_straight_dice5_special() {
        let test_dice: Vec<DieFace> = vec![1, 1, 1, 1, 1];
        let hand = Dice::roll_fake(test_dice);

        let scorecard = scorecard::get_new_scorecard_data();
        let score = (scorecard.get_line_by_id(L::LargeStraight).calc)(&hand, true);
        assert_eq!(score, VALUE_LARGE_STRAIGHT);
    }

    #[test]
    fn test_large_straight_low_straight() {
        let test_dice: Vec<DieFace> = vec![1, 3, 4, 2, 5];
        let hand = Dice::roll_fake(test_dice);

        let scorecard = scorecard::get_new_scorecard_data();
        let score = (scorecard.get_line_by_id(L::LargeStraight).calc)(&hand, false);
        assert_eq!(score, VALUE_LARGE_STRAIGHT);
    }

    #[test]
    fn test_full_house_no_full_house() {
        let test_dice: Vec<DieFace> = vec![6, 3, 3, 2, 5];
        let hand = Dice::roll_fake(test_dice);

        let scorecard = scorecard::get_new_scorecard_data();
        let score = (scorecard.get_line_by_id(L::FullHouse).calc)(&hand, false);
        assert_eq!(score, 0);
    }

    #[test]
    fn test_full_house_full_house() {
        let test_dice: Vec<DieFace> = vec![6, 3, 3, 6, 3];
        let hand = Dice::roll_fake(test_dice);

        let scorecard = scorecard::get_new_scorecard_data();
        let score = (scorecard.get_line_by_id(L::FullHouse).calc)(&hand, false);
        assert_eq!(score, VALUE_FULL_HOUSE);
    }

    #[test]
    fn test_full_house_dice5_special() {
        let test_dice: Vec<DieFace> = vec![2, 2, 2, 2, 2];
        let hand = Dice::roll_fake(test_dice);

        let scorecard = scorecard::get_new_scorecard_data();
        let score = (scorecard.get_line_by_id(L::FullHouse).calc)(&hand, true);
        assert_eq!(score, VALUE_FULL_HOUSE);
    }

    #[test]
    fn test_chance() {
        let test_dice: Vec<DieFace> = vec![1, 2, 3, 4, 5];
        let hand = Dice::roll_fake(test_dice);

        let scorecard = scorecard::get_new_scorecard_data();
        let score = (scorecard.get_line_by_id(L::Chance).calc)(&hand, false);
        assert_eq!(score, 1 + 2 + 3 + 4 + 5);
    }

    #[test]
    fn test_dice5_has_dice5() {
        let test_dice: Vec<DieFace> = vec![1, 1, 1, 1, 1];
        let hand = Dice::roll_fake(test_dice);

        let scorecard = scorecard::get_new_scorecard_data();
        let score = (scorecard.get_line_by_id(L::Dice5).calc)(&hand, false);
        assert_eq!(score, VALUE_DICE5);
    }

    #[test]
    fn test_dice5_has_no_dice5() {
        let test_dice: Vec<DieFace> = vec![2, 1, 1, 1, 1];
        let hand = Dice::roll_fake(test_dice);

        let scorecard = scorecard::get_new_scorecard_data();
        let score = (scorecard.get_line_by_id(L::Dice5).calc)(&hand, false);
        assert_eq!(score, 0);
    }

    #[test]
    fn test_aces_all_zeros() {
        let test_dice: Vec<DieFace> = vec![0, 0, 0, 0, 0];
        let hand = Dice::roll_fake(test_dice);

        let scorecard = scorecard::get_new_scorecard_data();
        let score = (scorecard.get_line_by_id(L::Ace).calc)(&hand, false);
        assert_eq!(score, 0);
    }
}
