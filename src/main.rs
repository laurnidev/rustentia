#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use eframe::egui;
use eframe::egui::{menu, FontId, RichText};

fn main() -> Result<(), eframe::Error> {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(1280.0, 720.0)),
        ..Default::default()
    };
    eframe::run_native(
        "Floating window",
        options,
        Box::new(|_cc| Box::<RunApp>::default()),
    )
}

struct RunApp {
    front: String,
    back: String,
    back_text: String,
    front_text: String,
    show_definition: bool,
}

impl Default for RunApp {
    fn default() -> Self {
        Self {
            front: "Hello".to_string(),
            back: "\"Hello\" is a common greeting used in English to acknowledge the presence of someone or to initiate a conversation. It is a shortened form of the greeting \"Hello, how are you?\" and is typically used in informal settings or as a polite and friendly greeting in more formal situations.".to_string(),
            back_text: "Show front".to_string(),
            front_text: "Show back".to_string(),
            show_definition: false,
        }
    }
}

impl RunApp {
    fn get_text(&self) -> &str {
        if self.show_definition {
            &self.back
        } else {
            &self.front
        }
    }
    fn toggle_text(&mut self) {
        self.show_definition = !self.show_definition;
    }
    fn get_descriptor(&self) -> &str {
        if self.show_definition {
            &self.back_text
        } else {
            &self.front_text
        }
    }
}

impl eframe::App for RunApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // egui::SidePanel::left("my_left_panel")
        //     .min_width(200.0)
        //     .resizable(false)
        //     .show_separator_line(false)
        //     .show(ctx, |ui| {
        //         ui.label("Hello World!");
        //     });
        // egui::SidePanel::right("my_right_panel")
        //     .min_width(200.0)
        //     .resizable(false)
        //     .show_separator_line(false)
        //     .show(ctx, |ui| {
        //         ui.label("Hello World!");
        //     });
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
                                RichText::new(self.get_descriptor())
                                    .font(FontId::proportional(15.0)),
                            ))
                            .clicked()
                        {
                            self.toggle_text();
                        };
                    },
                );
                ui.columns(2, |columns| {
                    columns[0].with_layout(
                        egui::Layout::top_down_justified(egui::Align::Center),
                        |ui| {
                            ui.add(egui::Button::new(
                                RichText::new("Incorrect ❌ ")
                                    .color(egui::Color32::LIGHT_RED)
                                    .font(FontId::proportional(15.0)),
                            ))
                            .clicked();
                        },
                    );
                    columns[1].with_layout(
                        egui::Layout::top_down_justified(egui::Align::Center),
                        |ui| {
                            ui.add(egui::Button::new(
                                RichText::new("Correct ✅ ")
                                    .color(egui::Color32::LIGHT_GREEN)
                                    .font(FontId::proportional(15.0)),
                            ))
                            .clicked();
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
                                        RichText::new(self.get_text())
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
