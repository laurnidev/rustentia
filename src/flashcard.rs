#[derive(Debug)]
pub enum FlashCardSide {
    Front,
    Back,
}

#[derive(Debug)]
pub struct FlashCard {
    pub front: String,
    pub back: String,
    pub correct: bool,
    pub current_side: FlashCardSide,
    pub idx: usize,
}

impl FlashCard {
    pub fn get_text(&self) -> &str {
        match self.current_side {
            FlashCardSide::Front => &self.front,
            FlashCardSide::Back => &self.back,
        }
    }

    pub fn get_descriptor(&self) -> &str {
        match self.current_side {
            FlashCardSide::Front => "Show back",
            FlashCardSide::Back => "Show front",
        }
    }

    pub fn toggle_text(&mut self) {
        match self.current_side {
            FlashCardSide::Front => self.current_side = FlashCardSide::Back,
            FlashCardSide::Back => self.current_side = FlashCardSide::Front,
        }
    }
}
