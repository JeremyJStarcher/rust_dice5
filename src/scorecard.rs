use super::calchand;
use super::dice;
use std::fmt;

#[derive(Debug)]
pub struct LineData {
    pub long_name: String,
    pub short_name: String,
    pub value: Option<i16>,
    pub calc: fn(dice: &dice::Dice) -> i16,
}
impl fmt::Display for LineData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // HELP: Avoid `clone`
        let sname = self.short_name.clone();
        let prefix = "<".to_string();
        let suffix = ">".to_string();

        let val = match self.value {
            None => [prefix, sname, suffix].join(""),
            _ => self.value.unwrap().to_string(),
        };
        write!(
            f,
            "{:width$} {:>width2$}",
            self.long_name,
            val,
            width = 15,
            width2 = 5
        )
    }
}

#[derive(Debug)]
pub struct ScoreCardData {
    pub ace: LineData,
    pub two: LineData,
    pub three: LineData,
    pub four: LineData,
    pub five: LineData,
    pub six: LineData,
    pub three_kind: LineData,
    pub four_kind: LineData,
    pub small_straight: LineData,
    pub large_straight: LineData,
    pub full_house: LineData,
    pub chance: LineData,
    pub yahtzee: LineData,

    pub yahtzee_bonus: i8,
}

impl fmt::Display for ScoreCardData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let out: Vec<String> = vec![
            format!("{}", self.ace),
            format!("{}", self.two),
            format!("{}", self.three),
            format!("{}", self.four),
            format!("{}", self.five),
            format!("{}", self.six),
            format!("-------------------------"),
            format!("{}", self.three_kind),
            format!("{}", self.four_kind),
            format!("{}", self.small_straight),
            format!("{}", self.large_straight),
            format!("{}", self.full_house),
            format!("{}", self.chance),
            format!("{}", self.yahtzee),
        ];

        write!(f, "{}", out.join("\n"))
    }
}

pub fn get_new_scorecard_data() -> ScoreCardData {
    let card = ScoreCardData {
        ace: LineData {
            long_name: "Aces".to_string(),
            short_name: "1".to_string(),
            value: None,
            calc: calchand::calc_ace,
        },
        two: LineData {
            long_name: "Twos".to_string(),
            short_name: "2".to_string(),
            value: None,
            calc: calchand::calc_two,
        },
        three: LineData {
            long_name: "Threes".to_string(),
            short_name: "3".to_string(),
            value: None,
            calc: calchand::calc_three,
        },
        four: LineData {
            long_name: "Fours".to_string(),
            short_name: "4".to_string(),
            value: None,
            calc: calchand::calc_four,
        },
        five: LineData {
            long_name: "Fives".to_string(),
            short_name: "5".to_string(),
            value: None,
            calc: calchand::calc_five,
        },
        six: LineData {
            long_name: "Sixes".to_string(),
            short_name: "6".to_string(),
            value: None,
            calc: calchand::calc_six,
        },
        three_kind: LineData {
            long_name: "Three of a Kind".to_string(),
            short_name: "3k".to_string(),
            value: None,
            calc: calchand::calc_3k,
        },
        four_kind: LineData {
            long_name: "Four of a Kind".to_string(),
            short_name: "4k".to_string(),
            value: None,
            calc: calchand::calc_4k,
        },
        small_straight: LineData {
            long_name: "Small Straight".to_string(),
            short_name: "ss".to_string(),
            value: None,
            calc: calchand::calc_ss,
        },
        large_straight: LineData {
            long_name: "Large Straight".to_string(),
            short_name: "ls".to_string(),
            value: None,
            calc: calchand::calc_ls,
        },
        full_house: LineData {
            long_name: "Full House".to_string(),
            short_name: "fh".to_string(),
            value: None,
            calc: calchand::calc_fh,
        },
        chance: LineData {
            long_name: "Chance".to_string(),
            short_name: "c".to_string(),
            value: None,
            calc: calchand::calc_chance,
        },
        yahtzee: LineData {
            long_name: "Yahtzee".to_string(),
            short_name: "y".to_string(),
            value: None,
            calc: calchand::calc_yahtzee,
        },
        yahtzee_bonus: 0,
    };
    card
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_new_scorecard_returns_card() {
        let scorecard = get_new_scorecard_data();
        assert_eq!(scorecard.ace.value, None);
    }
}
