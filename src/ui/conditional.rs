use std::time::Duration;

use eframe::{
    egui::{self, menu, RichText},
    epaint::FontId,
};
use egui_notify::Toasts;

use crate::{custom_frame, Deck};

pub struct Conditional {
    pub show_add_cards: bool,
    pub add_card_front: String,
    pub add_card_back: String,
    pub show_add_deck: bool,
    pub add_deck_name: String,
    pub show_edit_deck: bool,
    pub new_deck_name: String,
    pub show_rename_deck: bool,
    pub renamed_deck: String,
    pub show_edit_cards: bool,
    pub show_edit_card: bool,
    pub edit_idx: i32,
    pub new_front: String,
    pub new_back: String,
    pub exit_program: bool,
}

impl Conditional {
    pub fn new() -> Self {
        Self {
            show_add_cards: false,
            add_card_front: String::new(),
            add_card_back: String::new(),
            show_add_deck: false,
            add_deck_name: String::new(),
            show_edit_deck: false,
            new_deck_name: String::new(),
            show_rename_deck: false,
            renamed_deck: String::new(),
            show_edit_cards: false,
            show_edit_card: false,
            edit_idx: 0,
            new_front: String::new(),
            new_back: String::new(),
            exit_program: false,
        }
    }
    pub fn ui_edit_cards(
        &mut self,
        ctx: &egui::Context,
        _frame: &mut eframe::Frame,
        deck: &mut Deck,
        toasts: &mut Toasts,
    ) {
        egui::Window::new("Edit cards")
            .frame(custom_frame())
            .open(&mut self.show_edit_cards)
            .default_pos(egui::pos2(5.0, 100.0))
            .resizable(false)
            .show(ctx, |ui| {
                egui::ScrollArea::vertical()
                    .max_height(300.0)
                    .show(ui, |ui| {
                        egui::Grid::new("edit_cards_grid")
                            .min_row_height(0.5)
                            .max_col_width(200.0)
                            .show(ui, |ui| {
                                ui.add(egui::Label::new(
                                    RichText::new("Front").font(FontId::proportional(15.0)),
                                ));
                                ui.add(egui::Label::new(
                                    RichText::new("Back").font(FontId::proportional(15.0)),
                                ));
                                ui.end_row();
                                ui.end_row();
                                let mut editing_outcome = false;
                                let mut deleted = false;
                                for (i, card) in deck.flashcards.iter().enumerate() {
                                    let i: i32 = i as i32 + 1;
                                    ui.label(&card.front);
                                    ui.label(&card.back);
                                    match i == self.edit_idx {
                                        true => {
                                            if ui
                                                .add(egui::Button::new("➡").small())
                                                .on_hover_text("Cancel")
                                                .clicked()
                                            {
                                                self.show_edit_card = false;
                                                self.edit_idx = 0;
                                            }
                                        }
                                        false => {
                                            if ui
                                                .add(egui::Button::new("✏").small())
                                                .on_hover_text("Edit")
                                                .clicked()
                                            {
                                                self.show_edit_card = true;
                                                self.edit_idx = i;
                                                self.new_front = card.front.to_owned();
                                                self.new_back = card.back.to_owned();
                                            }
                                        }
                                    }
                                    ui.end_row();
                                    if self.show_edit_card && i == self.edit_idx {
                                        ui.text_edit_multiline(&mut self.new_front);
                                        ui.text_edit_multiline(&mut self.new_back);
                                        ui.vertical(|ui| {
                                            ui.add_space(15.0);
                                            if ui.small_button("Ok").clicked() {
                                                if !deck
                                                    .db
                                                    .edit_flashcard(
                                                        self.edit_idx,
                                                        &deck.name,
                                                        &self.new_front,
                                                        &self.new_back,
                                                    )
                                                    .unwrap()
                                                {
                                                    toasts
                                                        .info("Card already exists in deck!")
                                                        .set_duration(Some(Duration::from_secs(4)));
                                                } else {
                                                    editing_outcome = true;
                                                }
                                            }
                                            ui.add_space(15.0);
                                            if ui
                                                .add(
                                                    egui::Button::new(
                                                        RichText::new("❌")
                                                            .color(egui::Color32::LIGHT_RED),
                                                    )
                                                    .small(),
                                                )
                                                .on_hover_text("Delete")
                                                .clicked()
                                            {
                                                deck.db
                                                    .remove_flashcard(&deck.name, self.edit_idx)
                                                    .unwrap();
                                                deleted = true;
                                            }
                                        });
                                        ui.end_row();
                                    }
                                    if i != deck.flashcards.len() as i32 {
                                        ui.add(egui::Separator::default().spacing(0.5));
                                        ui.add(egui::Separator::default().spacing(0.5));
                                        ui.end_row();
                                    }
                                }
                                if deleted {
                                    deck.update_flashcards();
                                    deck.update_unanswered();
                                    self.edit_idx = 0;
                                    self.show_edit_card = false;
                                }
                                if editing_outcome {
                                    deck.update_flashcards();
                                    self.edit_idx = 0;
                                    self.show_edit_card = false;
                                }
                            });
                    });
            });
        if !self.show_edit_cards {
            self.show_edit_card = false;
            self.edit_idx = 0;
        }
    }

    pub fn ui_menu(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        ui.columns(2, |columns| {
            columns[0].with_layout(egui::Layout::left_to_right(egui::Align::LEFT), |ui| {
                menu::bar(ui, |ui| {
                    ui.menu_button("File", |ui| {
                        if ui.button("Exit").clicked() {
                            self.exit_program = true;
                        }
                    });
                    ui.menu_button("Cards", |ui| {
                        if ui.button("Add cards").clicked() {
                            self.show_add_cards = true;
                        }
                        if ui.button("Edit cards").clicked() {
                            self.show_edit_cards = true;
                        }
                    });
                    ui.menu_button("Decks", |ui| {
                        if ui.button("Add decks").clicked() {
                            self.show_add_deck = true;
                        }
                        if ui.button("Edit decks").clicked() {
                            self.show_edit_deck = true;
                        }
                    });
                });
            });
            columns[1].with_layout(egui::Layout::right_to_left(egui::Align::RIGHT), |ui| {
                ui.heading("Rustentia");
            });
        });
    }

    pub fn ui_edit_deck(
        &mut self,
        ctx: &egui::Context,
        _frame: &mut eframe::Frame,
        deck: &mut Deck,
        toasts: &mut Toasts,
    ) {
        egui::Window::new("Edit decks")
            .frame(custom_frame())
            .default_pos(egui::pos2(5.0, 100.0))
            .open(&mut self.show_edit_deck)
            .resizable(false)
            .show(ctx, |ui| {
                let deck_names = deck.db.get_deck_names().unwrap();
                let last_iter = deck_names.len();
                let mut deleted_deck = String::new();
                let mut new_active_deck = String::new();
                egui::Grid::new("some_unique_id")
                    .min_col_width(250.0)
                    .min_row_height(0.5)
                    .show(ui, |ui| {
                        for (i, deck_name) in deck_names.iter().enumerate() {
                            ui.horizontal(|ui| {
                                if ui
                                    .add(
                                        egui::Button::new(
                                            egui::RichText::new("🗙")
                                                .color(egui::Color32::LIGHT_RED),
                                        )
                                        .sense(egui::Sense::click())
                                        .small(),
                                    )
                                    .on_hover_text(
                                        egui::RichText::new("Delete")
                                            .color(egui::Color32::LIGHT_RED),
                                    )
                                    .clicked()
                                {
                                    deleted_deck = deck_name.to_owned();
                                    if i >= 1 {
                                        new_active_deck = deck_names.get(i - 1).unwrap().to_owned();
                                    } else {
                                        new_active_deck = match last_iter {
                                            1 => "Default".to_owned(),
                                            _ => deck_names.get(i + 1).unwrap().to_owned(),
                                        };
                                    }
                                }
                                if ui
                                    .add(egui::Button::new("✏").small())
                                    .on_hover_text("Rename")
                                    .clicked()
                                {
                                    self.show_rename_deck = true;
                                    self.renamed_deck = deck_name.to_owned();
                                    self.new_deck_name = deck_name.to_owned();
                                }
                                ui.add_space(5.0);
                                ui.add(egui::Label::new(
                                    egui::RichText::new(deck_name.to_owned()).strong(),
                                ));
                            });
                            ui.end_row();
                            if self.show_rename_deck && deck_name.to_owned() == self.renamed_deck {
                                ui.horizontal(|ui| {
                                    ui.add(
                                        egui::TextEdit::singleline(&mut self.new_deck_name)
                                            .desired_width(150.0),
                                    );
                                    if ui.button("Ok").clicked() {
                                        if !deck
                                            .db
                                            .rename_deck(&self.renamed_deck, &self.new_deck_name)
                                            .unwrap()
                                        {
                                            toasts
                                                .info("This deck already exists!")
                                                .set_duration(Some(Duration::from_secs(4)));
                                        } else {
                                            self.show_rename_deck = false;
                                            deck.name = self.new_deck_name.to_owned();
                                        }
                                    }
                                    if ui.button("Cancel").clicked() {
                                        self.show_rename_deck = false;
                                    }
                                });
                            }
                            ui.end_row();
                            if i != last_iter - 1 {
                                ui.add(egui::Separator::default().spacing(0.5));
                                ui.end_row();
                            }
                        }
                    });
                if deleted_deck != "" {
                    if deleted_deck == deck.name {
                        deck.other_deck(new_active_deck);
                    }
                    deck.db.remove_deck(&deleted_deck).unwrap();
                }
            });
        if !self.show_edit_deck {
            self.show_rename_deck = false;
        }
    }

    pub fn ui_add_cards(
        &mut self,
        ctx: &egui::Context,
        _frame: &mut eframe::Frame,
        deck: &mut Deck,
        toasts: &mut Toasts,
    ) {
        egui::Window::new("Add cards")
            .frame(custom_frame())
            .open(&mut self.show_add_cards)
            .default_pos(egui::pos2(5.0, 100.0))
            .resizable(false)
            .show(ctx, |ui| {
                if deck.name == "Default" {
                    ui.add_space(10.0);
                    ui.label("Please create a deck first!");
                    ui.add_space(10.0);
                    ui.label("Decks -> Add deck");
                    ui.add_space(10.0);
                } else {
                    ui.label("Front:");
                    ui.text_edit_multiline(&mut self.add_card_front);
                    ui.label("Back:");
                    ui.text_edit_multiline(&mut self.add_card_back);
                    if ui.button("Add card").clicked() {
                        let adding_outcome = deck
                            .db
                            .add_flashcard(&self.add_card_front, &self.add_card_back, &deck.name)
                            .unwrap();
                        if !adding_outcome {
                            toasts
                                .info("Card already exists in deck!")
                                .set_duration(Some(Duration::from_secs(4)));
                        } else if adding_outcome {
                            deck.update_flashcards();
                            deck.update_unanswered();
                        }
                        self.add_card_front.clear();
                        self.add_card_back.clear();
                    };
                }
            });
    }

    pub fn ui_add_deck(
        &mut self,
        ctx: &egui::Context,
        _frame: &mut eframe::Frame,
        deck: &mut Deck,
        toasts: &mut Toasts,
    ) {
        egui::Window::new("Add decks")
            .frame(custom_frame())
            .open(&mut self.show_add_deck)
            .default_pos(egui::pos2(5.0, 100.0))
            .resizable(false)
            .show(ctx, |ui| {
                ui.label("Deck name:");
                ui.text_edit_multiline(&mut self.add_deck_name);
                if ui.button("Add deck").clicked() {
                    if !deck.db.add_deck(&self.add_deck_name).unwrap() {
                        toasts
                            .info("Deck already exists!")
                            .set_duration(Some(Duration::from_secs(4)));
                        self.add_deck_name.clear();
                    } else {
                        if self.add_deck_name != "" {
                            deck.other_deck(self.add_deck_name.to_owned());
                            self.add_deck_name.clear();
                        }
                    }
                };
            });
    }
}
