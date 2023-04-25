use crate::database::*;
use crate::flashcard::*;

pub struct Deck {
    pub name: String,
    pub flashcards: Vec<FlashCard>,
    pub db: Database,
    pub idx: usize,
}

impl Deck {
    pub fn new(name: &str) -> Self {
        let database = match Database::new() {
            Ok(db) => db,
            Err(e) => {
                eprintln!("Error fetching deck: {}", e);
                std::process::exit(1);
            }
        };
        database.create_tables().unwrap();
        // println!("{:?}", database.get_all_decks());
        // println!("{:?}", database.get_flashcards(name));
        Self {
            name: name.to_string(),
            flashcards: database.get_flashcards(&name).unwrap(),
            db: database,
            idx: 0,
        }
    }
    pub fn current_card(&mut self) -> &mut FlashCard {
        &mut self.flashcards[self.idx]
    }
    pub fn next_card(&mut self) {
        if self.idx < self.flashcards.len() - 1 {
            self.idx += 1;
        }
    }
}
