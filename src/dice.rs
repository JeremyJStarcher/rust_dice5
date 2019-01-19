extern crate rand;
use rand::Rng;

#[derive(Debug)]
pub struct Dice {
    pub dice: Vec<i8>,
}

impl Dice {
    const NUMBER_OF_DICE: usize = 5;
    const NUMBER_OF_FACES: i8 = 6;

    pub fn roll_die() -> i8 {
        let mut rng = rand::thread_rng();
        rng.gen_range(0, Dice::NUMBER_OF_FACES) + 1
    }

    pub fn roll_all() -> Vec<i8> {
        let mut r = Vec::with_capacity(Dice::NUMBER_OF_DICE);

        for _i in 0..Dice::NUMBER_OF_DICE {
            let die = Dice::roll_die();
            r.push(die);
        }

        return r;
    }

    pub fn get_empty_hand() -> Vec<i8> {
        let mut r = Vec::with_capacity(Dice::NUMBER_OF_DICE);

        for _i in 0..Dice::NUMBER_OF_DICE {
            r.push(0);
        }
        return r;
    }
}

#[cfg(test)]
mod tests {
    use super::Dice;

    #[test]
    fn roll_all_correct_number_of_dice() {
        let k = Dice::roll_all();

        assert_eq!(k.len(), Dice::NUMBER_OF_DICE);
    }

    #[test]
    fn get_empty_hand_correct_number_of_dice() {
        let k = Dice::get_empty_hand();

        assert_eq!(k.len(), Dice::NUMBER_OF_DICE);
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
