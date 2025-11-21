use diesel::{QueryDsl, RunQueryDsl, SelectableHelper, SqliteConnection};
use eframe::Renderer;
use egui::{CentralPanel, ComboBox, Grid, TextEdit, Ui};

use crate::{
    models::{
        player::Player,
        sport::Sport,
        team::{Team, TeamMembership},
    },
    pages::Page,
    schema::Sports::sportName,
};

#[derive(Clone, PartialEq, PartialOrd, Ord, Eq)]
enum TabSelection {
    Players { filter: PlayerFilter },
    Sports,
    Teams { filter: TeamFilter },
}

impl TabSelection {
    pub fn is_player(&self) -> bool {
        matches!(self, TabSelection::Players { filter: _ })
    }
    pub fn is_team(&self) -> bool {
        matches!(self, TabSelection::Teams { filter: _ })
    }
    pub fn is_sports(&self) -> bool {
        matches!(self, TabSelection::Sports)
    }
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Copy)]
enum PlayerFilter {
    None,
    Team,
    Sport,
    Name,
}

impl PlayerFilter {
    pub fn kind(&self) -> &'static str {
        match self {
            PlayerFilter::Team => "Team",
            PlayerFilter::Sport => "Sport",
            PlayerFilter::Name => "Name",
            PlayerFilter::None => "None",
        }
    }
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Copy)]
enum TeamFilter {
    None,
    Sport,
    Name,
}

impl TeamFilter {
    pub fn kind(&self) -> &'static str {
        match self {
            TeamFilter::None => "None",
            TeamFilter::Sport => "Sport",
            TeamFilter::Name => "Name",
        }
    }
}

pub struct AdminPage {
    tab: TabSelection,
    players: Vec<Player>,
    sports: Vec<Sport>,
    teams: Vec<Team>,
    memberships: Vec<TeamMembership>,
    filter_contents: String,
}

impl Page for AdminPage {
    fn show(
        &mut self,
        ctx: &egui::Context,
        db: &mut SqliteConnection,
    ) -> Option<super::StateTransition> {
        CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                if ui.radio(self.tab.is_player(), "Players").clicked() {
                    self.tab = TabSelection::Players {
                        filter: PlayerFilter::None,
                    };
                }

                if ui.radio(self.tab.is_team(), "Teams").clicked() {
                    self.tab = TabSelection::Teams {
                        filter: TeamFilter::None,
                    };
                }

                if ui.radio(self.tab.is_sports(), "Sports").clicked() {
                    self.tab = TabSelection::Sports;
                }
                ui.label("Filter: ");
                match &mut self.tab {
                    TabSelection::Players { filter } => {
                        let c = ComboBox::from_label("").selected_text(filter.kind());
                        c.show_ui(ui, |ui| {
                            ui.selectable_value(filter, PlayerFilter::None, "None");
                            ui.selectable_value(filter, PlayerFilter::Team, "Team");
                            ui.selectable_value(filter, PlayerFilter::Sport, "Sport");
                            ui.selectable_value(filter, PlayerFilter::Name, "Name");
                        });

                        ui.label("Fitler: ");
                        ui.add(TextEdit::singleline(&mut self.filter_contents));
                    }
                    TabSelection::Sports => {}
                    TabSelection::Teams { filter } => {
                        let c = ComboBox::from_label("").selected_text(filter.kind());
                        c.show_ui(ui, |ui| {
                            ui.selectable_value(filter, TeamFilter::None, "None");
                            ui.selectable_value(filter, TeamFilter::Sport, "Sport");
                            ui.selectable_value(filter, TeamFilter::Name, "Name");
                        });
                        ui.label("Fitler: ");
                        ui.add(TextEdit::singleline(&mut self.filter_contents));
                    }
                }
            });
            match &self.tab {
                TabSelection::Players { filter } => {
                    if self.filter_contents.is_empty() {
                        render_player_grid(ui, self.players.iter_mut());
                    } else {
                        match filter {
                            PlayerFilter::Team => {
                                let query = &self.filter_contents;
                                let (team, _) = self
                                    .teams
                                    .iter()
                                    .map(|t| {
                                        (
                                            t,
                                            strsim::jaro_winkler(
                                                &format!("{} {}", t.homeTown, t.teamName),
                                                query,
                                            ),
                                        )
                                    })
                                    .max_by(|(_, a), (_, b)| a.total_cmp(b))
                                    .unwrap_or((&self.teams[0], 0.0));
                                let i = team.teamID.unwrap_or(0);
                                let mut team_memberships = self
                                    .memberships
                                    .iter()
                                    .filter(|tm| tm.teamID.is_some_and(|id| id == i));
                                let players = self.players.iter_mut().filter(|p| {
                                    team_memberships
                                        .by_ref()
                                        .any(|tm| p.playerID == tm.playerID)
                                });
                                render_player_grid(ui, players);
                            }
                            PlayerFilter::Sport => {
                                let query = &self.filter_contents;
                                let (sport, _) = self
                                    .sports
                                    .iter()
                                    .map(|s| (s, strsim::jaro_winkler(&s.sportName, query)))
                                    .max_by(|(_, a), (_, b)| a.total_cmp(b))
                                    .unwrap_or((&self.sports[0], 0.0));

                                let i = sport.sportID.unwrap_or(0);
                                let sp = self
                                    .sports
                                    .iter()
                                    .find(|sp| sp.sportID.is_some_and(|id| id == i));
                                let teams = self
                                    .teams
                                    .iter()
                                    .filter(|t| sp.is_some_and(|sp| sp.sportID == t.sportID))
                                    .cloned();
                                let tms = self
                                    .memberships
                                    .iter()
                                    .filter(|tms| teams.clone().any(|t| t.teamID == tms.teamID));
                                let ps = self.players.iter_mut().filter(|p| {
                                    tms.clone().by_ref().any(|tm| p.playerID == tm.playerID)
                                });
                                render_player_grid(ui, ps);
                            }
                            PlayerFilter::Name => {
                                let n = &self.filter_contents;
                                let ps = self
                                    .players
                                    .iter_mut()
                                    .filter(|p| strsim::jaro_winkler(n, &p.name) > 0.7);
                                render_player_grid(ui, ps);
                            }
                            PlayerFilter::None => {
                                render_player_grid(ui, self.players.iter_mut());
                            }
                        }
                    }
                }
                TabSelection::Sports => {}
                TabSelection::Teams { filter } => {}
            }
        });
        None
    }
}
fn render_player_grid<'a>(ui: &mut Ui, ps: impl Iterator<Item = &'a mut Player>) {
    egui::scroll_area::ScrollArea::vertical().show(ui, |ui| {
        Grid::new("Players")
            .striped(true)
            .min_col_width(80.0)
            .show(ui, |ui| {
                for p in ps {
                    p.ui_edit_row(ui);
                    ui.end_row();
                }
            });
    });
}

impl AdminPage {
    pub fn new(db: &mut SqliteConnection) -> Self {
        let players = crate::schema::Players::table
            .select(Player::as_select())
            .load(db)
            .unwrap_or_default();
        let sports = crate::schema::Sports::table
            .select(Sport::as_select())
            .load(db)
            .unwrap_or_default();
        let teams = crate::schema::Teams::table
            .select(Team::as_select())
            .load(db)
            .unwrap_or_default();
        let memberships = crate::schema::TeamMemberships::table
            .select(TeamMembership::as_select())
            .load(db)
            .unwrap_or_default();

        Self {
            players,
            sports,
            teams,
            memberships,
            tab: TabSelection::Players {
                filter: PlayerFilter::None,
            },
            filter_contents: String::new(),
        }
    }
}
