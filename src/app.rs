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
    pub show_edit_deck: bool,
    pub toasts: Toasts,
    pub new_deck_name: String,
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
            show_edit_deck: false,
            new_deck_name: "".to_string(),
        }
    }
}

impl eframe::App for RunApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // ctx.set_visuals(egui::style::Visuals::light());
        egui::TopBottomPanel::bottom("Buttons Panel")
            .min_height(53.0)
            .show_separator_line(false)
            .show(ctx, |ui| {
                ui_card_buttons(ui, _frame, &mut self.cd);
            });

        egui::TopBottomPanel::top("Menu Panel")
            .show_separator_line(true)
            .show(ctx, |ui| ui_menu(ui, _frame, self));

        egui::CentralPanel::default().show(ctx, |ui| {
            ui_middle(ctx, ui, _frame, self);
        });
        // Conditional
        ui_add_cards(ctx, _frame, self);
        ui_add_deck(ctx, _frame, self);
        ui_edit_deck(ctx, _frame, self);
        self.toasts.show(ctx);
    }
}

fn custom_frame() -> Frame {
    Frame {
        inner_margin: Margin {
            left: 5.0,
            right: 5.0,
            top: 5.0,
            bottom: 5.0,
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
        fill: Color32::from_rgb(40, 40, 40),
        stroke: Stroke {
            width: 0.5,
            color: Color32::DARK_GRAY,
        },
    }
}

fn ui_middle(
    ctx: &egui::Context,
    ui: &mut egui::Ui,
    _frame: &mut eframe::Frame,
    run_app: &mut RunApp,
) {
    egui::ScrollArea::vertical().show(ui, |ui| {
        ui.with_layout(
            egui::Layout::top_down_justified(egui::Align::Center),
            |ui| {
                ui.label(run_app.cd.name.to_string());
                ui.add_space(200.0);
                ui.columns(3, |columns| {
                    columns[2].with_layout(egui::Layout::right_to_left(egui::Align::RIGHT), |ui| {
                        ui.add_space(20.0);
                        egui::Window::new("Select deck")
                            .resizable(false)
                            .frame(custom_frame())
                            .anchor(Align2::RIGHT_TOP, [-5.0, 5.0])
                            .default_open(false)
                            .show(ctx, |ui| {
                                egui::Grid::new("some_unique_id").min_row_height(0.5).show(
                                    ui,
                                    |ui| {
                                        let deck_names = run_app.cd.db.get_deck_names().unwrap();
                                        let last_iter = deck_names.len();
                                        for (i, deck_name) in deck_names.iter().enumerate() {
                                            ui.horizontal(|ui| {
                                                if deck_name.to_string() != run_app.cd.name {
                                                    if ui
                                                        .add(
                                                            egui::Button::new("◻")
                                                                .sense(egui::Sense::click())
                                                                .small(),
                                                        )
                                                        .clicked()
                                                    {
                                                        run_app
                                                            .cd
                                                            .other_deck(deck_name.to_string());
                                                    }
                                                } else {
                                                    if ui
                                                        .add(
                                                            egui::Button::new("◼")
                                                                .sense(egui::Sense::click())
                                                                .small(),
                                                        )
                                                        .clicked()
                                                    {
                                                    }
                                                }
                                                if deck_name.to_string() != run_app.cd.name {
                                                    ui.add_space(5.0);
                                                    ui.add(egui::Label::new(egui::RichText::new(
                                                        deck_name.to_string(),
                                                    )));
                                                } else {
                                                    ui.add_space(5.0);
                                                    ui.add(egui::Label::new(
                                                        egui::RichText::new(deck_name.to_string())
                                                            .strong(),
                                                    ));
                                                }
                                            });
                                            ui.end_row();
                                            if i != last_iter - 1 {
                                                ui.add(egui::Separator::default().spacing(0.5));
                                                ui.end_row();
                                            }
                                        }
                                    },
                                );
                            });
                    });
                    columns[1].with_layout(
                        egui::Layout::top_down_justified(egui::Align::Center),
                        |ui| {
                            ui.add(egui::Label::new(
                                RichText::new(run_app.cd.current_card().get_text())
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
}

fn ui_menu(ui: &mut egui::Ui, _frame: &mut eframe::Frame, run_app: &mut RunApp) {
    ui.columns(2, |columns| {
        columns[0].with_layout(egui::Layout::left_to_right(egui::Align::LEFT), |ui| {
            menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| if ui.button("Open").clicked() {});
                ui.menu_button("Cards", |ui| {
                    if ui.button("Add cards").clicked() {
                        run_app.show_add_cards = true;
                    }
                    if ui.button("Edit cards").clicked() {}
                    if ui.button("Import cards").clicked() {}
                });
                ui.menu_button("Decks", |ui| {
                    if ui.button("Add decks").clicked() {
                        run_app.show_add_deck = true;
                    }
                    if ui.button("Edit decks").clicked() {
                        run_app.show_edit_deck = true;
                    }
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
}

fn ui_card_buttons(ui: &mut egui::Ui, _frame: &mut eframe::Frame, cd: &mut Deck) {
    ui.style_mut().spacing.item_spacing = egui::vec2(8.0, 8.0);
    ui.with_layout(
        egui::Layout::top_down_justified(egui::Align::Center),
        |ui| {
            if ui
                .add(egui::Button::new(
                    RichText::new(cd.current_card().get_descriptor())
                        .font(FontId::proportional(15.0)),
                ))
                .clicked()
            {
                cd.current_card().toggle_text();
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
                    cd.current_card().correct = false;
                    cd.next_card();
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
                    cd.current_card().correct = true;
                    cd.next_card();
                }
            },
        );
    });
}

fn ui_edit_deck(ctx: &egui::Context, _frame: &mut eframe::Frame, run_app: &mut RunApp) {
    egui::Window::new("Edit decks")
        .frame(custom_frame())
        .default_pos(egui::pos2(5.0, 100.0))
        .open(&mut run_app.show_edit_deck)
        .resizable(false)
        .show(ctx, |ui| {
            let deck_names = run_app.cd.db.get_deck_names().unwrap();
            let last_iter = deck_names.len();
            let mut deleted_deck = String::new();
            let mut new_active_deck = String::new();
            egui::Grid::new("some_unique_id")
                .min_row_height(0.5)
                .show(ui, |ui| {
                    for (i, deck_name) in deck_names.iter().enumerate() {
                        ui.horizontal(|ui| {
                            if ui
                                .add(
                                    egui::Button::new(
                                        egui::RichText::new("🗙").color(egui::Color32::LIGHT_RED),
                                    )
                                    .sense(egui::Sense::click())
                                    .small(),
                                )
                                .on_hover_text(
                                    egui::RichText::new("Delete").color(egui::Color32::LIGHT_RED),
                                )
                                .clicked()
                            {
                                deleted_deck = deck_name.to_string();
                                if i >= 1 {
                                    new_active_deck = deck_names.get(i - 1).unwrap().to_string();
                                } else if last_iter == 1 {
                                    new_active_deck = "Default".to_string();
                                } else if i == 0 {
                                    new_active_deck = deck_names.get(i + 1).unwrap().to_string();
                                }
                            }
                            if ui
                                .add(egui::Button::new("✏").small())
                                .on_hover_text("Rename")
                                .clicked()
                            {}
                            ui.add_space(5.0);
                            ui.add(egui::Label::new(
                                egui::RichText::new(deck_name.to_string()).strong(),
                            ));
                        });
                        ui.end_row();
                        if i != last_iter - 1 {
                            ui.add(egui::Separator::default().spacing(0.5));
                            ui.end_row();
                        }
                    }
                });
            if deleted_deck != "" {
                if deleted_deck == run_app.cd.name {
                    run_app.cd.other_deck(new_active_deck);
                }
                run_app.cd.db.remove_deck(&deleted_deck).unwrap();
            }
        });
}

fn ui_add_cards(ctx: &egui::Context, _frame: &mut eframe::Frame, run_app: &mut RunApp) {
    egui::Window::new("Add cards")
        .frame(custom_frame())
        .open(&mut run_app.show_add_cards)
        .default_pos(egui::pos2(5.0, 100.0))
        .resizable(false)
        .show(ctx, |ui| {
            if run_app.cd.name == "Default" {
                ui.add_space(10.0);
                ui.label("Please create a deck first!");
                ui.add_space(10.0);
                ui.label("Decks -> Add deck");
                ui.add_space(10.0);
            } else {
                ui.label("Front:");
                ui.text_edit_multiline(&mut run_app.add_card_front);
                ui.label("Back:");
                ui.text_edit_multiline(&mut run_app.add_card_back);
                if ui.button("Add card").clicked() {
                    let adding_outcome = run_app
                        .cd
                        .db
                        .add_flashcard(
                            &run_app.add_card_front,
                            &run_app.add_card_back,
                            &run_app.cd.name,
                        )
                        .unwrap();
                    if !adding_outcome {
                        run_app
                            .toasts
                            .info("Card already exists!")
                            .set_duration(Some(Duration::from_secs(4)));
                    } else if adding_outcome {
                        run_app.cd.update_flashcards();
                    }
                    run_app.add_card_front.clear();
                    run_app.add_card_back.clear();
                };
            }
        });
}

fn ui_add_deck(ctx: &egui::Context, _frame: &mut eframe::Frame, run_app: &mut RunApp) {
    egui::Window::new("Add decks")
        .frame(custom_frame())
        .open(&mut run_app.show_add_deck)
        .default_pos(egui::pos2(5.0, 100.0))
        .resizable(false)
        .show(ctx, |ui| {
            ui.label("Deck name:");
            ui.text_edit_multiline(&mut run_app.add_deck_name);
            if ui.button("Add deck").clicked() {
                if !run_app.cd.db.add_deck(&run_app.add_deck_name).unwrap() {
                    run_app
                        .toasts
                        .info("Deck already exists!")
                        .set_duration(Some(Duration::from_secs(4)));
                    run_app.add_deck_name.clear();
                } else {
                    if run_app.add_deck_name != "" {
                        run_app.cd.other_deck(run_app.add_deck_name.to_string());
                        run_app.add_deck_name.clear();
                    }
                }
            };
        });
}
