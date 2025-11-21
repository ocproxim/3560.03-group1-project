use diesel::{
    Selectable,
    prelude::{AsChangeset, Identifiable, Insertable, Queryable},
};

#[derive(Debug, Clone, Insertable, Identifiable, AsChangeset, Queryable, Selectable)]
#[diesel(table_name=crate::schema::Users)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[diesel(primary_key(userID))]
pub struct User {
    pub userID: Option<i32>,
    pub email: String,
    pub passwordHash: String,
    pub role: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum UserRole {
    User = 0,
    Admin = 1,
    Scorekeeper = 2,
}

impl UserRole {
    pub fn new(val: i32) -> Self {
        match val {
            0 => UserRole::User,
            1 => UserRole::Admin,
            2 => UserRole::Scorekeeper,
            _ => UserRole::User,
        }
    }
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
