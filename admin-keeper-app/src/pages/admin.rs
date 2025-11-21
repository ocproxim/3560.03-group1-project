use diesel::{QueryDsl, RunQueryDsl, SelectableHelper, SqliteConnection, query_builder};
use egui::{CentralPanel, ComboBox, Grid, TextEdit, Ui};
use rand::Rng;

use crate::{
    models::{
        player::Player,
        sport::Sport,
        team::{self, Team, TeamMembership},
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

                        if matches!(
                            filter,
                            PlayerFilter::Team | PlayerFilter::Sport | PlayerFilter::Name
                        ) {
                            ui.label("Fitler: ");
                            ui.add(TextEdit::singleline(&mut self.filter_contents));
                        }
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
                    egui::scroll_area::ScrollArea::vertical().show(ui, |ui| match filter {
                        PlayerFilter::Team => {
                            let query = &self.filter_contents;
                            let mut results = self
                                .teams
                                .iter()
                                .filter_map(|t| {
                                    let sim = strsim::jaro_winkler(
                                        &format!("{} {}", t.homeTown, t.teamName),
                                        query,
                                    );
                                    if sim > 0.7 || query.is_empty() {
                                        return Some((t, sim));
                                    }
                                    None
                                })
                                .collect::<Vec<_>>();
                            results.sort_by(|(_, a), (_, b)| a.total_cmp(b));

                            for (i, (team, _)) in results.into_iter().enumerate() {
                                ui.label(format!("Team: {} {}", team.homeTown, team.teamName));
                                let team_memberships = self.memberships.iter().filter(|tm| {
                                    tm.teamID.is_some_and(|id| Some(id) == team.teamID)
                                });
                                let players = self.players.iter_mut().filter(|p| {
                                    team_memberships.clone().any(|tm| p.playerID == tm.playerID)
                                });
                                render_player_grid(ui, players, &i.to_string());
                            }
                        }
                        PlayerFilter::Sport => {
                            let query = &self.filter_contents;
                            let mut sports = self
                                .sports
                                .iter()
                                .filter_map(|s| {
                                    let sim = strsim::jaro_winkler(&s.sportName, query);
                                    if sim > 0.7 || query.is_empty() {
                                        return Some((s, sim));
                                    }
                                    None
                                })
                                .collect::<Vec<_>>();
                            sports.sort_by(|(_, a), (_, b)| a.total_cmp(b));

                            for (i, (sport, _)) in sports.iter().enumerate() {
                                ui.label(format!("Sport: {}", sport.sportName));
                                let sp = self.sports.iter().find(|sp| {
                                    sp.sportID.is_some_and(|id| Some(id) == sport.sportID)
                                });
                                let teams = self
                                    .teams
                                    .iter()
                                    .filter(|t| sp.is_some_and(|sp| sp.sportID == t.sportID))
                                    .cloned();
                                let tms = self
                                    .memberships
                                    .iter()
                                    .filter(|tms| teams.clone().any(|t| t.teamID == tms.teamID));
                                let ps = self
                                    .players
                                    .iter_mut()
                                    .filter(|p| tms.clone().any(|tm| p.playerID == tm.playerID));
                                render_player_grid(ui, ps, &i.to_string());
                            }
                        }
                        PlayerFilter::Name => {
                            let query = &self.filter_contents;
                            let ps = self.players.iter_mut().filter(|p| {
                                strsim::jaro_winkler(query, &p.name) > 0.7 || query.is_empty()
                            });
                            render_player_grid(ui, ps, "Players");
                        }
                        PlayerFilter::None => {
                            render_player_grid(ui, self.players.iter_mut(), "Players");
                        }
                    });
                }
                TabSelection::Sports => {
                    egui::scroll_area::ScrollArea::vertical().show(ui, |ui| {
                        render_sport_grid(ui, self.sports.iter_mut(), "SportsGrid");
                    });
                }
                TabSelection::Teams { filter } => {
                    egui::scroll_area::ScrollArea::vertical().show(ui, |ui| match filter {
                        TeamFilter::Sport => {
                            let query = &self.filter_contents;
                            let mut sports = self
                                .sports
                                .iter()
                                .filter_map(|s| {
                                    let sim = strsim::jaro_winkler(&s.sportName, query);
                                    if sim > 0.7 || query.is_empty() {
                                        return Some((s, sim));
                                    }
                                    None
                                })
                                .collect::<Vec<_>>();

                            sports.sort_by(|(_, a), (_, b)| a.total_cmp(b));

                            for (i, (sport, _)) in sports.iter().enumerate() {
                                ui.label(format!("Sport: {}", sport.sportName));
                                let teams =
                                    self.teams.iter_mut().filter(|t| sport.sportID == t.sportID);
                                render_team_grid(ui, teams, &i.to_string());
                            }
                        }
                        TeamFilter::Name => {
                            let query = &self.filter_contents;
                            let mut results = self
                                .teams
                                .iter_mut()
                                .filter_map(|t| {
                                    let sim = strsim::jaro_winkler(
                                        &format!("{} {}", t.homeTown, t.teamName),
                                        query,
                                    );
                                    if sim > 0.7 || query.is_empty() {
                                        return Some((t, sim));
                                    }
                                    None
                                })
                                .collect::<Vec<_>>();
                            results.sort_by(|(_, a), (_, b)| a.total_cmp(b));
                            let teams = results.into_iter().map(|(team, _)| team);
                            render_team_grid(ui, teams, "Teams");
                        }
                        _ => {
                            render_team_grid(ui, self.teams.iter_mut(), "Teams");
                        }
                    });
                }
            }
        });
        None
    }
}

fn render_sport_grid<'a>(ui: &mut Ui, sports: impl Iterator<Item = &'a mut Sport>, id: &str) {
    Grid::new(id)
        .striped(true)
        .min_col_width(200.0)
        .show(ui, |ui| {
            for s in sports {
                s.ui_row(ui);
                ui.end_row();
            }
        });
}

fn render_team_grid<'a>(ui: &mut Ui, teams: impl Iterator<Item = &'a mut Team>, id: &str) {
    Grid::new(id)
        .striped(true)
        .min_col_width(200.0)
        .show(ui, |ui| {
            for t in teams {
                t.ui_row(ui);
                ui.end_row();
            }
        });
}
fn render_player_grid<'a>(ui: &mut Ui, players: impl Iterator<Item = &'a mut Player>, id: &str) {
    Grid::new(id)
        .striped(true)
        .min_col_width(200.0)
        .show(ui, |ui| {
            for p in players {
                p.ui_edit_row(ui);
                ui.end_row();
            }
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
