use super::super::engine;
use super::super::text;
use super::ui;
use engine::SetError as SErr;
use engine::{get_new_scorecard_data, Dice, ScoreCardData};
use std::io::BufRead;

fn read_line() -> String {
    for line in std::io::stdin().lock().lines() {
        let line = line.expect("Read error");

        let words: Vec<_> = line.split_whitespace().collect();

        if !words.is_empty() {
            return line;
        }
    }
    panic!("Out of input");
}

fn play(slot: &str, hand: &Dice, scorecard: &mut ScoreCardData) -> bool {
    let id = text::get_id_by_short_name(&slot);

    let point_result = scorecard.play(id, &hand);
    let mut ret = false;

    match point_result {
        Err(SErr::NotFound) => println!("I have no idea what this means: {}.", slot),
        Err(SErr::AlreadySet) => println!("A value for {} has already been set.", slot),
        Ok(points) => {
            let long_name = text::get_long_name(id);
            println!("Played {} points on {}", points, long_name);
            ret = true;
        }
    }
    ret
}

pub fn main() {
    let mut scorecard = get_new_scorecard_data();
    let mut hand = Dice::first_roll();

    ui::show_card(&scorecard);
    ui::show_hand(&hand);

    while !scorecard.game_over() {
        println!("Your turn.  'play', 'roll' or 'cheat' >> ");

        let line = read_line();
        let words: Vec<_> = line.split_whitespace().collect();

        match words[0] {
            "play" => match words.as_slice() {
                [_, slot] => {
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
                let dice = vec![6; 5];
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
                        let mut reroll_flags = vec![false; hand.dice.len()];

                        words[1..]
                            .iter()
                            .flat_map(|l| l.parse::<usize>())
                            .map(|p| p - 1)
                            .for_each(|p| {
                                if let Some(flag) = reroll_flags.get_mut(p) {
                                    *flag = true
                                }
                            });

                        hand.reroll(&reroll_flags);

                        ui::show_hand(&hand);
                    }
                }
            },
            _ => {}
        };
    }
}
