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

impl Sport {
    pub fn ui_row(&mut self, ui: &mut egui::Ui) -> bool {
        let mut changed = false;

        let id_label = self
            .sportID
            .map(|id| id.to_string())
            .unwrap_or_else(|| "*".to_string());
        ui.label(id_label);

        if ui.text_edit_singleline(&mut self.sportName).changed() {
            changed = true;
        }

        changed
    }
}
