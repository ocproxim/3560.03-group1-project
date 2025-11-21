use crate::models::{game::Game, player::Player, sport::Sport, stat::StatKind};

pub struct KeeperPage {
    games: Vec<Game>,
    stat_kinds: Vec<StatKind>,
    sports: Vec<Sport>,
}
