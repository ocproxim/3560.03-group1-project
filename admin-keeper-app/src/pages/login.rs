use diesel::{
    ExpressionMethods, RunQueryDsl, SelectableHelper, SqliteConnection,
    query_dsl::methods::FilterDsl,
};
use egui::{Button, Color32, Rect, RichText, TextEdit, Ui, UiBuilder};
use sha2::{Digest, Sha256};

use crate::{
    app::AppState,
    models::user::{User, UserRole},
    pages::{Page, StateTransition, admin::AdminPage},
};

#[derive(Default)]
pub struct LoginPage {
    user: String,
    password: String,
    error_text: Option<String>,
}

impl Page for LoginPage {
    fn show(
        &mut self,
        ctx: &egui::Context,
        db: &mut diesel::SqliteConnection,
    ) -> Option<StateTransition> {
        self.login_page(db, ctx)
    }
}
impl LoginPage {
    fn login_page(
        &mut self,
        db: &mut SqliteConnection,
        ctx: &egui::Context,
    ) -> Option<StateTransition> {
        let mut retval: Option<StateTransition> = None;
        egui::CentralPanel::default().show(ctx, |ui| {
            let size = ui.available_size();
            let panel_size = size / 3.0;
            let center = (size / 2.0).to_pos2();
            ui.horizontal_centered(|ui| {
                ui.scope_builder(
                    UiBuilder::new().max_rect(Rect::from_center_size(center, panel_size)),
                    |ui: &mut Ui| {
                        ui.vertical_centered(|ui| {
                            ui.label(
                                RichText::new("Admin Panel")
                                    .size(30.0)
                                    .text_style(egui::TextStyle::Heading),
                            );
                            ui.add(TextEdit::singleline(&mut self.user).hint_text("Username"));

                            ui.add(
                                TextEdit::singleline(&mut self.password)
                                    .hint_text("Password")
                                    .password(true),
                            );
                            if ui
                                .add_enabled(
                                    !self.user.is_empty() && !self.password.is_empty(),
                                    Button::new("Login"),
                                )
                                .clicked()
                            {
                                let password_hash =
                                    format!("{:x?}", Sha256::digest(&self.password).to_vec());
                                use crate::schema::Users::dsl::*;

                                let result = RunQueryDsl::load(
                                    diesel::QueryDsl::select(
                                        Users
                                            .filter(email.eq(self.user.as_str()))
                                            .filter(passwordHash.eq(password_hash.as_str())),
                                        User::as_select(),
                                    ),
                                    db,
                                );
                                if result.as_ref().is_ok_and(|q| !q.is_empty()) {
                                    self.error_text = None;
                                    let user = result.unwrap().first().unwrap().clone();
                                    let urole = UserRole::new(user.role);
                                    let new_page: Box<dyn Page> = match urole {
                                        UserRole::User => Box::new(AdminPage::new(db)),
                                        UserRole::Admin => Box::new(AdminPage::new(db)),
                                        UserRole::Scorekeeper => Box::new(AdminPage::new(db)),
                                    };

                                    retval = Some(Box::new(move |a: &mut AppState| {
                                        a.page = new_page;
                                    }));
                                } else {
                                    self.error_text = Some("Invalid Credentials".to_string());
                                }
                            }
                            if let Some(error) = self.error_text.as_ref() {
                                ui.label(RichText::new(error.as_str()).color(Color32::RED));
                            }
                        });
                    },
                );
            });
        });
        retval
    }
}
