mod calchand;
mod dice;
mod scorecard;
mod ui;

#[macro_use]
extern crate text_io;

fn main() {
    use scorecard::SetError as SErr;

    let mut scorecard = scorecard::get_new_scorecard_data();

    while !scorecard.game_over() {
        let hand = dice::Dice::first_roll();

        ui::show_card(&scorecard);

        println!("Your Play >> ");
        let line: String = read!("{}\n");

        let point_result = scorecard.get_points(&line, &hand, false);
        match point_result {
            Err(SErr::NotFound) => {
                panic!("Not found shouldn't happen");
            }
            Err(SErr::AlreadySet) => {
                panic!("Already Set shoudln't happen");
            }
            Ok(points) => match scorecard.set_val(&line, points) {
                _ => {}
            },
        }
    }
}
