use diesel::prelude::Insertable;

use crate::models::ids::*;

#[derive(Debug, Clone, Insertable)]
#[diesel(table_name=crate::schema::Users)]
pub struct User {
    pub userID: UserId,
    pub email: String,
    pub passwordHash: String,
    pub role: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum UserRole {
    User,
    Admin,
    Scorekeeper,
}

impl User {
    pub fn get_email(&self) -> &str {
        &self.email
    }

    pub fn get_password_hash(&self) -> &str {
        &self.passwordHash
    }

    pub fn get_role(&self) -> i32 {
        self.role
    }

    pub fn set_email(&mut self, new_email: String) {
        self.email = new_email;
    }

    pub fn set_password_hash(&mut self, new_password: String) {
        self.passwordHash = new_password;
    }
}
