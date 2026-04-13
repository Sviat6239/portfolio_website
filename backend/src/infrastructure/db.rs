use rusqlite::{Connection, Result};

#[derive(Debug)]
pub(crate) struct Project{
    pub id: i64,
    pub title: String,
    pub description: String,
    pub github_url: String,
}

#[derive(Debug)]
pub(crate) struct User{
    pub id: i64,
    pub name: String,
    pub surname: String,
    pub email: String,
    pub login: String,
    pub password_hash: String,
    pub is_admin: bool,
}

pub(crate) fn conn() -> Result<Connection>{
    let conn = Connection::open("db.db")?;
    Ok(conn)
}

pub(crate) fn create_table(conn: &Connection) -> Result<()>{
    conn.execute("CREATE TABLE IF NOT EXISTS projects(\
                        id INTEGER PRIMARY KEY, \
                        title TEXT NOT NULL, \
                        description TEXT NOT NULL, \
                        github_url TEXT NOT NULL)",
                 [])?;

    conn.execute("CREATE TABLE IF NOT EXISTS users(\
                        id INTEGER PRIMARY KEY, \
                        name TEXT NOT NULL, \
                        surname TEXT NOT NULL, \
                        email TEXT NOT NULL, \
                        login  TEXT NOT NULL, \
                        password_hash TEXT NOT NULL, \
                        is_admin INTEGER NOT NULL)",
                 [])?;
    Ok(())
}