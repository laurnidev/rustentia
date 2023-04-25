use crate::flashcard::*;
use rusqlite::{Connection, Result};

pub struct Database {
    pub connection: Connection,
}

impl Database {
    pub fn new() -> Result<Self> {
        let connection = Connection::open("flashcards.db")?;
        Ok(Self { connection })
    }
    pub fn create_tables(&self) -> Result<()> {
        self.connection.execute(
            "CREATE TABLE IF NOT EXISTS decks (
             id INTEGER PRIMARY KEY,
             name TEXT UNIQUE
             )",
            [],
        )?;
        self.connection.execute(
            "CREATE TABLE IF NOT EXISTS flashcards (
             id INTEGER PRIMARY KEY,
             front TEXT,
             back TEXT,
             correct BOOLEAN,
             current_side INTEGER,
             deck_id INTEGER,
             FOREIGN KEY(deck_id) REFERENCES decks(id)
         )",
            [],
        )?;
        Ok(())
    }

    pub fn add_deck(&self, name: &str) -> Result<()> {
        self.connection
            .execute("INSERT INTO decks (name) VALUES (?1)", &[name])?;
        Ok(())
    }

    pub fn add_flashcard(&self, front: &str, back: &str, deck_name: &str) -> Result<()> {
        let deck_id: String = self.connection.query_row(
            "SELECT id FROM decks WHERE name = ?1",
            &[deck_name],
            |row| row.get(0).map(|id: i64| id.to_string()),
        )?;
        println!("deck_id: {}", deck_id);
        self.connection.execute(
            "INSERT INTO flashcards (front, back, correct, current_side, deck_id)
            VALUES (?1, ?2, false, 0, ?3)",
            &[front, back, &deck_id],
        )?;
        Ok(())
    }
    pub fn get_all_decks(&self) -> Result<Vec<String>> {
        let mut stmt = self.connection.prepare("SELECT name FROM decks")?;
        let rows = stmt.query_map([], |row| Ok(row.get::<usize, String>(0)?))?;
        let mut decks = Vec::new();
        for name in rows {
            decks.push(name?);
        }
        Ok(decks)
    }
    pub fn get_flashcards(&self, deck_name: &str) -> Result<Vec<FlashCard>> {
        let mut stmt = self.connection.prepare(
            "SELECT front, back, correct FROM flashcards
             INNER JOIN decks ON flashcards.deck_id = decks.id
             WHERE decks.name = ?1",
        )?;
        let rows = stmt.query_map([deck_name], |row| {
            Ok(FlashCard {
                front: row.get(0)?,
                back: row.get(1)?,
                correct: row.get(2)?,
                current_side: FlashCardSide::Front,
            })
        })?;
        let flashcards: Result<Vec<FlashCard>> = rows.collect();
        flashcards
    }
    pub fn get_deck_count(&self) -> Result<i32> {
        let count: i32 = self
            .connection
            .query_row("SELECT COUNT(*) FROM decks", [], |row| {
                row.get(0).map(|id: i32| id)
            })?;
        Ok(count)
    }
}
