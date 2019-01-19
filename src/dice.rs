extern crate rand;
use rand::Rng;

#[derive(Debug)]
pub struct Dice {
    pub dice: Vec<i8>,
}

impl Dice {
    const NUMBER_OF_DICE: usize = 5;

    pub fn roll_all() -> Vec<i8> {
        let mut r = Vec::with_capacity(Dice::NUMBER_OF_DICE);
        let mut rng = rand::thread_rng();

        for _i in 0..Dice::NUMBER_OF_DICE {
            let die = rng.gen_range(1, 7);
            r.push(die);
        }

        return r;
    }
}
