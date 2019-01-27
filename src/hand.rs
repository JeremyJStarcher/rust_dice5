extern crate rand;
use rand::Rng;
use std::fmt;

pub type DieFace = i8;

#[derive(Debug)]
pub struct Dice {
    pub dice: Vec<DieFace>,
    pub rolls_left: i8,
}

impl fmt::Display for Dice {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let codes: Vec<_> = self
            .dice
            .iter()
            .map(|k| match k {
                1 => "⚀".to_string(),
                2 => "⚁".to_string(),
                3 => "⚂".to_string(),
                4 => "⚃".to_string(),
                5 => "⚄".to_string(),
                6 => "⚅".to_string(),
                _ => "🎲".to_string(),
            })
            .collect();

        write!(f, "{}", codes.join(" "))
    }
}

impl Dice {
    pub const NUMBER_OF_DICE: usize = 5;
    pub const NUMBER_OF_FACES: i8 = 6;
    pub const ROLLS_PER_TURN: i8 = 3;

    pub fn roll_die() -> DieFace {
        let mut rng = rand::thread_rng();
        rng.gen_range(0, Self::NUMBER_OF_FACES) + 1
    }

    #[allow(dead_code)]
    pub fn roll_fake(dice: Vec<DieFace>) -> Self {
        Self {
            dice,
            rolls_left: Self::ROLLS_PER_TURN - 1,
        }
    }

    pub fn first_roll() -> Self {
        let dice: Vec<_> = (0..Self::NUMBER_OF_DICE)
            .map(|_i| Self::roll_die())
            .collect();

        Dice {
            dice,
            rolls_left: Self::ROLLS_PER_TURN - 1,
        }
    }

    pub fn reroll_hand(hand: Self, reroll: &[bool]) -> Self {
        if hand.dice.len() != reroll.len() {
            panic!("dice.length and re-roll length must match");
        }

        let dice: Vec<_> = hand
            .dice
            .iter()
            .zip(reroll)
            .map(|(face, flag)| match flag {
                true => Self::roll_die(),
                false => *face,
            })
            .collect();

        Self {
            dice,
            rolls_left: hand.rolls_left - 1,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_roll_correct_number_of_dice() {
        let hand = Dice::first_roll();

        assert_eq!(hand.dice.len(), Dice::NUMBER_OF_DICE);
        assert_eq!(hand.rolls_left, Dice::ROLLS_PER_TURN - 1);
    }

    #[test]
    fn re_roll_correct_number_of_dice() {
        let hand = Dice::first_roll();
        let reroll_flags: Vec<bool> = hand.dice.iter().map(|_i| true).collect();

        assert_eq!(hand.dice.len(), Dice::NUMBER_OF_DICE);
        assert_eq!(hand.rolls_left, Dice::ROLLS_PER_TURN - 1);

        let hand2 = Dice::reroll_hand(hand, &reroll_flags);
        assert_eq!(hand2.dice.len(), Dice::NUMBER_OF_DICE);
        assert_eq!(hand2.rolls_left, Dice::ROLLS_PER_TURN - 2);
    }

    #[test]
    fn roll_die_face_in_range() {
        for _i in 0..1000 {
            let die = Dice::roll_die();
            assert!(!(die > 6), format!("die value too high {}", die));
            assert!(!(die < 1), format!("die value too low {}", die));
        }
    }
}
