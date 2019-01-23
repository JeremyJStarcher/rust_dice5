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

        let mut line = "".to_string();
        if cfg!(windows) {
          line = read!("{}\r\n");
        } else if cfg!(unix) {
          line = read!("{}\n");
        }

        let point_result = scorecard.get_points(&line, &hand, false);
        match point_result {
            Err(SErr::NotFound) => {
                println!("I have no idea what this means: {}.", line);
            }
            Err(SErr::AlreadySet) => {
                println!("A value for {} has already been set.", line);
            }
            Ok(points) => match scorecard.set_val(&line, points) {
                _ => {}
            },
        }
    }
}
