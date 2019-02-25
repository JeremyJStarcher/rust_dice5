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

// use term_painter::Attr::*;
// use term_painter::{Color, ToStyle};
use crate::engine::{Dice, DieFace, LineId, ScoreCardData};
use crate::text;
use term_painter::Color::*;
use term_painter::ToStyle;

const LONG_NAME_WIDTH: usize = 15;
const SCORE_BOX_WIDTH: usize = 5;

pub fn print_line(score_card: &ScoreCardData, id: LineId) {
    let line = score_card.get_line_by_id(id);
    print!(
        "{:width$}  ",
        White.bg(Black).paint(text::get_long_name(line.id)),
        width = LONG_NAME_WIDTH,
    );
    if let Some(val) = line.value {
        print!(
            "{:>width$}",
            Cyan.bg(Black).bold().paint(val),
            width = SCORE_BOX_WIDTH,
        );
    } else {
        let short = format!("<{}>", text::get_short_name(line.id));
        print!(
            "{:width$}",
            Yellow.bg(Black).bold().paint(short),
            width = SCORE_BOX_WIDTH
        );
    }
}

pub fn print_subtotal(id: LineId, score_card: &ScoreCardData) {
    let line = score_card.get_subtotal_by_id(id);
    let val = (line.calc)(&score_card);

    print!(
        "{:width$}",
        White.bg(Black).paint(text::get_long_name(line.id)),
        width = LONG_NAME_WIDTH,
    );

    print!("  ");

    print!(
        "{:width$}",
        Cyan.bg(Black).bold().paint(&val),
        width = SCORE_BOX_WIDTH,
    );
    println!();
}

pub fn show_card(score_card: &ScoreCardData) {
    print_line(score_card, LineId::Ace);
    println!();
    print_line(score_card, LineId::Two);
    println!();
    print_line(score_card, LineId::Three);
    println!();
    print_line(score_card, LineId::Four);
    println!();
    print_line(score_card, LineId::Five);
    println!();
    print_line(score_card, LineId::Six);
    println!();
    print_subtotal(LineId::UpperSubtotal, &score_card);
    print_subtotal(LineId::UpperBonus, &score_card);
    print_subtotal(LineId::UpperTotal, &score_card);
    println!("-------------------------");

    print_line(score_card, LineId::ThreeKind);
    println!();
    print_line(score_card, LineId::FourKind);
    println!();
    print_line(score_card, LineId::SmallStraight);
    println!();
    print_line(score_card, LineId::LargeStraight);
    println!();
    print_line(score_card, LineId::FullHouse);
    println!();
    print_line(score_card, LineId::Chance);
    println!();
    print_line(score_card, LineId::Dice5);
    println!();
    print_subtotal(LineId::BottomSubtotal, &score_card);
    println!("-------------------------");

    print_subtotal(LineId::Dice5Bonus, &score_card);
    println!("-------------------------");

    print_subtotal(LineId::GrandTotal, &score_card);
    println!("-------------------------");
}

pub fn show_hand(hand: &Dice) {
    fn print_color(s: &str, face: DieFace) {
        print!(
            "{}",
            match face {
                1 => Red.bg(White).paint(&s),
                2 => Magenta.bg(White).paint(&s),
                3 => BrightGreen.bg(White).paint(&s),
                4 => BrightCyan.bg(White).paint(&s),
                5 => Green.bg(White).paint(&s),
                6 => Black.bg(White).paint(&s),
                _ => Black.bg(White).paint(&s),
            }
        );
    }

    const LINES: usize = 3;
    #[rustfmt::skip]
    static DICE: [[&str; LINES]; 6] = [
        [
            "     ",
            "  ●  ",
            "     ",
        ],
        [
            "●    ",
            "     ",
            "    ●",
        ],
        [
            "    ●",
            "  ●  ",
            "●    ",
        ],
        [
            "●   ●",
            "     ",
            "●   ●",
        ],
        [
            "●   ●",
            "  ●  ",
            "●   ●",
        ],
        [
            "●   ●",
            "●   ●",
            "●   ●",
        ],
    ];

    for l in 0..LINES {
        for &v in &hand.dice {
            // v as usize wil panic if v < 0 since v: i8
            let line = DICE.get(v as usize - 1).expect("Unknown face")[l];

            print_color(line, v);
            print!("  ");
        }
        println!();
    }
    println!();
    println!("{}", hand);
    println!("Rolls left: {}", hand.rolls_left);
}
