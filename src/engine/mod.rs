mod calchand;
mod hand;

pub use hand::{Dice, DieFace};
use std::collections::HashMap;
use std::fmt;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum SetError {
    AlreadySet,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
pub enum LineId {
    Ace,
    Two,
    Three,
    Four,
    Five,
    Six,
    UpperSubtotal,
    UpperBonus,
    UpperTotal,

    ThreeKind,
    FourKind,
    SmallStraight,
    LargeStraight,
    FullHouse,
    Chance,
    Dice5,
    BottomSubtotal,
    Dice5Bonus,
    GrandTotal,
}

// #[derive(Debug)]
pub struct LineData {
    pub id: LineId,
    pub value: Option<i16>,
    pub calc: fn(dice: &Dice, special_dice: bool) -> i16,
}

// #[derive(Debug)]
pub struct SubtotalData {
    pub id: LineId,
    pub calc: fn(scorecard: &ScoreCardData) -> i16,
}

pub enum Data {
    Line(LineData),
    Subtotal(SubtotalData),
}

impl fmt::Display for LineData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?} ", self.id)?;

        if let Some(val) = self.value {
            write!(f, "{:>5}", val)
        } else {
            write!(f, "")
        }
    }
}

pub struct ScoreCardData {
    pub l2: HashMap<LineId, Data>,
    pub bonus_dice5: i8,
}

impl fmt::Display for ScoreCardData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let out: Vec<_> = vec![
            format!("{}", self.get_line_by_id(LineId::Ace)),
            format!("{}", self.get_line_by_id(LineId::Two)),
            format!("{}", self.get_line_by_id(LineId::Three)),
            format!("{}", self.get_line_by_id(LineId::Four)),
            format!("{}", self.get_line_by_id(LineId::Five)),
            format!("{}", self.get_line_by_id(LineId::Six)),
            "-------------------------".to_string(),
            format!("{}", self.get_line_by_id(LineId::ThreeKind)),
            format!("{}", self.get_line_by_id(LineId::FourKind)),
            format!("{}", self.get_line_by_id(LineId::SmallStraight)),
            format!("{}", self.get_line_by_id(LineId::LargeStraight)),
            format!("{}", self.get_line_by_id(LineId::FullHouse)),
            format!("{}", self.get_line_by_id(LineId::Chance)),
            format!("{}", self.get_line_by_id(LineId::Dice5)),
        ];

        write!(f, "{}", out.join("\n"))
    }
}

impl ScoreCardData {
    pub fn get_line_by_id(&self, zid: LineId) -> &LineData {
        if let Some(Data::Line(line_data)) = self.l2.get(&zid) {
            &line_data
        } else {
            panic!("Not found");
        }
    }

    pub fn get_subtotal_by_id(&self, zid: LineId) -> &SubtotalData {
        if let Some(Data::Subtotal(line_data)) = self.l2.get(&zid) {
            line_data
        } else {
            panic!("Not found");
        }
    }

    pub fn play(&mut self, zid: LineId, hand: &Dice) -> Result<i16, SetError> {
        let already_has_dice5 = self.get_line_by_id(LineId::Dice5).value.is_some();
        let is_dice5 = calchand::is_dice5(hand);
        let special_handling = already_has_dice5 && is_dice5;

        let point_result = self.get_points(zid, &hand, special_handling);

        if let Ok(points) = point_result {
            self.set_val(zid, points)?;
            if special_handling {
                self.bonus_dice5 += 1;
            }
        }
        point_result
    }

    pub fn set_val(&mut self, zid: LineId, value: i16) -> Result<(), SetError> {
        if let Some(Data::Line(line_data)) = self.l2.get_mut(&zid) {
            match line_data.value {
                None => {
                    line_data.value = Some(value);
                    Ok(())
                }
                _ => Err(SetError::AlreadySet),
            }
        } else {
            panic!("Not found");
        }
    }

    pub fn get_points(
        &mut self,
        zid: LineId,
        hand: &Dice,
        dice5_bonus: bool,
    ) -> Result<i16, SetError> {
        let line = self.get_line_by_id(zid);
        Ok((line.calc)(&hand, dice5_bonus))
    }

    pub fn game_over(&mut self) -> bool {
        false
        // self.line.iter().all(|l| l.value.is_some())
    }
}

fn calc_subtotal(scorecard: &ScoreCardData, a: Vec<LineId>) -> i16 {
    a.iter()
        .flat_map(|&line_id| scorecard.get_line_by_id(line_id).value)
        .sum()
}

fn calc_upper_subtotal(scorecard: &ScoreCardData) -> i16 {
    let a = vec![
        LineId::Ace,
        LineId::Two,
        LineId::Three,
        LineId::Four,
        LineId::Five,
        LineId::Six,
    ];
    calc_subtotal(&scorecard, a)
}

fn calc_upper_bonus(scorecard: &ScoreCardData) -> i16 {
    let upper_score = calc_upper_subtotal(scorecard);
    if upper_score >= 63 {
        35
    } else {
        0
    }
}

fn calc_upper_total(scorecard: &ScoreCardData) -> i16 {
    calc_upper_subtotal(scorecard) + calc_upper_bonus(scorecard)
}

fn calc_lower_subtotal(scorecard: &ScoreCardData) -> i16 {
    let a = vec![
        LineId::ThreeKind,
        LineId::FourKind,
        LineId::SmallStraight,
        LineId::LargeStraight,
        LineId::FullHouse,
        LineId::Chance,
        LineId::Dice5,
    ];
    calc_subtotal(&scorecard, a)
}

fn calc_dice5_bonus(scorecard: &ScoreCardData) -> i16 {
    i16::from(scorecard.bonus_dice5) * 100
}

fn calc_grand_total(scorecard: &ScoreCardData) -> i16 {
    calc_upper_subtotal(scorecard) + calc_lower_subtotal(scorecard) + calc_dice5_bonus(scorecard)
}

pub fn get_new_scorecard_data() -> ScoreCardData {
    let mut v = HashMap::new();

    v.insert(
        LineId::Ace,
        Data::Line(LineData {
            id: LineId::Ace,
            value: None,
            calc: calchand::calc_ace,
        }),
    );

    v.insert(
        LineId::Two,
        Data::Line(LineData {
            id: LineId::Two,
            value: None,
            calc: calchand::calc_two,
        }),
    );

    v.insert(
        LineId::Three,
        Data::Line(LineData {
            id: LineId::Three,
            value: None,
            calc: calchand::calc_three,
        }),
    );

    v.insert(
        LineId::Four,
        Data::Line(LineData {
            id: LineId::Four,
            value: None,
            calc: calchand::calc_four,
        }),
    );

    v.insert(
        LineId::Five,
        Data::Line(LineData {
            id: LineId::Five,
            value: None,
            calc: calchand::calc_five,
        }),
    );

    v.insert(
        LineId::Six,
        Data::Line(LineData {
            id: LineId::Six,
            value: None,
            calc: calchand::calc_six,
        }),
    );

    v.insert(
        LineId::ThreeKind,
        Data::Line(LineData {
            id: LineId::ThreeKind,
            value: None,
            calc: calchand::calc_3k,
        }),
    );

    v.insert(
        LineId::FourKind,
        Data::Line(LineData {
            id: LineId::FourKind,
            value: None,
            calc: calchand::calc_4k,
        }),
    );

    v.insert(
        LineId::SmallStraight,
        Data::Line(LineData {
            id: LineId::SmallStraight,
            value: None,
            calc: calchand::calc_ss,
        }),
    );

    v.insert(
        LineId::LargeStraight,
        Data::Line(LineData {
            id: LineId::LargeStraight,
            value: None,
            calc: calchand::calc_ls,
        }),
    );

    v.insert(
        LineId::FullHouse,
        Data::Line(LineData {
            id: LineId::FullHouse,
            value: None,
            calc: calchand::calc_fh,
        }),
    );

    v.insert(
        LineId::Chance,
        Data::Line(LineData {
            id: LineId::Chance,
            value: None,
            calc: calchand::calc_chance,
        }),
    );

    v.insert(
        LineId::Dice5,
        Data::Line(LineData {
            id: LineId::Dice5,
            value: None,
            calc: calchand::calc_dice5,
        }),
    );

    v.insert(
        LineId::UpperSubtotal,
        Data::Subtotal(SubtotalData {
            id: LineId::UpperSubtotal,
            calc: calc_upper_subtotal,
        }),
    );

    v.insert(
        LineId::UpperSubtotal,
        Data::Subtotal(SubtotalData {
            id: LineId::UpperSubtotal,
            calc: calc_upper_subtotal,
        }),
    );

    v.insert(
        LineId::UpperSubtotal,
        Data::Subtotal(SubtotalData {
            id: LineId::UpperBonus,
            calc: calc_upper_bonus,
        }),
    );

    v.insert(
        LineId::UpperSubtotal,
        Data::Subtotal(SubtotalData {
            id: LineId::UpperTotal,
            calc: calc_upper_total,
        }),
    );

    v.insert(
        LineId::UpperSubtotal,
        Data::Subtotal(SubtotalData {
            id: LineId::BottomSubtotal,
            calc: calc_lower_subtotal,
        }),
    );

    v.insert(
        LineId::UpperSubtotal,
        Data::Subtotal(SubtotalData {
            id: LineId::Dice5Bonus,
            calc: calc_dice5_bonus,
        }),
    );

    v.insert(
        LineId::UpperSubtotal,
        Data::Subtotal(SubtotalData {
            id: LineId::GrandTotal,
            calc: calc_grand_total,
        }),
    );

    ScoreCardData {
        l2: v,
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
        let score = scorecard.get_line_by_id(L::Ace).value;
        assert_eq!(score, None);
    }

    #[test]
    fn set_score_short_name_exists_score_set() {
        let mut scorecard = get_new_scorecard_data();
        let points = 99;

        let result = scorecard.set_val(L::Ace, points);
        match result {
            Err(SErr::AlreadySet) => {
                panic!("Already Set shoudln't happen");
            }
            Ok(_) => {
                let p = scorecard.get_line_by_id(L::Ace).value.unwrap();
                assert_eq!(p, points);
                assert!(true);
            }
        }
    }

    #[test]
    fn get_points() {
        let mut scorecard = get_new_scorecard_data();
        let dice = Dice::first_roll();

        let result = scorecard.get_points(L::Chance, &dice, false);

        match result {
            Err(SErr::AlreadySet) => {
                panic!("Already Set shoudln't happen");
            }
            Ok(points) => {
                assert_ne!(points, 0);
            }
        }
    }

    #[test]
    fn set_score_short_name_exists_score_set_twice() {
        let mut scorecard = get_new_scorecard_data();
        let points1 = 99;
        let points2 = 32;

        let result1 = scorecard.set_val(L::Ace, points1);
        match result1 {
            Err(SErr::AlreadySet) => {}
            Ok(_) => {}
        }

        let result = scorecard.set_val(L::Ace, points2);
        match result {
            Err(SErr::AlreadySet) => {
                let p = scorecard.get_line_by_id(L::Ace).value.unwrap();
                assert_eq!(p, points1);
            }
            Ok(_) => {
                panic!("OK shouldn't happen");
            }
        }
    }

    #[test]
    fn game_over_new_game() {
        let mut scorecard = get_new_scorecard_data();

        assert_eq!(false, scorecard.game_over());
    }

    //    #[test]
    //    fn game_over_game_over() {
    //        let mut scorecard = get_new_scorecard_data();
    //        for i in 0..scorecard.line.len() {
    //            scorecard.line[i].value = Some(4);
    //        }

    //        assert_eq!(true, scorecard.game_over());
    //    }
}
