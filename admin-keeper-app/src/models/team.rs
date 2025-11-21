use diesel::{
    Selectable,
    prelude::{AsChangeset, Identifiable, Insertable, Queryable},
};

#[derive(Debug, Clone, Insertable, Identifiable, AsChangeset, Queryable, Selectable)]
#[diesel(table_name=crate::schema::Teams)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[diesel(primary_key(teamID))]
pub struct Team {
    pub teamID: Option<i32>,
    pub sportID: Option<i32>,
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

    pub fn ui_row(&mut self, ui: &mut egui::Ui) -> bool {
        let mut changed = false;

        let id_label = self
            .teamID
            .map(|id| id.to_string())
            .unwrap_or_else(|| "*".to_string());
        ui.label(id_label);

        let mut sport_id_val = self.sportID.unwrap_or(0);
        if ui.add(egui::DragValue::new(&mut sport_id_val)).changed() {
            self.sportID = Some(sport_id_val);
            changed = true;
        }

        if ui.text_edit_singleline(&mut self.teamName).changed() {
            changed = true;
        }

        if ui.text_edit_singleline(&mut self.homeTown).changed() {
            changed = true;
        }

        changed
    }
}
#[derive(Debug, Clone, Insertable, AsChangeset, Queryable, Selectable, Identifiable)]
#[diesel(table_name=crate::schema::TeamMemberships)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[diesel(primary_key(membershipID))]
pub struct TeamMembership {
    pub membershipID: Option<i32>,
    pub teamID: Option<i32>,
    pub playerID: Option<i32>,
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
