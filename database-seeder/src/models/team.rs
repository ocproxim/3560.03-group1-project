use diesel::prelude::Insertable;

use crate::models::ids::*;

#[derive(Debug, Clone, Insertable)]
#[diesel(table_name=crate::schema::Teams)]
pub struct Team {
    pub teamID: TeamId,
    pub sportID: SportId,
    pub teamName: String,
    pub homeTown: String,
}

impl Team {
    pub fn get_team_name(&self) -> &str {
        &self.teamName
    }

    pub fn get_home(&self) -> &str {
        &self.homeTown
    }

    pub fn set_team_name(&mut self, new_name: String) {
        self.teamName = new_name;
    }

    pub fn set_home(&mut self, new_home_town: String) {
        self.homeTown = new_home_town;
    }
}
#[derive(Debug, Clone, Insertable)]
#[diesel(table_name=crate::schema::TeamMemberships)]
pub struct TeamMembership {
    pub membershipID: TeamMembershipId,
    pub teamID: TeamId,
    pub playerID: PlayerId,
    pub season: String,
    pub jerseyNumber: i32,
}

impl TeamMembership {
    pub fn get_season(&self) -> &str {
        &self.season
    }

    pub fn get_jersey_number(&self) -> i32 {
        self.jerseyNumber
    }

    pub fn set_season(&mut self, new_season: String) {
        self.season = new_season;
    }

    pub fn set_jersey_number(&mut self, new_jersey_number: i32) {
        self.jerseyNumber = new_jersey_number;
    }
}
