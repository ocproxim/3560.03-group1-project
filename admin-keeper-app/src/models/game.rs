use std::str::FromStr;

use diesel::prelude::*;
use jiff::civil::DateTime;

use crate::models::ids::*;
#[derive(Debug, Clone, Insertable, Queryable, Identifiable, AsChangeset)]
#[diesel(table_name = crate::schema::Games)]
#[diesel(check_for_backend(diesel::sqlite::SQLite))]
#[diesel(primary_key(gameID))]
pub struct Game {
    pub gameID: GameId,
    pub homeTeamID: TeamId,
    pub awayTeamID: TeamId,
    pub homeScore: f32,
    pub awayScore: f32,
    pub gameTime: String,
    pub venue: String,
}

impl Game {
    pub fn get_home_score(&self) -> f32 {
        self.homeScore
    }

    pub fn get_away_score(&self) -> f32 {
        self.awayScore
    }

    pub fn get_game_time(&self) -> DateTime {
        DateTime::from_str(&self.gameTime).unwrap()
    }

    pub fn get_venue(&self) -> &str {
        &self.venue
    }

    pub fn set_home_score(&mut self, new_home_score: f32) {
        self.homeScore = new_home_score;
    }

    pub fn set_away_score(&mut self, new_away_score: f32) {
        self.awayScore = new_away_score;
    }

    pub fn set_game_time(&mut self, new_game_time: DateTime) {
        self.gameTime = new_game_time.to_string();
    }

    pub fn set_venue(&mut self, new_venue: String) {
        self.venue = new_venue;
    }
}
