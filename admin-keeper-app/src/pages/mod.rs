use diesel::SqliteConnection;

use crate::app::AppState;

pub mod admin;
pub mod keeper;
pub mod login;

pub type StateTransition = Box<dyn FnOnce(&mut AppState) + 'static>;
pub trait Page {
    fn show(&mut self, ctx: &egui::Context, db: &mut SqliteConnection) -> Option<StateTransition>;
}
