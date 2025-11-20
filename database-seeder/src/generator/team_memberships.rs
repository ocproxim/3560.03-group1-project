use std::collections::{HashMap, HashSet};

use rand::{Rng, rngs::StdRng, seq::IndexedRandom};

use crate::models::{
    ids::TeamId,
    player::Player,
    team::{Team, TeamMembership},
};

pub fn generate_team_memberships(
    players: &[Player],
    teams: &[Team],
    players_per_team: i32,
    rng: &mut StdRng,
) -> Vec<TeamMembership> {
    let mut open_teams: Vec<TeamId> = teams.iter().map(|team| team.teamID).collect();

    let mut used_jersey_numbers: HashMap<TeamId, HashSet<u32>> = HashMap::new();
    for team in teams {
        used_jersey_numbers.insert(team.teamID, HashSet::default());
    }

    let season = "2025";
    let mut id = 0;

    players
        .iter()
        .map(|player| {
            let found_id = loop {
                let candidate = open_teams.choose(rng).unwrap();
                if used_jersey_numbers.get(candidate).unwrap().len() >= players_per_team as usize {
                    //team is full, remove it as an option
                    let team_index = open_teams.iter().position(|id| id == candidate).unwrap();
                    let _ = open_teams.remove(team_index);
                    continue;
                }
                break candidate;
            };

            let jersey_number = loop {
                let candidate: u32 = rng.random_range(0..99);
                if !used_jersey_numbers
                    .get(found_id)
                    .unwrap()
                    .contains(&candidate)
                {
                    break candidate;
                }
            };

            let membership = TeamMembership {
                membershipID: id,
                teamID: *found_id,
                playerID: player.playerID,
                season: season.to_string(),
                jerseyNumber: jersey_number as i32,
            };
            id += 1;
            membership
        })
        .collect()
}

#[cfg(test)]
mod test {
    use rand::{SeedableRng, rngs::StdRng};

    use crate::generator::{
        RNG_SEED, player::generate_players, sport::generate_sports, team::generate_teams,
        team_memberships::generate_team_memberships,
    };

    #[test]
    fn distribute_players() {
        let sports = generate_sports();
        let teams_per_sport = 4_i32;
        let num_teams = (sports.len() as i32 * teams_per_sport) as i32;
        let players_per_team = 15_i32;
        let num_players = num_teams * players_per_team;

        let mut rng = StdRng::seed_from_u64(RNG_SEED);
        let players = generate_players(num_players, &mut rng);
        let teams = generate_teams(&sports, 2, &mut rng);
        let memberships = generate_team_memberships(&players, &teams, players_per_team, &mut rng);

        for membership in memberships {
            let player = players
                .iter()
                .find(|player| player.playerID == membership.playerID)
                .unwrap();
            let team = teams
                .iter()
                .find(|team| team.teamID == membership.teamID)
                .unwrap();
            let sport = sports
                .iter()
                .find(|sport| sport.sportID == team.sportID)
                .unwrap();

            println!("-----------------------------------------------------");
            println!("Team Membership: {:?}", membership.membershipID);
            println!(
                "Team: {} {}, Sport: {}",
                team.homeTown, team.teamName, sport.sportName
            );
            println!(
                "Player: {}, Jersey: {}",
                player.name, membership.jerseyNumber
            );
        }

        println!("-----------------------------------------------------");
    }
}
