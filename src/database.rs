use rusqlite::{Connection, Result};

use crate::flashcard::{FlashCard, FlashCardSide};

#[derive(Debug)]
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
             card_idx INTEGER,
             FOREIGN KEY(deck_id) REFERENCES decks(id) ON DELETE CASCADE
         )",
            [],
        )?;
        Ok(())
    }

    pub fn remove_deck(&mut self, deck_name: &str) -> Result<()> {
        let deck_id: i64 = self.connection.query_row(
            "SELECT id FROM decks WHERE name = ?1",
            &[deck_name],
            |row| row.get(0),
        )?;
        let tx = self.connection.transaction()?;
        tx.execute("DELETE FROM decks WHERE id = ?1", [deck_id])?;
        tx.commit()?;
        Ok(())
    }

    pub fn add_deck(&self, name: &str) -> Result<bool> {
        if !self.deck_is_unique(name)? {
            return Ok(false);
        }
        self.connection
            .execute("INSERT INTO decks (name) VALUES (?1)", &[name])?;
        Ok(true)
    }

    pub fn add_flashcard(&self, front: &str, back: &str, deck_name: &str) -> Result<bool> {
        if !self.card_is_unique(&front, &back, &deck_name)? {
            return Ok(false);
        }
        let deck_id: String = self.connection.query_row(
            "SELECT id FROM decks WHERE name = ?1",
            &[deck_name],
            |row| row.get(0).map(|id: i64| id.to_string()),
        )?;
        self.connection.execute(
            "INSERT INTO flashcards (front, back, correct, current_side, deck_id, card_idx)
            VALUES (?1, ?2, false, 0, ?3, ?4)",
            (
                front,
                back,
                &deck_id,
                self.get_card_count(deck_name).unwrap() + 1,
            ),
        )?;
        Ok(true)
    }

    pub fn get_deck_names(&self) -> Result<Vec<String>> {
        let mut stmt = self
            .connection
            .prepare("SELECT name FROM decks WHERE name != 'Default'")?;
        let rows = stmt.query_map([], |row| Ok(row.get::<usize, String>(0)?))?;
        let mut decks = Vec::new();
        for name in rows {
            decks.push(name?);
        }
        Ok(decks)
    }

    pub fn get_flashcards(&self, deck_name: &str) -> Result<Vec<FlashCard>> {
        let mut stmt = self.connection.prepare(
            "SELECT front, back, correct, card_idx FROM flashcards
             INNER JOIN decks ON flashcards.deck_id = decks.id
             WHERE decks.name = ?1",
        )?;
        let rows = stmt.query_map([deck_name], |row| {
            Ok(FlashCard {
                front: row.get(0)?,
                back: row.get(1)?,
                correct: row.get(2)?,
                current_side: FlashCardSide::Front,
                idx: row.get(3)?,
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

    pub fn get_card_count(&self, name: &str) -> Result<i32> {
        let count: i32 = self.connection.query_row(
            "SELECT COUNT(*) FROM flashcards
             INNER JOIN decks ON flashcards.deck_id = decks.id
             WHERE decks.name = ?1",
            &[name],
            |row| row.get(0).map(|id: i32| id),
        )?;
        Ok(count)
    }

    fn deck_is_unique(&self, name: &str) -> Result<bool> {
        let count: i32 = self.connection.query_row(
            "SELECT COUNT(*) FROM decks
             WHERE name = ?1",
            &[name],
            |row| row.get(0).map(|id: i32| id),
        )?;
        Ok(count == 0)
    }

    fn card_is_unique(&self, front: &str, back: &str, deck_name: &str) -> Result<bool> {
        let count: i32 = self.connection.query_row(
            "SELECT COUNT(*) FROM flashcards
             INNER JOIN decks ON flashcards.deck_id = decks.id
             WHERE front = ?1 AND back = ?2 AND decks.name = ?3",
            (front, back, deck_name),
            |row| row.get(0).map(|id: i32| id),
        )?;
        Ok(count == 0)
    }
}
