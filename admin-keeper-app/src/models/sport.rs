use diesel::{
    Selectable,
    prelude::{AsChangeset, Identifiable, Insertable, Queryable},
};

#[derive(Debug, Clone, Insertable, AsChangeset, Identifiable, Queryable, Selectable)]
#[diesel(table_name=crate::schema::Sports)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[diesel(primary_key(sportID))]
pub struct Sport {
    pub sportID: Option<i32>,
    pub sportName: String,
}
