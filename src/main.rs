mod calchand;
mod dice;
mod scorecard;

fn main() {
    let hand = dice::Dice::roll_all();
    let scoredata = scorecard::get_new_scorecard_data();

    println!("{}", hand);
    println!("{:?}", calchand::calc_3k(&hand));
}
