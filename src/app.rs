use eframe::egui;
use eframe::egui::{menu, FontId, RichText};

use crate::deck::*;

pub struct RunApp {
    pub cd: Deck,
}

impl Default for RunApp {
    fn default() -> Self {
        Self {
            cd: Deck::new(&mut "Default".to_string()),
        }
    }
}

impl eframe::App for RunApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::bottom("my_panel")
            .min_height(53.0)
            .show_separator_line(false)
            .show(ctx, |ui| {
                // ctx.set_visuals(egui::style::Visuals::light());
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
                                if ui.button("Add cards").clicked() {}
                                if ui.button("Edit cards").clicked() {}
                                if ui.button("Import cards").clicked() {}
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
                        ui.add_space(200.0);
                        ui.columns(3, |columns| {
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
    }
}
