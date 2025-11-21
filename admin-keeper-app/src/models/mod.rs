#[allow(non_snake_case)]
pub mod game;
pub mod ids;
pub mod junction_table;

#[allow(non_snake_case)]
pub mod player;

#[allow(non_snake_case)]
pub mod sport;

#[allow(non_snake_case)]
pub mod stat;

#[allow(non_snake_case)]
pub mod team;

#[allow(non_snake_case)]
pub mod user;

pub trait SQLClass {
    fn create_table_statement() -> String;
    fn insert_statement() -> String;
}
