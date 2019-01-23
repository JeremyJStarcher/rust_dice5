mod calchand;
mod dice;
mod scorecard;
mod ui;

#[macro_use]
extern crate text_io;

fn main() {
    let mut scorecard = scorecard::get_new_scorecard_data();

    while !scorecard.game_over() {
        ui::show_card(&scorecard);

        println!("Your Play >> ");
        let line: String = read!("{}\n");
        println!("You entered {} !", line);
    }
}
