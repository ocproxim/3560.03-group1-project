use diesel::{
    Selectable,
    prelude::{AsChangeset, Identifiable, Insertable, Queryable},
};

#[derive(Debug, Clone, Insertable, Queryable, AsChangeset, Identifiable, Selectable)]
#[diesel(table_name = crate::schema::StatInstances)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[diesel(primary_key(statInstanceID))]
pub struct StatInstance {
    pub statInstanceID: Option<i32>,
    pub gameID: Option<i32>,
    pub playerID: Option<i32>,
    pub statKindID: i32,
    pub timestamp: Option<String>,
    pub value: f32,
}

#[derive(Debug, Clone, Insertable, Identifiable, AsChangeset, Queryable, Selectable)]
#[diesel(table_name = crate::schema::StatKinds)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[diesel(primary_key(statKindID))]
pub struct StatKind {
    pub statKindID: Option<i32>,
    pub sportID: Option<i32>,
    pub statName: String,
    pub unit: String,
}
