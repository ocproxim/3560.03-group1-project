use rand::rngs::StdRng;

use crate::{
    generator::{
        game::generate_games,
        player::generate_players,
        sport::generate_sports,
        stats::{generate_stat_instances, generate_stat_kinds},
        team_memberships::generate_team_memberships,
        user::generate_users,
    },
    models::{
        game::Game,
        player::Player,
        sport::Sport,
        stat::{StatInstance, StatKind},
        team::{Team, TeamMembership},
        user::User,
    },
};

pub mod game;
pub mod player;
pub mod sport;
pub mod stats;
pub mod team;
pub mod team_memberships;
pub mod user;

pub struct SeededDB {
    pub users: Vec<User>,
    pub sports: Vec<Sport>,
    pub players: Vec<Player>,
    pub teams: Vec<Team>,
    pub memberships: Vec<TeamMembership>,
    pub games: Vec<Game>,
    pub stat_kinds: Vec<StatKind>,
    pub stat_instances: Vec<StatInstance>,
}

pub const RNG_SEED: u64 = 15309156293440041409;

pub fn generate_db_data() -> SeededDB {
    let mut rng = <StdRng as rand::SeedableRng>::seed_from_u64(RNG_SEED);
    let sports = generate_sports();
    let players_per_team = 15;
    let teams_per_sport = 4i32;
    let games_per_sport = 40;

    let teams = team::generate_teams(&sports, teams_per_sport, &mut rng);

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
            generate_games(&teams, &mut game_id, games_per_sport, &mut rng)
        })
        .collect::<Vec<_>>();

    let team_memberships = generate_team_memberships(&players, &teams, players_per_team, &mut rng);

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

            involved_players
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

    let users = generate_users();

    SeededDB {
        users,
        sports,
        players,
        teams,
        memberships: team_memberships,
        games,
        stat_kinds,
        stat_instances,
    }
}
