use diesel::{Connection, SqliteConnection};

pub const DATABASE_URL: &str = "sqlite://../sqlite.db";

type DbResult<T> = Result<T, Box<dyn std::error::Error + Send + Sync + 'static>>;

pub fn establish_connection(db_url: &str) -> Result<SqliteConnection, diesel::ConnectionError> {
    SqliteConnection::establish(db_url)
}

fn main() -> eframe::Result {
    let db = establish_connection(DATABASE_URL).expect("Could not connect to the DB");
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([400.0, 300.0])
            .with_min_inner_size([300.0, 220.0]),
        ..Default::default()
    };
    eframe::run_native(
        "eframe template",
        native_options,
        Box::new(|_| Ok(Box::new(admin_keeper_app::app::App::new(db)))),
    )
}
