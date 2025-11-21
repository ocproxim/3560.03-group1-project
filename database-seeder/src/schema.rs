// @generated automatically by Diesel CLI.

diesel::table! {
    Games (gameID) {
        gameID -> Nullable<Integer>,
        homeTeamID -> Nullable<Integer>,
        awayTeamID -> Nullable<Integer>,
        homeScore -> Float,
        awayScore -> Float,
        gameTime -> Text,
        venue -> Text,
    }
}

diesel::table! {
    Players (playerID) {
        playerID -> Nullable<Integer>,
        name -> Text,
        dateOfBirth -> Text,
        height -> Integer,
        weight -> Integer,
    }
}

diesel::table! {
    Sports (sportID) {
        sportID -> Nullable<Integer>,
        sportName -> Text,
    }
}

diesel::table! {
    StatInstances (statInstanceID) {
        statInstanceID -> Nullable<Integer>,
        playerID -> Nullable<Integer>,
        gameID -> Nullable<Integer>,
        statKindID -> Integer,
        timestamp -> Nullable<Text>,
        value -> Float,
    }
}

diesel::table! {
    StatKinds (statKindID) {
        statKindID -> Nullable<Integer>,
        sportID -> Nullable<Integer>,
        statName -> Text,
        unit -> Text,
    }
}

diesel::table! {
    TeamMemberships (membershipID) {
        membershipID -> Nullable<Integer>,
        playerID -> Nullable<Integer>,
        teamID -> Nullable<Integer>,
        season -> Text,
        jerseyNumber -> Integer,
    }
}

diesel::table! {
    Teams (teamID) {
        teamID -> Nullable<Integer>,
        sportID -> Nullable<Integer>,
        teamName -> Text,
        homeTown -> Text,
    }
}

diesel::table! {
    Users (userID) {
        userID -> Nullable<Integer>,
        email -> Text,
        passwordHash -> Text,
        role -> Integer,
    }
}

diesel::joinable!(StatInstances -> Games (gameID));
diesel::joinable!(StatInstances -> Players (playerID));
diesel::joinable!(StatInstances -> StatKinds (statKindID));
diesel::joinable!(StatKinds -> Sports (sportID));
diesel::joinable!(TeamMemberships -> Players (playerID));
diesel::joinable!(TeamMemberships -> Teams (teamID));
diesel::joinable!(Teams -> Sports (sportID));

diesel::allow_tables_to_appear_in_same_query!(
    Games,
    Players,
    Sports,
    StatInstances,
    StatKinds,
    TeamMemberships,
    Teams,
    Users,
);
