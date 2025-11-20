use diesel::prelude::Insertable;

use crate::models::ids::*;

#[derive(Debug, Clone, Insertable)]
#[diesel(table_name = crate::schema::StatInstances)]
pub struct StatInstance {
    pub statInstanceID: StatInstanceId,
    pub gameID: GameId,
    pub playerID: PlayerId,
    pub statKindID: StatKindId,
    pub timestamp: String,
    pub value: f32,
}

#[derive(Debug, Clone, Insertable)]
#[diesel(table_name = crate::schema::StatKinds)]
pub struct StatKind {
    pub statKindID: StatKindId,
    pub sportID: SportId,
    pub statName: String,
    pub unit: String,
}
