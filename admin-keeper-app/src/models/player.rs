use std::str::FromStr;

use diesel::{
    Selectable,
    prelude::{AsChangeset, Identifiable, Insertable, Queryable},
};
use jiff::civil::{Date, DateTime};

use crate::pages::UIInteract;

#[derive(Debug, Clone, Insertable, Queryable, Identifiable, AsChangeset, Selectable)]
#[diesel(table_name=crate::schema::Players)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[diesel(primary_key(playerID))]
pub struct Player {
    pub playerID: Option<i32>,
    pub name: String,
    pub dateOfBirth: String,
    pub height: i32,
    pub weight: i32,
}

impl Player {
    pub fn ui_row(&mut self, ui: &mut egui::Ui) -> UIInteract {
        let mut input_changed = false;

        ui.label(
            self.playerID
                .map(|id| id.to_string())
                .unwrap_or_else(|| "New".to_string()),
        );

        if ui
            .add(egui::TextEdit::singleline(&mut self.name).hint_text("Player Name"))
            .changed()
        {
            input_changed = true;
        }

        if let UIInteract::Modified = self.ui_date_editor(ui) {
            input_changed = true;
        }

        if ui
            .add(
                egui::DragValue::new(&mut self.height)
                    .speed(1.0)
                    .range(0..=300)
                    .suffix(" cm"),
            )
            .changed()
        {
            input_changed = true;
        }

        if ui
            .add(
                egui::DragValue::new(&mut self.weight)
                    .speed(1.0)
                    .range(0..=500)
                    .suffix(" kg"),
            )
            .changed()
        {
            input_changed = true;
        }

        let save_btn = if input_changed {
            egui::Button::new("💾").fill(egui::Color32::from_rgb(100, 200, 100)) // Green tint if changed
        } else {
            egui::Button::new("💾")
        };

        if ui.add(save_btn).on_hover_text("Save Changes").clicked() {
            return UIInteract::Modified;
        }

        if ui.button("🗑").on_hover_text("Delete Player").clicked() {
            return UIInteract::Delete;
        }

        UIInteract::None
    }

    fn ui_date_editor(&mut self, ui: &mut egui::Ui) -> UIInteract {
        let parsed_date = Date::from_str(&self.dateOfBirth);
        let mut changed = false;

        match parsed_date {
            Ok(date) => {
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = 2.0;

                    let mut year = date.year();
                    if ui
                        .add(
                            egui::DragValue::new(&mut year)
                                .range(1900..=2100)
                                .speed(0.5),
                        )
                        .changed()
                        && let Ok(new_date) = Date::new(year, date.month(), date.day())
                    {
                        self.dateOfBirth = new_date.to_string();
                        changed = true;
                    }
                    ui.label("-");

                    let mut month = date.month();
                    if ui
                        .add(egui::DragValue::new(&mut month).range(1..=12).speed(0.1))
                        .changed()
                        && let Ok(new_date) = Date::new(date.year(), month, date.day())
                    {
                        self.dateOfBirth = new_date.to_string();
                        changed = true;
                    }
                    ui.label("-");

                    let mut day = date.day();
                    if ui
                        .add(egui::DragValue::new(&mut day).range(1..=31).speed(0.1))
                        .changed()
                        && let Ok(new_date) = Date::new(date.year(), date.month(), day)
                    {
                        self.dateOfBirth = new_date.to_string();
                        changed = true;
                    }
                });
            }
            Err(_) => {
                if ui
                    .add(
                        egui::TextEdit::singleline(&mut self.dateOfBirth)
                            .text_color(egui::Color32::RED)
                            .desired_width(100.0),
                    )
                    .changed()
                {
                    changed = true;
                }
            }
        }

        if changed {
            UIInteract::Modified
        } else {
            UIInteract::None
        }
    }
}

impl Player {
    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_height(&self) -> i32 {
        self.height
    }

    pub fn get_weight(&self) -> i32 {
        self.weight
    }

    pub fn set_name(&mut self, new_name: String) {
        self.name = new_name;
    }

    pub fn set_dob(&mut self, new_dob: DateTime) {
        self.dateOfBirth = new_dob.to_string();
    }

    pub fn set_height(&mut self, new_height: i32) {
        self.height = new_height;
    }

    pub fn set_weight(&mut self, new_weight: i32) {
        self.weight = new_weight;
    }
}
