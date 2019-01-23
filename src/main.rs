mod calchand;
mod dice;
mod scorecard;
mod ui;

#[macro_use]
extern crate text_io;

fn read_line() -> String {
    if cfg!(windows) {
        read!("{}\r\n")
    } else if cfg!(unix) {
        read!("{}\n")
    } else {
        panic!("No idea what to do with this");
    }
}
fn main() {
    use scorecard::SetError as SErr;

    let mut scorecard = scorecard::get_new_scorecard_data();

    while !scorecard.game_over() {
        let hand = dice::Dice::first_roll();

        ui::show_card(&scorecard);
        ui::show_hand(&hand);

        println!("Your Play >> ");
        let line = read_line();

        let point_result = scorecard.get_points(&line, &hand, false);
        match point_result {
            Err(SErr::NotFound) => println!("I have no idea what this means: {}.", line),
            Err(SErr::AlreadySet) => println!("A value for {} has already been set.", line),
            Ok(points) => match scorecard.set_val(&line, points) {
                _ => {}
            },
        }
    }
}
