use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper, SqliteConnection};
use egui::{CentralPanel, ComboBox, Grid, TextEdit, Ui};

use crate::{
    models::{
        player::Player,
        sport::Sport,
        team::{Team, TeamMembership},
    },
    pages::{Page, UIInteract},
    schema,
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
/*
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
                if self.tab != TabSelection::Sports {
                    ui.label("Filter by: ");
                }
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
                            ui.add(
                                TextEdit::singleline(&mut self.filter_contents)
                                    .hint_text("Enter a search term..."),
                            );
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
                        ui.add(
                            TextEdit::singleline(&mut self.filter_contents)
                                .hint_text("Enter a search term..."),
                        );
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
                            results.sort_by(|(_, a), (_, b)| b.total_cmp(a));

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
                            sports.sort_by(|(_, a), (_, b)| b.total_cmp(a));

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

                            sports.sort_by(|(_, a), (_, b)| b.total_cmp(a));

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
                                    let fullname_sim = strsim::jaro_winkler(
                                        &format!("{} {}", t.homeTown, t.teamName),
                                        query,
                                    );
                                    let name_sim = strsim::jaro_winkler(&t.teamName, query);

                                    let town_sim = strsim::jaro_winkler(&t.homeTown, query);

                                    let avg = fullname_sim + name_sim + town_sim / 3.0;
                                    if name_sim > 0.7
                                        || town_sim > 0.7
                                        || fullname_sim > 0.7
                                        || query.is_empty()
                                    {
                                        return Some((t, avg));
                                    }
                                    None
                                })
                                .collect::<Vec<_>>();
                            results.sort_by(|(_, a), (_, b)| b.total_cmp(a));
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
}*/

fn render_sport_grid<'a>(
    ui: &mut Ui,
    sports: impl Iterator<Item = &'a mut Sport>,
    id: &str,
) -> Option<(UIInteract, i32)> {
    let mut retval = None;
    Grid::new(id)
        .striped(true)
        .min_col_width(200.0)
        .show(ui, |ui| {
            for s in sports {
                let interact = s.ui_row(ui);
                if matches!(interact, UIInteract::Modified | UIInteract::Delete)
                    && let Some(id) = s.sportID
                {
                    retval = Some((interact, id));
                }
                ui.end_row();
            }
        });
    retval
}

fn render_team_grid<'a>(
    ui: &mut Ui,
    teams: impl Iterator<Item = &'a mut Team>,
    id: &str,
) -> Option<(UIInteract, i32)> {
    let mut retval = None;
    Grid::new(id)
        .striped(true)
        .min_col_width(200.0)
        .show(ui, |ui| {
            for t in teams {
                let interact = t.ui_row(ui);
                if matches!(interact, UIInteract::Modified | UIInteract::Delete)
                    && let Some(id) = t.teamID
                {
                    retval = Some((interact, id));
                }
                ui.end_row();
            }
        });
    retval
}

fn render_player_grid<'a>(
    ui: &mut Ui,
    players: impl Iterator<Item = &'a mut Player>,
    id: &str,
) -> Option<(UIInteract, i32)> {
    let mut retval = None;
    Grid::new(id)
        .striped(true)
        .min_col_width(200.0)
        .show(ui, |ui| {
            for player in players {
                let interact = player.ui_row(ui);
                if matches!(interact, UIInteract::Modified | UIInteract::Delete)
                    && let Some(id) = player.playerID
                {
                    retval = Some((interact, id))
                }
                ui.end_row();
            }
        });
    retval
}

impl Page for AdminPage {
    fn show(
        &mut self,
        ctx: &egui::Context,
        db: &mut SqliteConnection,
    ) -> Option<super::StateTransition> {
        let mut ui_interact: Option<(UIInteract, i32)> = None;

        CentralPanel::default().show(ctx, |ui| {
            // --- Tab Selection and Filtering UI (Unchanged) ---
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
                if self.tab != TabSelection::Sports {
                    ui.label("Filter by: ");
                }
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
                            ui.add(
                                TextEdit::singleline(&mut self.filter_contents)
                                    .hint_text("Enter a search term..."),
                            );
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
                        ui.add(
                            TextEdit::singleline(&mut self.filter_contents)
                                .hint_text("Enter a search term..."),
                        );
                    }
                }
            });

            ui.separator();

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
                            results.sort_by(|(_, a), (_, b)| b.total_cmp(a));

                            for (i, (team, _)) in results.into_iter().enumerate() {
                                ui.label(format!("Team: {} {}", team.homeTown, team.teamName));
                                let team_memberships = self.memberships.iter().filter(|tm| {
                                    tm.teamID.is_some_and(|id| Some(id) == team.teamID)
                                });
                                let players = self.players.iter_mut().filter(|p| {
                                    team_memberships.clone().any(|tm| p.playerID == tm.playerID)
                                });
                                if let Some(interaction) =
                                    render_player_grid(ui, players, &i.to_string())
                                {
                                    ui_interact = Some(interaction);
                                }
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
                            sports.sort_by(|(_, a), (_, b)| b.total_cmp(a));

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
                                if let Some(interaction) =
                                    render_player_grid(ui, ps, &i.to_string())
                                {
                                    ui_interact = Some(interaction);
                                }
                            }
                        }
                        PlayerFilter::Name | PlayerFilter::None => {
                            let ps = self.players.iter_mut().filter(|p| {
                                self.filter_contents.is_empty()
                                    || strsim::jaro_winkler(&self.filter_contents, &p.name) > 0.7
                            });
                            if let Some(interaction) = render_player_grid(ui, ps, "Players") {
                                ui_interact = Some(interaction);
                            }
                        }
                    });
                }
                TabSelection::Sports => {
                    egui::scroll_area::ScrollArea::vertical().show(ui, |ui| {
                        if let Some(interaction) =
                            render_sport_grid(ui, self.sports.iter_mut(), "SportsGrid")
                        {
                            ui_interact = Some(interaction);
                        }
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

                            sports.sort_by(|(_, a), (_, b)| b.total_cmp(a));

                            for (i, (sport, _)) in sports.iter().enumerate() {
                                ui.label(format!("Sport: {}", sport.sportName));
                                let teams =
                                    self.teams.iter_mut().filter(|t| sport.sportID == t.sportID);
                                if let Some(interaction) =
                                    render_team_grid(ui, teams, &i.to_string())
                                {
                                    ui_interact = Some(interaction);
                                }
                            }
                        }
                        TeamFilter::Name | TeamFilter::None => {
                            let teams = self.teams.iter_mut().filter(|t| {
                                let query = &self.filter_contents;
                                query.is_empty() || {
                                    let fullname_sim = strsim::jaro_winkler(
                                        &format!("{} {}", t.homeTown, t.teamName),
                                        query,
                                    );
                                    let name_sim = strsim::jaro_winkler(&t.teamName, query);
                                    let town_sim = strsim::jaro_winkler(&t.homeTown, query);
                                    name_sim > 0.7 || town_sim > 0.7 || fullname_sim > 0.7
                                }
                            });

                            if let Some(interaction) = render_team_grid(ui, teams, "Teams") {
                                ui_interact = Some(interaction);
                            }
                        }
                    });
                }
            }
        });

        if let Some((interact, id)) = ui_interact {
            match self.tab {
                TabSelection::Players { filter: _ } => match interact {
                    UIInteract::Modified => {
                        if let Some(player) = self.players.iter().find(|p| p.playerID == Some(id)) {
                            let _ = diesel::update(schema::Players::dsl::Players)
                                .filter(schema::Players::dsl::playerID.eq(id))
                                .set(player)
                                .execute(db);
                        }
                    }
                    UIInteract::Delete => {
                        if let Some(index) =
                            self.players.iter().position(|p| p.playerID == Some(id))
                        {
                            self.players.remove(index);
                        }
                    }
                    UIInteract::None => {}
                },
                TabSelection::Sports => match interact {
                    UIInteract::Modified => {
                        if let Some(sport) = self.sports.iter().find(|s| s.sportID == Some(id)) {
                            let _ = diesel::update(schema::Sports::dsl::Sports)
                                .filter(schema::Sports::dsl::sportID.eq(id))
                                .set(sport)
                                .execute(db);
                        }
                    }
                    UIInteract::Delete => {
                        if let Some(index) = self.sports.iter().position(|s| s.sportID == Some(id))
                        {
                            self.sports.remove(index);
                        }
                    }
                    UIInteract::None => {}
                },
                TabSelection::Teams { filter: _ } => match interact {
                    UIInteract::Modified => {
                        if let Some(team) = self.teams.iter().find(|t| t.teamID == Some(id)) {
                            let _ = diesel::update(schema::Teams::dsl::Teams)
                                .filter(schema::Teams::dsl::teamID.eq(id))
                                .set(team)
                                .execute(db);
                        }
                    }
                    UIInteract::Delete => {
                        if let Some(index) = self.teams.iter().position(|t| t.teamID == Some(id)) {
                            self.teams.remove(index);
                        }
                    }
                    UIInteract::None => {}
                },
            }
        }
        None
    }
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
