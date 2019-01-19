mod dice;
mod scorecard;

fn main() {
    let dice = dice::Dice::roll_all();
    let scoredata = scorecard::get_new_scorecard_data();

    let dice = dice::Dice { dice };

    println!("{}", dice);
    println!("{:?}", scoredata);
}
