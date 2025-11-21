use jiff::{Span, civil::Date};
use rand::{Rng, rngs::StdRng, seq::IndexedRandom};

use crate::models::player::Player;

pub fn generate_players(num: i32, rng: &mut StdRng) -> Vec<Player> {
    (0..num).map(|i| generate_random_player(i, rng)).collect()
}

const FIRST_NAMES: &[&str] = &[
    "James",
    "Mary",
    "John",
    "Patricia",
    "Robert",
    "Jennifer",
    "Michael",
    "Linda",
    "David",
    "Elizabeth",
    "William",
    "Barbara",
    "Richard",
    "Susan",
    "Joseph",
    "Jessica",
    "Thomas",
    "Sarah",
    "Charles",
    "Karen",
    "Daniel",
    "Nancy",
    "Matthew",
    "Lisa",
];

const LAST_NAMES: &[&str] = &[
    "Smith",
    "Johnson",
    "Williams",
    "Brown",
    "Jones",
    "Garcia",
    "Miller",
    "Davis",
    "Rodriguez",
    "Martinez",
    "Hernandez",
    "Lopez",
    "Gonzalez",
    "Wilson",
    "Anderson",
    "Thomas",
    "Taylor",
    "Moore",
    "Jackson",
    "Martin",
    "Lee",
    "Perez",
    "Thompson",
    "White",
];

fn generate_random_player(id: i32, mut rng: &mut StdRng) -> Player {
    let date = Date::new(2025, 1, 1).unwrap();
    let years_to_subtract = rng.random_range(18..=40);
    let days_to_subtract = rng.random_range(0..365);

    let random_period = Span::new().years(years_to_subtract).days(days_to_subtract);

    let date_of_birth = date
        .checked_sub(random_period)
        .expect("Failed to subtract period from datetime");

    let player_id = id;

    let first_name = FIRST_NAMES.choose(&mut rng).unwrap();
    let last_name = LAST_NAMES.choose(&mut rng).unwrap();

    let name = format!("{} {}", first_name, last_name);
    let height = rng.random_range(170..=210);
    let weight = rng.random_range(70..=110);

    Player {
        playerID: player_id,
        name,
        dateOfBirth: date_of_birth.to_string(),
        height,
        weight,
    }
}

#[cfg(test)]
mod test {
    use rand::{SeedableRng as _, rngs::StdRng};

    use crate::generator::{RNG_SEED, player::generate_players};

    #[test]
    fn gen_10_players() {
        let mut rng = StdRng::seed_from_u64(RNG_SEED);
        let players = generate_players(10, &mut rng);
        for player in players {
            println!("{player:?}");
        }
    }
}
