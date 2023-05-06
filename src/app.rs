use eframe::egui;

use egui_notify::Toasts;

use crate::conditional::*;
use crate::deck::*;
use crate::static_ui::{ui_card_buttons, ui_middle};

pub struct RunApp {
    pub dk: Deck,
    pub cd: Conditional,
    pub toasts: Toasts,
}

impl Default for RunApp {
    fn default() -> Self {
        Self {
            dk: Deck::new(),
            cd: Conditional::new(),
            toasts: Toasts::new().with_margin(egui::vec2(5.0, 70.0)),
        }
    }
}

impl eframe::App for RunApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // ctx.set_visuals(egui::style::Visuals::light());
        let deck = &mut self.dk;
        let toasts = &mut self.toasts;
        egui::TopBottomPanel::bottom("Buttons Panel")
            .min_height(53.0)
            .show_separator_line(false)
            .show(ctx, |ui| {
                ui_card_buttons(ui, _frame, deck);
            });
        egui::TopBottomPanel::top("Menu Panel")
            .show_separator_line(true)
            .show(ctx, |ui| self.cd.ui_menu(ui, _frame));

        egui::CentralPanel::default().show(ctx, |ui| {
            ui_middle(ctx, ui, _frame, deck);
        });
        self.cd.ui_add_cards(ctx, _frame, deck, toasts);
        self.cd.ui_add_deck(ctx, _frame, deck, toasts);
        self.cd.ui_edit_deck(ctx, _frame, deck, toasts);
        self.cd.ui_edit_cards(ctx, _frame, deck, toasts);
        self.toasts.show(ctx);
    }
}
