mod calchand;
mod dice;
mod scorecard;
mod ui;

#[macro_use]
extern crate text_io;

fn read_line() -> String {
    loop {
        let line: String = if cfg!(windows) {
            read!("{}\r\n")
        } else if cfg!(unix) {
            read!("{}\n")
        } else {
            panic!("Neither Windows nor Unix? What manner of beast art thou?");
        };

        if line.len() > 0 {
            break line;
        }
    }
}

fn play(slot: &str, hand: &dice::Dice, scorecard: &mut scorecard::ScoreCardData) -> bool {
    use scorecard::SetError as SErr;
    let point_result = scorecard.get_points(&slot, &hand, false);
    let mut ret = false;

    match point_result {
        Err(SErr::NotFound) => println!("I have no idea what this means: {}.", slot),
        Err(SErr::AlreadySet) => println!("A value for {} has already been set.", slot),
        Ok(points) => match scorecard.set_val(&slot, points) {
            _ => ret = true,
        },
    }
    ret
}
fn main() {
    let mut scorecard = scorecard::get_new_scorecard_data();
    let mut hand = dice::Dice::first_roll();

    ui::show_card(&scorecard);
    ui::show_hand(&hand);

    while !scorecard.game_over() {
        println!("Your turn.  'play' or 'roll' >> ");

        let line = read_line();
        let words: Vec<&str> = line.split_whitespace().collect();

        match words[0] {
            "play" => match words.len() {
                2 => {
                    let slot = words[1];
                    if play(&slot, &hand, &mut scorecard) {
                        hand = dice::Dice::first_roll();
                        ui::show_card(&scorecard);
                        ui::show_hand(&hand);
                    }
                }
                _ => {
                    println!("Play in a position, like 'play fh'");
                }
            },
            "roll" => {
                println!("Rock and roll");
            }
            _ => {}
        };
    }
}
