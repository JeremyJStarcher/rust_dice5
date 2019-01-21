extern crate term_painter;

use term_painter::Attr::*;
use term_painter::Color::*;
use term_painter::{Color, ToStyle};

use super::scorecard;
use super::scorecard::LineId;
pub fn print_line(score_card: &scorecard::ScoreCardData, id: LineId) {
    let line = score_card.by_id(id);

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
        width = 15,
    );

    print!("  ");

    match line.value {
        None => print!("{:width$}", Yellow.bg(Black).bold().paint(&val), width = 5,),
        _ => print!("{:>width$}", Cyan.bg(Black).bold().paint(&val), width = 5,),
    }
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
    print_line(score_card, LineId::Yahtzee);
    println!("");
}

// pub fn demo() {
//     struct_sizes();

//     simple_examples();
//     with_example();
//     doc_examples();

//     all_styles(&[
//         NotSet, Black, Red, Green, Yellow, Blue, Magenta, Cyan, White,
//     ]);
//     all_styles(&[
//         BrightBlack,
//         BrightRed,
//         BrightGreen,
//         BrightYellow,
//         BrightBlue,
//         BrightMagenta,
//         BrightCyan,
//         BrightWhite,
//     ]);
// }

// fn struct_sizes() {
//     use std::mem::size_of;

//     println!("size_of(Style): {}", size_of::<term_painter::Style>());
//     println!("size_of(Color): {}", size_of::<Color>());
//     println!("size_of(Attr):  {}", size_of::<term_painter::Attr>());
// }

// fn simple_examples() {
//     println!(
//         "{} | {} | {} | {} | {}",
//         Red.bg(Green).bold().paint("Red-Green-Bold"),
//         Blue.paint("Blue"),
//         Blue.bold().paint("BlueBold"),
//         Blue.bg(Magenta).paint("BlueMagentaBG"),
//         Plain.underline().paint("Underline")
//     );
// }

// fn with_example() {
//     Red.with(|| {
//         print!("JustRed");
//         Bold.with(|| {
//             print!(" BoldRed {} BoldRed ", Underline.paint("Underline"));
//         });
//         print!("JustRed ");

//         print!("{}", Blue.paint("Blue (overwrite) "));
//         Green.with(|| {
//             println!("Green (overwrite)");
//         });
//     });
// }

// fn doc_examples() {
//     // --- Doc example 1
//     println!(
//         "{} or {} or {}",
//         Red.paint("Red"),
//         Bold.paint("Bold"),
//         Red.bold().paint("Both!")
//     );

//     // --- Doc example 2
//     let x = 5;

//     // These two are equivalent
//     println!("{} | {}", x, Plain.paint(x));

//     // These two are equivalent, too
//     println!("{} | {}", Red.paint(x), Plain.fg(Red).paint(x));

//     // --- Doc example 3
//     let non_copy = "cake".to_string(); // String is *not* Copy
//     let copy = 27; // usize/isize *is* Copy

//     println!("{}", Plain.paint(&non_copy));
//     println!("{}", Plain.paint(&copy));
//     // non_copy is still usable here...
//     // copy is still usable here...

//     println!("{}", Plain.paint(non_copy));
//     println!("{}", Plain.paint(copy));
//     // non_copy was moved into paint, so it not usable anymore...
//     // copy is still usable here...
// }

// fn all_styles(colors: &[Color]) {
//     // Normal test
//     for c in colors {
//         print!("{:?} ", c.paint(c));
//     }
//     println!("    (fg)");
//     for c in colors {
//         print!("{:?} ", Plain.bg(*c).paint(c));
//     }
//     println!("    (bg)");

//     // Bold text
//     for c in colors {
//         print!("{:?} ", c.bold().paint(c));
//     }
//     println!("    (bold fg)");
//     for c in colors {
//         print!("{:?} ", Bold.bg(*c).paint(c));
//     }
//     println!("    (bold bg)");

//     // Dim text
//     for c in colors {
//         print!("{:?} ", c.dim().paint(c));
//     }
//     println!("    (dim fg)");
//     for c in colors {
//         print!("{:?} ", Dim.bg(*c).paint(c));
//     }
//     println!("    (dim bg)");

//     // Underlined text
//     for c in colors {
//         print!("{:?} ", c.underline().paint(c));
//     }
//     println!("    (underline fg)");
//     for c in colors {
//         print!("{:?} ", Underline.bg(*c).paint(c));
//     }
//     println!("    (underline bg)");

//     // Blinking text
//     for c in colors {
//         print!("{:?} ", c.blink().paint(c));
//     }
//     println!("    (blink fg)");
//     for c in colors {
//         print!("{:?} ", Blink.bg(*c).paint(c));
//     }
//     println!("    (blink bg)");

//     // Reverse text
//     for c in colors {
//         print!("{:?} ", c.reverse().paint(c));
//     }
//     println!("    (reverse fg)");
//     for c in colors {
//         print!("{:?} ", Reverse.bg(*c).paint(c));
//     }
//     println!("    (reverse bg)");

//     // Secure text
//     for c in colors {
//         print!("{:?} ", c.secure().paint(c));
//     }
//     println!("    (secure fg)");
//     for c in colors {
//         print!("{:?} ", Secure.bg(*c).paint(c));
//     }
//     println!("    (secure bg)");
// }
