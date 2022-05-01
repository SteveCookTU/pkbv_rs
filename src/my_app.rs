use crate::drawing::{draw_left_column, draw_right_column};
use crate::load_bv::load_bv;
use crate::pk6::Pk6;
use crate::pokemon_image::PokemonImage;
use eframe::egui;
use eframe::egui::{vec2, CentralPanel, Context, ScrollArea, Sense, TextEdit, Vec2};
use eframe::epi::{App, Frame};
use egui_extras::RetainedImage;
use std::fs::File;
use std::io::Read;

pub struct MyApp {
    pub(crate) pkmn_images: [Option<RetainedImage>; 12],
    pub(crate) versus_text: String,
    pub(crate) summary_text: String,
    pub(crate) instruction_text: String,
    pub(crate) recorded_at: String,
    pub(crate) uploaded_at: String,
    pub(crate) mode: String,
    pub(crate) style: String,
    pub(crate) debug_1: String,
    pub(crate) debug_2: String,
    pub(crate) ra: String,
    pub(crate) r0: String,
    pub(crate) rm: String,
    pub(crate) rn: String,
}

impl MyApp {
    pub fn reset(&mut self) {
        self.pkmn_images = [
            None, None, None, None, None, None, None, None, None, None, None, None,
        ];
        self.versus_text = "".to_string();
        self.summary_text = "".to_string();
        self.instruction_text = "".to_string();
        self.recorded_at = "".to_string();
        self.uploaded_at = "".to_string();
        self.mode = "".to_string();
        self.style = "".to_string();
        self.debug_1 = "".to_string();
        self.debug_2 = "".to_string();
        self.ra = "".to_string();
        self.r0 = "".to_string();
        self.rm = "".to_string();
        self.rn = "".to_string();
    }
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            pkmn_images: [
                None, None, None, None, None, None, None, None, None, None, None, None,
            ],
            versus_text: "".to_string(),
            summary_text: "test\ntest\ntest\ntest\ntest\ntest\ntest\ntest\ntest\ntest".to_string(),
            instruction_text: "".to_string(),
            recorded_at: "".to_string(),
            uploaded_at: "".to_string(),
            mode: "".to_string(),
            style: "".to_string(),
            debug_1: "".to_string(),
            debug_2: "".to_string(),
            ra: "".to_string(),
            r0: "".to_string(),
            rm: "".to_string(),
            rn: "".to_string(),
        }
    }
}

impl App for MyApp {
    fn update(&mut self, ctx: &Context, frame: &Frame) {
        CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                draw_left_column(self, ui, ctx);
                ui.separator();
                draw_right_column(self, ui);
            });
        });

        // Resize the native window to be just the size we need it to be:
        frame.set_window_size(ctx.used_size())
    }

    fn name(&self) -> &str {
        "pkBV-rs"
    }
}
