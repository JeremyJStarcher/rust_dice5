use super::engine::LineId;

pub fn get_long_name(zid: LineId) -> String {
    match zid {
        LineId::Ace => "Aces".to_string(),
        LineId::Two => "Twos".to_string(),
        LineId::Three => "Threes".to_string(),
        LineId::Four => "Fours".to_string(),
        LineId::Five => "Fives".to_string(),
        LineId::Six => "Sixes".to_string(),
        LineId::ThreeKind => "3 Kind".to_string(),
        LineId::FourKind => "4 Kind".to_string(),
        LineId::SmallStraight => "Small Straight".to_string(),
        LineId::LargeStraight => "Large Straight".to_string(),
        LineId::FullHouse => "Full House".to_string(),
        LineId::Chance => "Change".to_string(),
        LineId::Dice5 => "Dice 5".to_string(),
    }
}

pub fn get_short_name(zid: LineId) -> String {
    match zid {
        LineId::Ace => "1".to_string(),
        LineId::Two => "2".to_string(),
        LineId::Three => "3".to_string(),
        LineId::Four => "4".to_string(),
        LineId::Five => "5".to_string(),
        LineId::Six => "6".to_string(),
        LineId::ThreeKind => "3k".to_string(),
        LineId::FourKind => "4k".to_string(),
        LineId::SmallStraight => "ss".to_string(),
        LineId::LargeStraight => "ls".to_string(),
        LineId::FullHouse => "fh".to_string(),
        LineId::Chance => "c".to_string(),
        LineId::Dice5 => "d".to_string(),
    }
}

pub fn get_id_by_short_name(s: &str) -> LineId {
    match s {
        "1" => LineId::Ace,
        "2" => LineId::Two,
        "3" => LineId::Three,
        "4" => LineId::Four,
        "5" => LineId::Five,
        "6" => LineId::Six,
        "3k" => LineId::ThreeKind,
        "4k" => LineId::FourKind,
        "ss" => LineId::SmallStraight,
        "ls" => LineId::LargeStraight,
        "fh" => LineId::FullHouse,
        "c" => LineId::Chance,
        "d" => LineId::Dice5,
        _ => panic!("Ooops"),
    }
}
