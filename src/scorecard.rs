#[derive(Debug)]
pub struct LineData {
    long_name: String,
    short_name: String,
    value: Option<i16>,
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
}

pub fn get_new_scorecard_data() -> ScoreCardData {
    let card = ScoreCardData {
        ace: LineData {
            long_name: "Aces".to_string(),
            short_name: "1".to_string(),
            value: None,
        },
        two: LineData {
            long_name: "Twos".to_string(),
            short_name: "2".to_string(),
            value: None,
        },
        three: LineData {
            long_name: "Threes".to_string(),
            short_name: "3".to_string(),
            value: None,
        },
        four: LineData {
            long_name: "Fours".to_string(),
            short_name: "4".to_string(),
            value: None,
        },
        five: LineData {
            long_name: "Fives".to_string(),
            short_name: "5".to_string(),
            value: None,
        },
        six: LineData {
            long_name: "Sixes".to_string(),
            short_name: "6".to_string(),
            value: None,
        },
        three_kind: LineData {
            long_name: "Three of a Kind".to_string(),
            short_name: "3k".to_string(),
            value: None,
        },
        four_kind: LineData {
            long_name: "Four of a Kind".to_string(),
            short_name: "4k".to_string(),
            value: None,
        },
        small_straight: LineData {
            long_name: "Small Straight".to_string(),
            short_name: "ss".to_string(),
            value: None,
        },
        large_straight: LineData {
            long_name: "Large Straight".to_string(),
            short_name: "ls".to_string(),
            value: None,
        },
        full_house: LineData {
            long_name: "Full House".to_string(),
            short_name: "fh".to_string(),
            value: None,
        },
        chance: LineData {
            long_name: "Chance".to_string(),
            short_name: "c".to_string(),
            value: None,
        },
        yahtzee: LineData {
            long_name: "Yahtzee".to_string(),
            short_name: "y".to_string(),
            value: None,
        },
    };
    card
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_test() {
        assert_eq!(true, true);
    }

    #[test]
    fn get_new_scorecard_returns_card() {
        let scorecard = get_new_scorecard_data();
        assert_eq!(scorecard.ace.value, None);
    }
}
