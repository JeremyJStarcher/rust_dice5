mod calchand;
mod hand;
mod scorecard;
mod ui;

use std::io;
use hand::Dice;
use hand::DieFace;

#[macro_use]
extern crate text_io;

fn read_command() -> Vec<String> {
    let mut input = String::new();
    
    loop {
        io::stdin().read_line(&mut input).expect("Unable to read stdin");

        let words: Vec<_> = input.split_whitespace().cloned().collect();
        if words.len() > 0 {
            return words;
        }
    }
}

fn play(slot: &str, hand: &Dice, scorecard: &mut scorecard::ScoreCardData) -> bool {
    use scorecard::SetError as SErr;

    let point_result = scorecard.play(&slot, &hand);
    let mut ret = false;

    match point_result {
        Err(SErr::NotFound) => println!("I have no idea what this means: {}.", slot),
        Err(SErr::AlreadySet) => println!("A value for {} has already been set.", slot),
        Ok(points) => {
            let line = scorecard.get_line_by_short_name(slot);
            println!("Played {} points on {}", points, line.long_name);
            ret = true;
        }
    }
    ret
}

fn main() {
    let mut scorecard = scorecard::get_new_scorecard_data();
    let mut hand = Dice::first_roll();

    ui::show_card(&scorecard);
    ui::show_hand(&hand);

    while !scorecard.game_over() {
        println!("Your turn.  'play', 'roll' or 'cheat' >> ");

        let words = read_command();

        match words[0] {
            "play" => match words.len() {
                2 => {
                    let slot = words[1];
                    if play(&slot, &hand, &mut scorecard) {
                        hand = Dice::first_roll();
                        ui::show_card(&scorecard);
                        ui::show_hand(&hand);
                    }
                }
                _ => {
                    println!("Play in a position, like 'play fh'");
                }
            },
            "cheat" => {
                let dice: Vec<DieFace> = vec![6, 6, 6, 6, 6];
                hand = Dice::roll_fake(dice);
                ui::show_hand(&hand);
            }
            "roll" => match words.len() {
                1 => {
                    println!("Which die positions to roll?");
                    println!("Example 'roll 1 2 3' to re-roll the first three dice.");
                }
                _ => {
                    if hand.rolls_left == 0 {
                        println!("No rolls left");
                    } else {
                        let position_to_reroll: Vec<_> = words[1..].iter().collect();
                        let position_to_reroll: Vec<_> = position_to_reroll
                            .iter()
                            .map(|l| (***l).parse().unwrap_or(0) - 1)
                            .collect();

                        let range = 0..hand.dice.len();

                        let reroll_flags: Vec<_> = range
                            .map(|p| position_to_reroll.iter().any(|&x| x == p))
                            .collect();

                        hand = Dice::reroll_hand(hand, &reroll_flags);

                        ui::show_hand(&hand);
                    }
                }
            },
            _ => {}
        };
    }
}
