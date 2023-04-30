use eframe::egui::{self, Frame, Margin};
use eframe::egui::{menu, FontId, RichText};

use eframe::emath::Align2;
use eframe::epaint::{Color32, Rounding, Shadow, Stroke};
use egui_notify::Toasts;
use std::time::Duration;

use crate::deck::*;

pub struct RunApp {
    pub cd: Deck,
    pub show_add_cards: bool,
    pub add_card_front: String,
    pub add_card_back: String,
    pub show_add_deck: bool,
    pub add_deck_name: String,
    pub new_deck_name: String,
    pub toasts: Toasts,
}

impl Default for RunApp {
    fn default() -> Self {
        Self {
            cd: Deck::new(),
            show_add_cards: false,
            add_card_front: "".to_string(),
            add_card_back: "".to_string(),
            show_add_deck: false,
            add_deck_name: "".to_string(),
            toasts: Toasts::new().with_margin(egui::vec2(5.0, 70.0)),
            new_deck_name: "".to_string(),
        }
    }
}

impl RunApp {
    fn set_deck(&mut self, deck_name: String) -> Deck {
        Deck::other_deck(deck_name)
    }
}

impl eframe::App for RunApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // ctx.set_visuals(egui::style::Visuals::light());
        egui::TopBottomPanel::bottom("my_panel")
            .min_height(53.0)
            .show_separator_line(false)
            .show(ctx, |ui| {
                ui.style_mut().spacing.item_spacing = egui::vec2(8.0, 8.0);
                ui.with_layout(
                    egui::Layout::top_down_justified(egui::Align::Center),
                    |ui| {
                        if ui
                            .add(egui::Button::new(
                                RichText::new(self.cd.current_card().get_descriptor())
                                    .font(FontId::proportional(15.0)),
                            ))
                            .clicked()
                        {
                            self.cd.current_card().toggle_text();
                        };
                    },
                );
                ui.columns(2, |columns| {
                    columns[0].with_layout(
                        egui::Layout::top_down_justified(egui::Align::Center),
                        |ui| {
                            if ui
                                .add(egui::Button::new(
                                    RichText::new("Incorrect ❌ ")
                                        .color(egui::Color32::LIGHT_RED)
                                        .font(FontId::proportional(15.0)),
                                ))
                                .clicked()
                            {
                                self.cd.current_card().correct = false;
                                self.cd.next_card();
                            }
                        },
                    );
                    columns[1].with_layout(
                        egui::Layout::top_down_justified(egui::Align::Center),
                        |ui| {
                            if ui
                                .add(egui::Button::new(
                                    RichText::new("Correct ✅ ")
                                        .color(egui::Color32::LIGHT_GREEN)
                                        .font(FontId::proportional(15.0)),
                                ))
                                .clicked()
                            {
                                self.cd.current_card().correct = true;
                                self.cd.next_card();
                            }
                        },
                    );
                });
            });
        egui::TopBottomPanel::top("mypanel")
            .show_separator_line(true)
            .show(ctx, |ui| {
                ui.columns(2, |columns| {
                    columns[0].with_layout(egui::Layout::left_to_right(egui::Align::LEFT), |ui| {
                        menu::bar(ui, |ui| {
                            ui.menu_button("File", |ui| if ui.button("Open").clicked() {});
                            ui.menu_button("Cards", |ui| {
                                if ui.button("Add cards").clicked() {
                                    self.show_add_cards = true;
                                }
                                if ui.button("Edit cards").clicked() {}
                                if ui.button("Import cards").clicked() {}
                            });
                            ui.menu_button("Decks", |ui| {
                                if ui.button("Add decks").clicked() {
                                    self.show_add_deck = true;
                                }
                                if ui.button("Edit decks").clicked() {}
                                if ui.button("Import decks").clicked() {}
                            });
                            ui.menu_button("Settings", |ui| if ui.button("Open").clicked() {});
                            ui.menu_button("Help", |ui| if ui.button("Open").clicked() {});
                        });
                    });
                    columns[1].with_layout(egui::Layout::right_to_left(egui::Align::RIGHT), |ui| {
                        ui.heading("Rustentia");
                    });
                });
            });
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.with_layout(
                    egui::Layout::top_down_justified(egui::Align::Center),
                    |ui| {
                        ui.label(self.cd.name.to_string());
                        ui.add_space(200.0);
                        ui.columns(3, |columns| {
                            columns[2].with_layout(
                                egui::Layout::right_to_left(egui::Align::RIGHT),
                                |ui| {
                                    ui.add_space(20.0);
                                    egui::Window::new("Select deck")
                                        .frame(custom_frame())
                                        .anchor(Align2::RIGHT_TOP, [-5.0, 5.0])
                                        .default_open(false)
                                        .show(ctx, |ui| {
                                            let deck_count = self.cd.db.get_deck_count().unwrap();
                                            if deck_count > 1 {
                                                let mut i = 1;
                                                while i < deck_count {
                                                    let deck_name = // Skip the "Default" deck
                                                        self.cd.db.get_deck_name(i + 1).unwrap();
                                                    if ui.button(deck_name.to_string()).clicked() {
                                                        self.cd = self.set_deck(deck_name)
                                                    };
                                                    i += 1;
                                                }
                                            }
                                        });
                                },
                            );
                            columns[1].with_layout(
                                egui::Layout::top_down_justified(egui::Align::Center),
                                |ui| {
                                    ui.add(egui::Label::new(
                                        RichText::new(self.cd.current_card().get_text())
                                            .font(FontId::proportional(15.0)),
                                    ));
                                    ui.add_space(20.0);
                                    ui.separator();
                                },
                            );
                        });
                    },
                );
            });
        });
        // Conditional
        if self.show_add_cards {
            egui::Window::new("Add cards")
                .open(&mut self.show_add_cards) // Here we toggle open/not
                .default_pos(egui::pos2(5.0, 100.0))
                .show(ctx, |ui| {
                    if self.cd.name == "Default" {
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
                            let adding_outcome = self
                                .cd
                                .db
                                .add_flashcard(
                                    &self.add_card_front,
                                    &self.add_card_back,
                                    &self.cd.name,
                                )
                                .unwrap();
                            if !adding_outcome {
                                self.toasts
                                    .info("Card already exists!")
                                    .set_duration(Some(Duration::from_secs(4)));
                            } else if adding_outcome {
                                self.cd.update_flashcards();
                            }
                            self.add_card_front.clear();
                            self.add_card_back.clear();
                        };
                    }
                });
        }
        if self.show_add_deck {
            egui::Window::new("Add decks")
                .open(&mut self.show_add_deck) // Here we toggle open/not
                .default_pos(egui::pos2(5.0, 100.0))
                .show(ctx, |ui| {
                    ui.label("Deck name:");
                    ui.text_edit_multiline(&mut self.add_deck_name);
                    if ui.button("Add deck").clicked() {
                        if !self.cd.db.add_deck(&self.add_deck_name).unwrap() {
                            self.toasts
                                .info("Deck already exists!")
                                .set_duration(Some(Duration::from_secs(4)));
                            self.add_deck_name.clear();
                        }
                        if self.add_deck_name != "" {
                            self.new_deck_name = self.add_deck_name.to_string();
                        }
                    };
                });
            if self.new_deck_name != "" {
                self.cd = self.set_deck(self.new_deck_name.to_string());
                self.add_deck_name.clear();
                self.new_deck_name.clear();
            }
        }
        self.toasts.show(ctx);
    }
}

fn custom_frame() -> Frame {
    Frame {
        inner_margin: Margin {
            left: 10.0,
            right: 10.0,
            top: 7.0,
            bottom: 7.0,
        },
        outer_margin: Margin {
            left: 0.0,
            right: 0.0,
            top: 0.0,
            bottom: 0.0,
        },
        rounding: Rounding {
            nw: 2.0,
            ne: 2.0,
            sw: 2.0,
            se: 2.0,
        },
        shadow: Shadow::NONE,
        fill: Color32::default(),
        stroke: Stroke {
            width: 0.5,
            color: Color32::DARK_GRAY,
        },
    }
}
