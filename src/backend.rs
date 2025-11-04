use anyhow::Result;
use dioxus::prelude::*;

#[cfg(feature = "server")]
thread_local! {
    pub static DB: rusqlite::Connection = {
        // Open the database from the persisted "hotdog.db" file
        let conn = rusqlite::Connection::open("hotdog.db").expect("Failed to open database");

        // Create the "dogs" table if it doesn't already exist
        conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS dogs (
                id INTEGER PRIMARY KEY,
                url TEXT NOT NULL
            );",
        ).unwrap();

        // Return the connection
        conn
    };
}

#[server]
pub async fn list_dogs() -> Result<Vec<(usize, String)>> {
    DB.with(|db| {
        Ok(db
            .prepare("SELECT id, url FROM dogs ORDER BY id DESC LIMIT 10")?
            .query_map([], |row| Ok((row.get(0)?, row.get(1)?)))?
            .collect::<Result<Vec<(usize, String)>, rusqlite::Error>>()?)
    })
}

#[server]
pub async fn remove_dog(id: usize) -> Result<()> {
    DB.with(|db| db.execute("DELETE FROM dogs WHERE id = ?1", [id]))?;
    Ok(())
}

#[server]
pub async fn save_dog(image: String) -> Result<()> {
    DB.with(|db| db.execute("INSERT INTO dogs (url) VALUES (?1)", [&image]))?;
    Ok(())
}
