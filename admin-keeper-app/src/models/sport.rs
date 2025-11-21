use diesel::{
    Selectable,
    prelude::{AsChangeset, Identifiable, Insertable, Queryable},
};

use crate::pages::UIInteract;

#[derive(Debug, Clone, Insertable, AsChangeset, Identifiable, Queryable, Selectable)]
#[diesel(table_name=crate::schema::Sports)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[diesel(primary_key(sportID))]
pub struct Sport {
    pub sportID: Option<i32>,
    pub sportName: String,
}
impl Sport {
    pub fn ui_row(&mut self, ui: &mut egui::Ui) -> UIInteract {
        let mut input_changed = false;

        let id_label = self
            .sportID
            .map(|id| id.to_string())
            .unwrap_or_else(|| "*".to_string());
        ui.label(id_label);

        if ui.text_edit_singleline(&mut self.sportName).changed() {
            input_changed = true;
        }

        let save_btn = if input_changed {
            egui::Button::new("💾").fill(egui::Color32::from_rgb(100, 200, 100))
        } else {
            egui::Button::new("💾")
        };

        if ui.add(save_btn).on_hover_text("Save Changes").clicked() {
            return UIInteract::Modified;
        }

        UIInteract::None
    }
}
