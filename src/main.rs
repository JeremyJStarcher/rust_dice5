mod dice;

fn main() {
    let dice = dice::Dice::roll_all();

    let d = dice::Dice { dice };

    println!("{:?}", d);
}