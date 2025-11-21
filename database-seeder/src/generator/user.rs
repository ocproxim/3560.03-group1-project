use sha2::{Digest, Sha256};

use crate::models::user::User;

pub fn generate_users() -> Vec<User> {
    let admin_email = "admin@goated.rus".to_string();
    let admin_password = "adminpass";
    let admin_hash = format!("{:x?}", Sha256::digest(admin_password).to_vec());
    let admin_account = User {
        userID: 0,
        email: admin_email,
        passwordHash: admin_hash,
        role: 1,
    };

    let user_email = "user@goated.rus".to_string();
    let user_password = "userpass";
    let user_hash = format!("{:x?}", Sha256::digest(user_password).to_vec());
    let user_account = User {
        userID: 1,
        email: user_email,
        passwordHash: user_hash,
        role: 0,
    };

    let keeper_email = "keeper@goated.rus".to_string();
    let keeper_password = "keeperpass";
    let keeper_hash = format!("{:x?}", Sha256::digest(keeper_password).to_vec());
    let keeper_account = User {
        userID: 2,
        email: keeper_email,
        passwordHash: keeper_hash,
        role: 2,
    };

    vec![admin_account, user_account, keeper_account]
}
