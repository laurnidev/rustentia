use eframe::{
    egui::{self, RichText},
    emath::Align2,
    epaint::FontId,
};

use crate::{custom_frame, Deck};

pub fn ui_card_buttons(ui: &mut egui::Ui, _frame: &mut eframe::Frame, deck: &mut Deck) {
    if ui.add(egui::Button::new("Reset ⟲")).clicked() {
        deck.reset_deck();
    }
    ui.add(egui::Label::new(
        RichText::new(format!("Correct: {}", deck.correct_count)).font(FontId::proportional(15.0)),
    ));
    ui.add(egui::Label::new(
        RichText::new(format!("Incorrect: {}", deck.incorrect_count))
            .font(FontId::proportional(15.0)),
    ));
    ui.add(egui::Label::new(
        RichText::new(format!("Unanswered: {}", deck.unanswered_count))
            .font(FontId::proportional(15.0)),
    ));
    ui.add_space(8.0);
    ui.style_mut().spacing.item_spacing = egui::vec2(8.0, 8.0);
    ui.with_layout(
        egui::Layout::top_down_justified(egui::Align::Center),
        |ui| {
            if ui
                .add(egui::Button::new(
                    RichText::new(deck.current_card().get_descriptor())
                        .font(FontId::proportional(15.0)),
                ))
                .clicked()
            {
                deck.current_card().toggle_text();
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
                    deck.current_card().correct = false;
                    let fc_len = deck.flashcards.len();
                    if deck.incorrect_count != fc_len
                        && fc_len - deck.correct_count > deck.incorrect_count
                    {
                        deck.incorrect_count += 1;
                    }
                    if deck.unanswered_count != 0 {
                        deck.unanswered_count -= 1;
                    }
                    deck.next_card();
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
                    deck.current_card().correct = true;
                    deck.correct_count += 1;
                    if deck.incorrect_count != 0 && deck.unanswered_count == 0 {
                        deck.incorrect_count -= 1;
                    }
                    if deck.unanswered_count != 0 {
                        deck.unanswered_count -= 1;
                    }
                    deck.next_card();
                }
            },
        );
    });
}

pub fn ui_middle(
    ctx: &egui::Context,
    ui: &mut egui::Ui,
    _frame: &mut eframe::Frame,
    deck: &mut Deck,
) {
    egui::ScrollArea::vertical().show(ui, |ui| {
        ui.with_layout(
            egui::Layout::top_down_justified(egui::Align::Center),
            |ui| {
                ui.label(format!("{}", deck.name.to_owned()));
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
                                        let deck_names = deck.db.get_deck_names().unwrap();
                                        let last_iter = deck_names.len();
                                        for (i, deck_name) in deck_names.iter().enumerate() {
                                            ui.horizontal(|ui| {
                                                if deck_name.to_owned() != deck.name {
                                                    if ui
                                                        .add(
                                                            egui::Button::new("◻")
                                                                .sense(egui::Sense::click())
                                                                .small(),
                                                        )
                                                        .clicked()
                                                    {
                                                        deck.other_deck(deck_name.to_owned());
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
                                                if deck_name.to_owned() != deck.name {
                                                    ui.add_space(5.0);
                                                    ui.add(egui::Label::new(egui::RichText::new(
                                                        deck_name.to_owned(),
                                                    )));
                                                } else {
                                                    ui.add_space(5.0);
                                                    ui.add(egui::Label::new(
                                                        egui::RichText::new(deck_name.to_owned())
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
                                RichText::new(deck.current_card().get_text())
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
