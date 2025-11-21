use std::str::FromStr;

use diesel::{
    Selectable,
    prelude::{AsChangeset, Identifiable, Insertable, Queryable},
};
use jiff::civil::{Date, DateTime};

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

impl Player {
    /// Renders a specific row for this player.
    /// Designed to be used inside an `egui::Grid`.
    pub fn ui_edit_row(&mut self, ui: &mut egui::Ui) {
        // 1. ID (Read-only)
        ui.label(
            self.playerID
                .map(|id| id.to_string())
                .unwrap_or_else(|| "New".to_string()),
        );

        // 2. Name
        ui.add(egui::TextEdit::singleline(&mut self.name).hint_text("Player Name"));

        // 3. Date of Birth (Smart Jiff Editor)
        self.ui_date_editor(ui);

        // 4. Height (cm)
        ui.add(
            egui::DragValue::new(&mut self.height)
                .speed(1.0)
                .range(0..=300)
                .suffix(" cm"),
        );

        // 5. Weight (kg)
        ui.add(
            egui::DragValue::new(&mut self.weight)
                .speed(1.0)
                .range(0..=500)
                .suffix(" kg"),
        );
    }

    /// Helper to handle the String <-> Jiff Date conversion for the UI
    fn ui_date_editor(&mut self, ui: &mut egui::Ui) {
        // Try to parse the stored String as a Jiff Date (YYYY-MM-DD)
        // We use Date instead of DateTime for DOB usually, but adapt as needed.
        let parsed_date = Date::from_str(&self.dateOfBirth);

        match parsed_date {
            Ok(date) => {
                // If valid, show fancy date pickers
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = 2.0;

                    // Year
                    let mut year = date.year();
                    if ui
                        .add(
                            egui::DragValue::new(&mut year)
                                .range(1900..=2100)
                                .speed(0.5),
                        )
                        .changed()
                    {
                        // Handle year change logic (clamping, leap years etc handled by Jiff mostly)
                        if let Ok(new_date) = Date::new(year, date.month(), date.day()) {
                            self.dateOfBirth = new_date.to_string();
                        }
                    }
                    ui.label("-");

                    // Month
                    let mut month = date.month();
                    if ui
                        .add(egui::DragValue::new(&mut month).range(1..=12).speed(0.1))
                        .changed()
                        && let Ok(new_date) = Date::new(date.year(), month, date.day())
                    {
                        self.dateOfBirth = new_date.to_string();
                    }
                    ui.label("-");

                    // Day
                    let mut day = date.day();
                    if ui
                        .add(egui::DragValue::new(&mut day).range(1..=31).speed(0.1))
                        .changed()
                        && let Ok(new_date) = Date::new(date.year(), date.month(), day)
                    {
                        self.dateOfBirth = new_date.to_string();
                    }
                });
            }
            Err(_) => {
                // Fallback: If string is malformed, show raw text edit so user can fix it manually
                let response = ui.add(
                    egui::TextEdit::singleline(&mut self.dateOfBirth)
                        .text_color(egui::Color32::RED)
                        .desired_width(100.0),
                );
                response.on_hover_text("Invalid Date Format. Use YYYY-MM-DD");
            }
        }
    }
}
