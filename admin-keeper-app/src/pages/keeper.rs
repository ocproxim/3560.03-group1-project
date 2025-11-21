use diesel::SqliteConnection;

use crate::{
    models::{game::Game, player::Player, sport::Sport, stat::StatKind},
    pages::Page,
};

pub struct KeeperPage {
    games: Vec<Game>,
    stat_kinds: Vec<StatKind>,
    sports: Vec<Sport>,
}

impl KeeperPage {
    pub fn new(db: &mut SqliteConnection) -> Self {
        todo!()
    }
}

impl Page for KeeperPage {
    fn show(
        &mut self,
        ctx: &egui::Context,
        db: &mut diesel::SqliteConnection,
    ) -> Option<super::StateTransition> {
        todo!()
    }
}
