use crate::infrastructure;
use infrastructure::db::{conn, create_table, User};
use bcrypt::*;
use rusqlite::Connection;

pub fn hash_password(password: &str, rounds: u32) -> Result<String, BcryptError> {
    let hashed = hash(password, rounds)?;
    Ok(hashed)
}

pub fn register(conn: &Connection, name: &str, surname: &str, email: &str, login: &str, password: &str, is_admin: bool, ) -> Result<User, Box<dyn std::error::Error>> {

    let hashed = hash_password(password, 14)?;

    conn.execute(
        "INSERT INTO users (name, surname, email, login, hashed, is_admin)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        (&name, &surname, &email, &login, &hashed, is_admin),
    )?;

    let id = conn.last_insert_rowid();

    Ok(User {
        id,
        name: name.to_string(),
        surname: surname.to_string(),
        email: email.to_string(),
        login: login.to_string(),
        password_hash: hashed.to_string(),
        is_admin,
    })
}

pub fn login(conn: &Connection, login: &str, password: &str) -> Result<()>{
    conn.execute("SELECT user WHERE login = ?1")?;


}

pub fn logout(){}