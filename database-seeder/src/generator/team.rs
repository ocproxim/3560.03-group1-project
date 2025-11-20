use std::collections::HashSet;

use rand::{rngs::StdRng, seq::IndexedRandom};

use crate::models::{sport::Sport, team::Team};

const NAMES: &[&str] = &[
    "Eagles",
    "Tigers",
    "Hawks",
    "Lions",
    "Dragons",
    "Giants",
    "Sharks",
    "Wolverines",
    "Bears",
    "Stallions",
];

const HOMETOWNS: &[&str] = &[
    "Seattle",
    "Boston",
    "Miami",
    "Denver",
    "Chicago",
    "Los Angeles",
    "Phoenix",
    "Austin",
    "Portland",
    "Atlanta",
];

pub fn generate_teams(sports: &[Sport], teams_per_sport: i32, rng: &mut StdRng) -> Vec<Team> {
    let mut id = 0;
    sports
        .iter()
        .flat_map(|sport| {
            let mut used = HashSet::new();
            (0..teams_per_sport)
                .map(|_| {
                    let (hometown, name) = loop {
                        let hometown = HOMETOWNS.choose(rng).unwrap();
                        let name = NAMES.choose(rng).unwrap();

                        if !used.contains(name) && !used.contains(hometown) {
                            used.insert(hometown);

                            used.insert(name);
                            break (hometown, name);
                        }
                    };

                    let team = Team {
                        teamID: id,
                        sportID: sport.sportID,
                        teamName: name.to_string(),
                        homeTown: hometown.to_string(),
                    };
                    id += 1;
                    team
                })
                .collect::<Vec<_>>()
        })
        .collect()
}

#[cfg(test)]
mod test {
    use rand::{SeedableRng, rngs::StdRng};

    use crate::generator::{RNG_SEED, sport::generate_sports, team::generate_teams};

    #[test]
    fn gen_10_teams() {
        let mut rng = StdRng::seed_from_u64(RNG_SEED);
        let sports = generate_sports();
        let teams = generate_teams(&sports, 4, &mut rng);

        for team in teams {
            println!("{team:?}");
        }
    }
}
