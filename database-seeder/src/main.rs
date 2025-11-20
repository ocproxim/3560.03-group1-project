use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;

use async_std::task;

use crate::generator::{SeededDB, generate_db_data};
pub mod generator;
pub mod models;
pub mod schema;
const DATABASE_URL: &str = "sqlite://sqlite.db";

type DbResult<T> = Result<T, Box<dyn std::error::Error + Send + Sync + 'static>>;

pub fn establish_connection(db_url: &str) -> Result<SqliteConnection, diesel::ConnectionError> {
    SqliteConnection::establish(db_url)
}

#[async_std::main]
async fn main() -> DbResult<()> {
    let db = generate_db_data();

    task::spawn_blocking(move || -> DbResult<()> {
        let mut conn = establish_connection(DATABASE_URL)?;
        println!("Connection established.");

        let SeededDB {
            users,
            sports,
            players,
            teams,
            memberships,
            games,
            stat_kinds,
            stat_instances,
        } = db;

        conn.transaction(|conn| -> DbResult<()> {
            use crate::schema;

            println!("Inserting users... ({} records)", users.len());
            diesel::insert_into(crate::schema::Users::table)
                .values(&users)
                .execute(conn)?;

            println!("Inserting sports... ({} records)", sports.len());
            diesel::insert_into(crate::schema::Sports::table)
                .values(&sports)
                .execute(conn)?;

            println!("Inserting teams... ({} records)", teams.len());
            diesel::insert_into(crate::schema::Teams::table)
                .values(&teams)
                .execute(conn)?;

            println!("Inserting players... ({} records)", players.len());
            diesel::insert_into(schema::Players::table)
                .values(&players)
                .execute(conn)?;

            println!("Inserting memberships... ({} records)", memberships.len());
            diesel::insert_into(schema::TeamMemberships::table)
                .values(&memberships)
                .execute(conn)?;

            println!("Inserting games... ({} records)", games.len());
            diesel::insert_into(schema::Games::table)
                .values(&games)
                .execute(conn)?;

            println!("Inserting stat_kinds... ({} records)", stat_kinds.len());
            diesel::insert_into(schema::StatKinds::table)
                .values(&stat_kinds)
                .execute(conn)?;

            println!(
                "Inserting stat_instances... ({} records)",
                stat_instances.len()
            );
            diesel::insert_into(schema::StatInstances::table)
                .values(&stat_instances)
                .execute(conn)?;

            Ok(())
        })?;

        Ok(())
    })
    .await?;

    println!("Done!");
    Ok(())
}
