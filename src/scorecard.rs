pub struct ScoreCard {
    pub ace: Option<i16>,
    pub two: Option<i16>,
    pub three: Option<i16>,
    pub four: Option<i16>,
    pub five: Option<i16>,
    pub six: Option<i16>,
    pub three_kind: Option<i16>,
    pub four_kind: Option<i16>,
    pub small_straight: Option<i16>,
    pub large_straight: Option<i16>,
    pub full_house: Option<i16>,
    pub chance: Option<i16>,
    pub yahtzee: Option<i16>,
}

pub fn get_new_scorecard() -> ScoreCard {
    let card = ScoreCard {
        ace: None,
        two: None,
        three: None,
        four: None,
        five: None,
        six: None,
        three_kind: None,
        four_kind: None,
        small_straight: None,
        large_straight: None,
        full_house: None,
        chance: None,
        yahtzee: None,
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
        let mut scorecard = get_new_scorecard();
        assert_eq!(scorecard.ace, None);

        scorecard.five = Some(10);
    }

}
