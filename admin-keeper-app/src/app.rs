use diesel::SqliteConnection;

use crate::pages::{Page, StateTransition, login::LoginPage};

pub struct App {
    state: AppState,
    db_conn: SqliteConnection,
}

pub struct AppState {
    pub page: Box<dyn Page>,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            page: Box::new(LoginPage::default()),
        }
    }
}

impl AppState {
    pub fn show(
        &mut self,
        ctx: &egui::Context,
        db: &mut SqliteConnection,
    ) -> Option<StateTransition> {
        self.page.show(ctx, db)
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.set_pixels_per_point(1.5);
        if let Some(func) = self.state.show(ctx, &mut self.db_conn) {
            func(&mut self.state);
        }
    }
}

impl App {
    pub fn new(conn: SqliteConnection) -> Self {
        Self {
            state: Default::default(),
            db_conn: conn,
        }
    }
}
