use jiff::civil::Date;
use rand::{Rng, rngs::StdRng, seq::IndexedRandom};

use crate::models::{game::Game, team::Team};

pub fn generate_games(
    teams: &[Team],
    game_id: &mut i32,
    num_games: u32,
    rng: &mut StdRng,
) -> Vec<Game> {
    (0..num_games)
        .map(|_| {
            let home_team = teams.choose(rng).unwrap();
            let away_team = loop {
                let candidate = teams.choose(rng).unwrap();
                if candidate.teamID != home_team.teamID {
                    break candidate;
                }
            };

            let game_date = Date::new(2025, 3, 15).unwrap();
            let home_score = rng.random_range(50.0f32..120.0).round();
            let away_score = rng.random_range(50.0f32..120.0).round();

            let game = Game {
                gameID: *game_id,
                homeTeamID: home_team.teamID,
                awayTeamID: away_team.teamID,
                homeScore: home_score,
                awayScore: away_score,
                gameTime: game_date.to_string(),
                venue: format!("{} Arena", home_team.homeTown),
            };
            *game_id += 1;
            game
        })
        .collect()
}

#[cfg(test)]
mod test {
    use rand::{SeedableRng, rngs::StdRng};

    use crate::generator::{
        RNG_SEED, game::generate_games, sport::generate_sports, team::generate_teams,
    };

    #[test]
    fn gen_games() {
        let mut rng = StdRng::seed_from_u64(RNG_SEED);
        let sports = generate_sports();
        let teams_per_sport = 4;

        let teams = generate_teams(&sports, teams_per_sport, &mut rng);
        let mut game_id = 0;

        let games = sports
            .iter()
            .flat_map(|sport| {
                let teams = teams
                    .iter()
                    .filter(|team| team.sportID == sport.sportID)
                    .cloned()
                    .collect::<Vec<_>>();
                generate_games(&teams, &mut game_id, 30, &mut rng)
            })
            .collect::<Vec<_>>();

        for game in games {
            println!("{game:?}");
        }
    }
}
