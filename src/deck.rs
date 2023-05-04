use crate::database::*;
use crate::flashcard::*;

#[derive(Debug)]
pub struct Deck {
    pub name: String,
    pub flashcards: Vec<FlashCard>,
    pub db: Database,
    pub idx: usize,
    pub default_card: FlashCard,
}

impl Deck {
    pub fn new() -> Self {
        let name: &str = "Default";
        let database = match Database::new() {
            Ok(db) => db,
            Err(e) => {
                eprintln!("Error fetching deck: {}", e);
                std::process::exit(1);
            }
        };
        database.create_tables().unwrap();
        // Check if there are any decks in the database, if not, create a default deck
        if database.get_deck_count().unwrap() == 0 {
            database.add_deck(name).unwrap();
            database
                .add_flashcard(
                    "First, create a deck: Decks -> Add deck, then click 'Show back'",
                    "Now add your first cards: Cards -> Add cards",
                    name,
                )
                .unwrap();
        }
        let default_card = FlashCard {
            front: "".to_string(),
            back: "".to_string(),
            correct: false,
            current_side: FlashCardSide::Front,
            idx: 0,
        };
        Self {
            name: name.to_string(),
            flashcards: database.get_flashcards(&name).unwrap(),
            db: database,
            idx: 0,
            default_card,
        }
    }

    pub fn other_deck(&mut self, deck_name: String) {
        self.flashcards = self.db.get_flashcards(&deck_name).unwrap();
        self.name = deck_name;
        self.idx = 0;
    }

    pub fn current_card(&mut self) -> &mut FlashCard {
        if self.flashcards.len() == 0 {
            &mut self.default_card
        } else {
            &mut self.flashcards[self.idx]
        }
    }

    pub fn next_card(&mut self) {
        let fc_len = self.flashcards.len();
        if self.idx < fc_len {
            self.idx += 1;
            for idx in self.idx..fc_len {
                if !self.flashcards[idx].correct {
                    self.idx = idx;
                    return;
                }
            }
        }
        self.idx = 0;
        for idx in 0..fc_len {
            if !self.flashcards[idx].correct {
                self.idx = idx;
                return;
            }
        }
        for card in &mut self.flashcards {
            card.correct = false;
        }
        self.idx = 0;
    }

    pub fn update_flashcards(&mut self) {
        self.flashcards = self.db.get_flashcards(&self.name).unwrap();
    }
}
