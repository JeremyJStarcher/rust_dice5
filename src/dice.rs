extern crate rand;
use rand::Rng;
use std::fmt;

pub type DieFace = i8;

#[derive(Debug)]
pub struct Dice {
    pub dice: Vec<DieFace>,
}

impl fmt::Display for Dice {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let codes: Vec<String> = self
            .dice
            .iter()
            .map(|k| {
                let m = match k {
                    1 => "⚀".to_string(),
                    2 => "⚁".to_string(),
                    3 => "⚂".to_string(),
                    4 => "⚃".to_string(),
                    5 => "⚄".to_string(),
                    6 => "⚅".to_string(),
                    _ => "🎲".to_string(),
                };
                m
            })
            .collect();

        write!(f, "{}", codes.join(" "))
    }
}

impl Dice {
    const NUMBER_OF_DICE: usize = 5;
    const NUMBER_OF_FACES: i8 = 6;

    pub fn roll_die() -> DieFace {
        let mut rng = rand::thread_rng();
        rng.gen_range(0, Dice::NUMBER_OF_FACES) + 1
    }

    #[allow(dead_code)]
    pub fn roll_fake(dice: Vec<DieFace>) -> Dice {
        let dice = Dice { dice };
        dice
    }

    pub fn roll_all() -> Dice {
        let hand = Dice::get_empty_hand();

        let reroll_flags: Vec<bool> = hand.dice.iter().map(|_i| true).collect();
        println!("reroll_flags {:?}", reroll_flags);

        let new_hand = Dice::reroll_hand(hand, reroll_flags);
        new_hand
    }

    pub fn reroll_hand(hand: Dice, reroll: Vec<bool>) -> Dice {
        if hand.dice.len() != reroll.len() {
            panic!("dice.length and re-roll length must match");
        }

        let dice: Vec<DieFace> = hand
            .dice
            .iter()
            .zip(reroll)
            .map(|(face, flag)| {
                let f = if flag { Dice::roll_die() } else { *face };
                f
            })
            .collect();

        return Dice { dice };
    }

    pub fn get_empty_hand() -> Dice {
        let mut dice = Vec::with_capacity(Dice::NUMBER_OF_DICE);

        for _i in 0..Dice::NUMBER_OF_DICE {
            dice.push(0);
        }
        Dice { dice }
    }
}

#[cfg(test)]
mod tests {
    use super::Dice;

    #[test]
    fn roll_all_correct_number_of_dice() {
        let hand = Dice::roll_all();

        assert_eq!(hand.dice.len(), Dice::NUMBER_OF_DICE);
    }

    #[test]
    fn get_empty_hand_correct_number_of_dice() {
        let hand = Dice::get_empty_hand();

        assert_eq!(hand.dice.len(), Dice::NUMBER_OF_DICE);
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
