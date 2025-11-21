use crate::models::ids::*;

#[derive(Debug, Clone)]
pub struct JunctionTable {
    pub junction_table_id: JunctionTableId,
    pub game_id: GameId,
    pub team_id: TeamId,
}
