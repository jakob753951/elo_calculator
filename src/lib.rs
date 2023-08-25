pub enum GameResult {
    Win,
    Draw,
    Loss,
}

impl GameResult {
    pub fn get_points(&self) -> f32 {
        match self {
            GameResult::Win => 1.0,
            GameResult::Draw => 0.5,
            GameResult::Loss => 0.0,
        }
    }
}

pub enum Experience {
    New,
    Intermediate,
    Expert,
}

// https://ratings.fide.com/calc.phtml
impl Experience {
    pub fn get_experience(age: u32, games_played: u32, highest_rating: u32) -> Self {
        if (age < 18 && highest_rating < 2300) || games_played < 30 {
            Experience::New
        } else if highest_rating < 2400 {
            Experience::Intermediate
        } else {
            Experience::Expert
        }
    }

    pub fn get_development_coefficient(&self) -> u32 {
        match self {
            Experience::New => 40,
            Experience::Intermediate => 20,
            Experience::Expert => 10,
        }
    }
}

// https://en.wikipedia.org/wiki/Elo_rating_system
pub fn calc_delta_rating(player_rating: f32, opponent_rating: f32, result: GameResult, experience: Experience) -> f32 {
    let development_coefficient = experience.get_development_coefficient() as f32;
    let score = result.get_points();
    let expected_score = get_expected_result(player_rating, opponent_rating);
    development_coefficient * (score - expected_score)
}

pub fn get_expected_result(player_rating: f32, opponent_rating: f32) -> f32 {
    1.0 / (1.0 + 10_f32.powf((opponent_rating - player_rating) / 400.0))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn magnus_v_nepo() {
        // https://ratings.fide.com/calc.phtml?page=change
        let result = calc_delta_rating(2793.0, 2852.0, GameResult::Win, Experience::Expert);
        assert_eq!((result*10.0).round()/10.0, 5.8);
    }

    #[test]
    fn lose_2000_v_1000_not_new() {
        // https://ratings.fide.com/calc.phtml?page=change
        let result = calc_delta_rating(2000.0, 1000.0, GameResult::Loss, Experience::Intermediate);
        assert_eq!((result*10.0).round()/10.0, -18.4);
    }
}
