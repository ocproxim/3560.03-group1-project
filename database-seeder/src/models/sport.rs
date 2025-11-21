use diesel::prelude::Insertable;

use crate::models::ids::*;

#[derive(Debug, Clone, Insertable)]
#[diesel(table_name=crate::schema::Sports)]
pub struct Sport {
    pub sportID: SportId,
    pub sportName: String,
}
