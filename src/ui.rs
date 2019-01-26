/*
 *  Notes for the Reviewer
 *
 * term_painter must be used inside of a print! macro.
 * It cannot be used inside of write or format any of those wonderful
 * tools.
 *
 * Short version: The Windows console isn't a VT100/ANSI console and so
 * magick must happen.
*/

extern crate term_painter;

// use term_painter::Attr::*;
use term_painter::Color::*;
// use term_painter::{Color, ToStyle};
use term_painter::ToStyle;

use super::hand::DieFace;
use super::hand::Dice;
use super::scorecard;
use super::scorecard::LineId;

const LONG_NAME_WIDTH: usize = 15;
const SCORE_BOX_WIDTH: usize = 5;

pub fn print_line(score_card: &scorecard::ScoreCardData, id: LineId) {
    let line = score_card.get_line_by_id(&id);

    // HELP: How to get rid of the clone?
    let sname = line.short_name.clone();
    let prefix = "<".to_string();
    let suffix = ">".to_string();

    let val = match line.value {
        None => [prefix, sname, suffix].join(""),
        _ => line.value.unwrap().to_string(),
    };

    print!(
        "{:width$}",
        White.bg(Black).paint(&line.long_name),
        width = LONG_NAME_WIDTH,
    );

    print!("  ");

    match line.value {
        None => print!(
            "{:width$}",
            Yellow.bg(Black).bold().paint(&val),
            width = SCORE_BOX_WIDTH,
        ),
        _ => print!(
            "{:>width$}",
            Cyan.bg(Black).bold().paint(&val),
            width = SCORE_BOX_WIDTH,
        ),
    }
}

pub fn print_subtotal(line: &scorecard::SubtotalData, score_card: &scorecard::ScoreCardData) {
    let val = (line.calc)(&score_card);

    print!(
        "{:width$}",
        White.bg(Black).paint(&line.long_name),
        width = LONG_NAME_WIDTH,
    );

    print!("  ");

    print!(
        "{:width$}",
        Cyan.bg(Black).bold().paint(&val),
        width = SCORE_BOX_WIDTH,
    );
    println!("");
}

pub fn show_card(score_card: &scorecard::ScoreCardData) {
    print_line(score_card, LineId::Ace);
    println!("");
    print_line(score_card, LineId::Two);
    println!("");
    print_line(score_card, LineId::Three);
    println!("");
    print_line(score_card, LineId::Four);
    println!("");
    print_line(score_card, LineId::Five);
    println!("");
    print_line(score_card, LineId::Six);
    println!("");
    print_subtotal(&score_card.calc_upper_subtotal, &score_card);
    print_subtotal(&score_card.calc_upper_bonus, &score_card);
    print_subtotal(&score_card.calc_upper_total, &score_card);
    println!("-------------------------");

    print_line(score_card, LineId::ThreeKind);
    println!("");
    print_line(score_card, LineId::FourKind);
    println!("");
    print_line(score_card, LineId::SmallStraight);
    println!("");
    print_line(score_card, LineId::LargeStraight);
    println!("");
    print_line(score_card, LineId::FullHouse);
    println!("");
    print_line(score_card, LineId::Chance);
    println!("");
    print_line(score_card, LineId::Dice5);
    println!("");
    print_subtotal(&score_card.calc_lower_subtotal, &score_card);
    println!("-------------------------");

    print_subtotal(&score_card.calc_dice5_bonus, &score_card);
    println!("-------------------------");

    print_subtotal(&score_card.calc_grand_total, &score_card);
    println!("-------------------------");
}

pub fn show_hand(hand: &Dice) {
    fn print_color(s: &str, face: DieFace) {
        match face {
            1 => print!("{}", Red.bg(Black).paint(&s)),
            2 => print!("{}", Magenta.bg(Black).paint(&s)),
            3 => print!("{}", Yellow.bg(Black).paint(&s)),
            4 => print!("{}", BrightCyan.bg(Black).paint(&s)),
            5 => print!("{}", Green.bg(Black).paint(&s)),
            6 => print!("{}", White.bg(Black).paint(&s)),
            _ => print!("{}", Black.bg(White).paint(&s)),
        }
    }

    const SIX: &[&str] = &[
        "●   ●", //
        "●   ●", //
        "●   ●", //
    ];
    const FIVE: &[&str] = &[
        "●   ●", //
        "  ●  ",   //
        "●   ●", //
    ];
    const FOUR: &[&str] = &[
        "●   ●", //
        "     ",     //
        "●   ●", //
    ];
    const THREE: &[&str] = &[
        "    ●", //
        "  ●  ", //
        "●    ", //
    ];
    const TWO: &[&str] = &[
        "●    ", //
        "     ",   //
        "    ●", //
    ];
    const ONE: &[&str] = &[
        "     ",   //
        "  ●  ", //
        "     ",   //
    ];
    const LINES: usize = 3;

    for l in 0..LINES {
        for d in 0..hand.dice.len() {
            let v = hand.dice[d];
            let face = match v {
                1 => ONE,
                2 => TWO,
                3 => THREE,
                4 => FOUR,
                5 => FIVE,
                6 => SIX,
                _ => panic!("Unknown face"),
            };

            let s = face[l];
            print_color(s, v);
            print!("  ");
        }
        println!("");
    }
    println!("");
    println!("Rolls left: {}", hand.rolls_left);
}
