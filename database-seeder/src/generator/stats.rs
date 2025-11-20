use crate::models::{
    game::Game,
    ids::SportId,
    player::Player,
    sport::Sport,
    stat::{StatInstance, StatKind},
};

pub fn generate_stat_kinds(sports: &[crate::models::sport::Sport]) -> Vec<StatKind> {
    let mut stat_kinds = Vec::new();
    let mut stat_id = 0;

    for sport in sports {
        let kinds = match sport.sportName.as_str() {
            "Basketball" => generate_basketball_stats(&mut stat_id, &sport.sportID),
            "Soccer" => generate_soccer_stats(&mut stat_id, &sport.sportID),
            "Baseball" => generate_baseball_stats(&mut stat_id, &sport.sportID),
            _ => vec![],
        };
        stat_kinds.extend(kinds);
    }

    stat_kinds
}

fn generate_basketball_stats(stat_id: &mut i32, sport_id: &SportId) -> Vec<StatKind> {
    vec![
        create_stat(stat_id, sport_id, "Assists", "count"),
        create_stat(stat_id, sport_id, "Steals", "count"),
        create_stat(stat_id, sport_id, "Blocks", "count"),
        create_stat(stat_id, sport_id, "Three Pointers Attempted", "count"),
        create_stat(stat_id, sport_id, "Free Throws Attempted", "count"),
    ]
}

fn generate_soccer_stats(stat_id: &mut i32, sport_id: &SportId) -> Vec<StatKind> {
    vec![
        create_stat(stat_id, sport_id, "Assists", "count"),
        create_stat(stat_id, sport_id, "Shots", "count"),
        create_stat(stat_id, sport_id, "Shots on Target", "count"),
        create_stat(stat_id, sport_id, "Passes", "count"),
        create_stat(
            stat_id,
            sport_id,
            "Pass Completion Percentage",
            "percentage",
        ),
        create_stat(stat_id, sport_id, "Tackles", "count"),
        create_stat(stat_id, sport_id, "Interceptions", "count"),
        create_stat(stat_id, sport_id, "Fouls", "count"),
        create_stat(stat_id, sport_id, "Yellow Cards", "count"),
    ]
}

fn generate_baseball_stats(stat_id: &mut i32, sport_id: &SportId) -> Vec<StatKind> {
    vec![
        create_stat(stat_id, sport_id, "At Bats", "count"),
        create_stat(stat_id, sport_id, "Strikeouts", "count"),
        create_stat(stat_id, sport_id, "Walks", "count"),
        create_stat(stat_id, sport_id, "Doubles", "count"),
        create_stat(stat_id, sport_id, "Triples", "count"),
        create_stat(stat_id, sport_id, "Stolen Bases", "count"),
    ]
}

fn create_stat(stat_id: &mut i32, sport_id: &SportId, stat_name: &str, unit: &str) -> StatKind {
    let kind = StatKind {
        statKindID: *stat_id,
        sportID: *sport_id,
        statName: stat_name.to_string(),
        unit: unit.to_string(),
    };
    *stat_id += 1;
    kind
}
use rand::{Rng, rngs::StdRng};

pub fn generate_stat_instances(
    stat_kinds: &[StatKind],
    stat_instance_id: &mut u32,
    game: &Game,
    player: &Player,
    sport: &Sport,
    timestamp: String,
    rng: &mut StdRng,
) -> Vec<StatInstance> {
    let sport_stats: Vec<_> = stat_kinds
        .iter()
        .filter(|sk| sk.sportID == sport.sportID)
        .collect();

    sport_stats
        .iter()
        .map(|stat_kind| {
            let value = generate_stat_value(&stat_kind.statName, rng);
            let instance = StatInstance {
                statInstanceID: *stat_instance_id as i32,
                gameID: game.gameID,
                playerID: player.playerID,
                statKindID: stat_kind.statKindID,
                timestamp: timestamp.clone(),
                value,
            };
            *stat_instance_id += 1;
            instance
        })
        .collect()
}

fn generate_stat_value(stat_name: &str, rng: &mut StdRng) -> f32 {
    match stat_name {
        // Basketball stats
        "Rebounds" => rng.random_range(0.0f32..15.0).round(),
        "Assists" => rng.random_range(0.0f32..12.0).round(),
        "Steals" => rng.random_range(0.0f32..5.0).round(),
        "Blocks" => rng.random_range(0.0f32..5.0).round(),

        // Soccer stats
        "Shots" => rng.random_range(0.0f32..8.0).round(),
        "Shots on Target" => rng.random_range(0.0f32..5.0).round(),
        "Passes" => rng.random_range(20.0f32..100.0).round(),
        "Pass Completion Percentage" => rng.random_range(60.0f32..95.0).round(),
        "Tackles" => rng.random_range(0.0f32..8.0).round(),
        "Interceptions" => rng.random_range(0.0f32..5.0).round(),
        "Fouls" => rng.random_range(0.0f32..4.0).round(),
        "Yellow Cards" => rng.random_range(0.0f32..2.0).round(),

        // Baseball stats
        "Hits" => rng.random_range(0.0f32..4.0).round(),
        "At Bats" => rng.random_range(3.0f32..5.0).round(),
        "Strikeouts" => rng.random_range(0.0f32..3.0).round(),
        "Walks" => rng.random_range(0.0f32..2.0).round(),
        "Doubles" => rng.random_range(0.0f32..2.0).round(),
        "Triples" => rng.random_range(0.0f32..1.0).round(),
        "Stolen Bases" => rng.random_range(0.0f32..2.0).round(),

        _ => rng.random_range(0.0f32..10.0).round(),
    }
}

#[cfg(test)]
mod test {

    use rand::{SeedableRng, rngs::StdRng};

    use crate::generator::{
        RNG_SEED,
        game::generate_games,
        player::generate_players,
        sport::generate_sports,
        stats::{generate_stat_instances, generate_stat_kinds},
        team::generate_teams,
        team_memberships::generate_team_memberships,
    };

    #[test]
    fn gen_stat_kinds() {
        let sports = generate_sports();
        let stat_kinds = generate_stat_kinds(&sports);

        for stat_kind in stat_kinds {
            println!("{stat_kind:?}");
        }
    }

    #[test]
    fn gen_stat_instances() {
        let mut rng = StdRng::seed_from_u64(RNG_SEED);
        let sports = generate_sports();
        let players_per_team = 15;
        let teams_per_sport = 4;

        let teams = generate_teams(&sports, teams_per_sport, &mut rng);

        let players = generate_players(
            sports.len() as i32 * teams_per_sport * players_per_team,
            &mut rng,
        );
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

        let team_memberships =
            generate_team_memberships(&players, &teams, players_per_team, &mut rng);

        let stat_kinds = generate_stat_kinds(&sports);
        let mut stat_instance_id = 0;

        let stat_instances = games
            .iter()
            .flat_map(|game| {
                let home_team = teams
                    .iter()
                    .find(|team| team.teamID == game.homeTeamID)
                    .unwrap();
                let away_team = teams
                    .iter()
                    .find(|team| team.teamID == game.awayTeamID)
                    .unwrap();
                let sport = sports
                    .iter()
                    .find(|sport| home_team.sportID == sport.sportID)
                    .unwrap();

                let mut involved_players = team_memberships
                    .iter()
                    .filter(|m| m.teamID == home_team.teamID)
                    .map(|membership| {
                        players
                            .iter()
                            .find(|p| p.playerID == membership.playerID)
                            .unwrap()
                    })
                    .cloned()
                    .collect::<Vec<_>>();
                let away_players = team_memberships
                    .iter()
                    .filter(|m| m.teamID == away_team.teamID)
                    .map(|membership| {
                        players
                            .iter()
                            .find(|p| p.playerID == membership.playerID)
                            .unwrap()
                    })
                    .cloned()
                    .collect::<Vec<_>>();

                involved_players.extend(away_players);

                let applicable_stats = stat_kinds
                    .iter()
                    .filter(|s| s.sportID == sport.sportID)
                    .cloned()
                    .collect::<Vec<_>>();

                players
                    .iter()
                    .flat_map(|player| {
                        generate_stat_instances(
                            &applicable_stats,
                            &mut stat_instance_id,
                            game,
                            player,
                            sport,
                            game.gameTime.clone(),
                            &mut rng,
                        )
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        for instance in stat_instances {
            let stat = stat_kinds
                .iter()
                .find(|s| s.statKindID == instance.statKindID)
                .unwrap();
            println!("stat: {}, value: {}", stat.statName, instance.value);
        }
    }
}
