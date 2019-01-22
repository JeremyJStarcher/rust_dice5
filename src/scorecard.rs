use super::calchand;
use super::dice;
use std::fmt;

#[derive(PartialEq, Eq, Debug)]
pub enum SetError {
    AlreadySet,
    NotFound,
}

#[derive(PartialEq, Eq, Debug)]
pub enum LineId {
    Ace,
    Two,
    Three,
    Four,
    Five,
    Six,
    ThreeKind,
    FourKind,
    SmallStraight,
    LargeStraight,
    FullHouse,
    Chance,
    Dice5,
}

#[derive(Debug)]
pub struct LineData {
    pub id: LineId,
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

pub struct ScoreCardData {
    pub line: Vec<LineData>,
    pub bonus_dice5: i8,
}

impl fmt::Display for ScoreCardData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let out: Vec<String> = vec![
            format!("{}", self.by_id(LineId::Ace)),
            format!("{}", self.by_id(LineId::Two)),
            format!("{}", self.by_id(LineId::Three)),
            format!("{}", self.by_id(LineId::Four)),
            format!("{}", self.by_id(LineId::Five)),
            format!("{}", self.by_id(LineId::Six)),
            format!("-------------------------"),
            format!("{}", self.by_id(LineId::ThreeKind)),
            format!("{}", self.by_id(LineId::FourKind)),
            format!("{}", self.by_id(LineId::SmallStraight)),
            format!("{}", self.by_id(LineId::LargeStraight)),
            format!("{}", self.by_id(LineId::FullHouse)),
            format!("{}", self.by_id(LineId::Chance)),
            format!("{}", self.by_id(LineId::Dice5)),
        ];

        write!(f, "{}", out.join("\n"))
    }
}

impl ScoreCardData {
    pub fn by_id(&self, zid: LineId) -> &LineData {
        let line = self.line.iter().find(|l| l.id == zid);

        // HELP: How can I do this without the match?
        match line {
            None => panic!("not found"),
            Some(x) => x,
        }
    }

    pub fn set_val(&mut self, short_name: &String, value: i16) -> Result<bool, SetError> {
        let line = self.line.iter_mut().find(|l| l.short_name == *short_name);

        match line {
            None => Err(SetError::NotFound),
            Some(l) => match l.value {
                None => {
                    l.value = Some(value);
                    Ok(true)
                }
                _ => Err(SetError::AlreadySet),
            },
        }
    }
}

pub fn get_new_scorecard_data() -> ScoreCardData {
    let z: Vec<LineData> = vec![
        LineData {
            id: LineId::Ace,
            long_name: "Aces".to_string(),
            short_name: "1".to_string(),
            value: None,
            calc: calchand::calc_ace,
        },
        LineData {
            id: LineId::Two,
            long_name: "Twos".to_string(),
            short_name: "2".to_string(),
            value: None,
            calc: calchand::calc_two,
        },
        LineData {
            id: LineId::Three,
            long_name: "Threes".to_string(),
            short_name: "3".to_string(),
            value: None,
            calc: calchand::calc_three,
        },
        LineData {
            id: LineId::Four,
            long_name: "Fours".to_string(),
            short_name: "4".to_string(),
            value: None,
            calc: calchand::calc_four,
        },
        LineData {
            id: LineId::Five,
            long_name: "Fives".to_string(),
            short_name: "5".to_string(),
            value: None,
            calc: calchand::calc_five,
        },
        LineData {
            id: LineId::Six,
            long_name: "Sixes".to_string(),
            short_name: "6".to_string(),
            value: None,
            calc: calchand::calc_six,
        },
        LineData {
            id: LineId::ThreeKind,
            long_name: "Three of a Kind".to_string(),
            short_name: "3k".to_string(),
            value: None,
            calc: calchand::calc_3k,
        },
        LineData {
            id: LineId::FourKind,
            long_name: "Four of a Kind".to_string(),
            short_name: "4k".to_string(),
            value: None,
            calc: calchand::calc_4k,
        },
        LineData {
            id: LineId::SmallStraight,
            long_name: "Small Straight".to_string(),
            short_name: "ss".to_string(),
            value: None,
            calc: calchand::calc_ss,
        },
        LineData {
            id: LineId::LargeStraight,
            long_name: "Large Straight".to_string(),
            short_name: "ls".to_string(),
            value: None,
            calc: calchand::calc_ls,
        },
        LineData {
            id: LineId::FullHouse,
            long_name: "Full House".to_string(),
            short_name: "fh".to_string(),
            value: None,
            calc: calchand::calc_fh,
        },
        LineData {
            id: LineId::Chance,
            long_name: "Chance".to_string(),
            short_name: "c".to_string(),
            value: None,
            calc: calchand::calc_chance,
        },
        LineData {
            id: LineId::Dice5,
            long_name: "Dice 5".to_string(),
            short_name: "y".to_string(),
            value: None,
            calc: calchand::calc_dice5,
        },
    ];

    ScoreCardData {
        line: z,
        bonus_dice5: 0,
    }
}

#[cfg(test)]
mod tests {
    use super::LineId as L;
    use super::SetError as SErr;
    use super::*;

    #[test]
    fn get_new_scorecard_returns_card() {
        let scorecard = get_new_scorecard_data();
        let score = scorecard.by_id(L::Ace).value;
        assert_eq!(score, None);
    }

    #[test]
    fn set_score_short_name_exists_score_set() {
        let mut scorecard = get_new_scorecard_data();
        let points = 99;

        let line = scorecard.by_id(L::Ace);
        let result = scorecard.set_val(&line.short_name.clone(), points);
        match result {
            Err(SErr::NotFound) => {
                panic!("Not found shouldn't happen");
            }
            Err(SErr::AlreadySet) => {
                panic!("Already Set shoudln't happen");
            }
            Ok(_) => {
                let p = scorecard.by_id(L::Ace).value.unwrap();
                assert_eq!(p, points);
                assert!(true);
            }
        }
    }

    #[test]
    fn set_score_short_name_not_exists_err_set() {
        let mut scorecard = get_new_scorecard_data();
        let points = 99;

        let short_name = "ZZZZ".to_string();
        let result = scorecard.set_val(&short_name, points);
        match result {
            Err(SErr::NotFound) => {
                assert!(true);
            }
            Err(SErr::AlreadySet) => {
                panic!("Already Set shouldn't happen");
            }
            Ok(_) => {
                panic!("Value Set shouldn't happen");
            }
        }
    }

    #[test]
    fn set_score_short_name_exists_score_set_twice() {
        let mut scorecard = get_new_scorecard_data();
        let points1 = 99;
        let points2 = 32;

        let line = scorecard.by_id(L::Ace);
        let sname = line.short_name.clone();

        let result1 = scorecard.set_val(&sname, points1);
        match result1 {
            Err(SErr::NotFound) => {}
            Err(SErr::AlreadySet) => {}
            Ok(_) => {}
        }

        let result = scorecard.set_val(&sname, points2);
        match result {
            Err(SErr::NotFound) => {
                panic!("Not found shouldn't happen");
            }
            Err(SErr::AlreadySet) => {
                let p = scorecard.by_id(L::Ace).value.unwrap();
                assert_eq!(p, points1);
            }
            Ok(_) => {
                panic!("OK shouldn't happen");
            }
        }
    }
}
