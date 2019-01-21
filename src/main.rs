mod calchand;
mod dice;
mod scorecard;

fn main() {
    let hand = dice::Dice::first_roll();
    let reroll_flags: Vec<bool> = hand.dice.iter().map(|_i| true).collect();
    println!("{}", hand);
    println!("{:?}", calchand::calc_3k(&hand));

    let hand2 = dice::Dice::reroll_hand(hand, reroll_flags);
    println!("{}", hand2);
    println!("{:?}", calchand::calc_3k(&hand2));

    let mut scorecard = scorecard::get_new_scorecard_data();
    let points = calchand::calc_ace(&hand2);

    let result = scorecard.set_val(&"1".to_string(), points);
    match result {
        Err(scorecard::SetError::NotFound) => {}
        Err(scorecard::SetError::AlreadySet) => {}
        Ok(_) => {}
    }

    let points = calchand::calc_two(&hand2);
    let result = scorecard.set_val(&"2".to_string(), points);
    match result {
        Err(scorecard::SetError::NotFound) => {}
        Err(scorecard::SetError::AlreadySet) => {}
        Ok(_) => {}
    }

    println!("{}", scorecard);
}
