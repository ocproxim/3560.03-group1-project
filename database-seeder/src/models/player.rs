use diesel::prelude::Insertable;
use jiff::civil::DateTime;

use crate::models::ids::*;

#[derive(Debug, Clone, Insertable)]
#[diesel(table_name=crate::schema::Players)]
pub struct Player {
    pub playerID: PlayerId,
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
