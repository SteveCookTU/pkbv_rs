use crate::drawing::{draw_left_column, draw_right_column};
use crate::load_bv::load_bv;
use crate::pk6::Pk6;
use crate::pokemon_image::PokemonImage;
use eframe::egui::{vec2, CentralPanel, Context, ScrollArea, Sense, TextEdit, Vec2, Visuals};
use eframe::{egui, App, CreationContext, Frame};
use egui_extras::RetainedImage;
use std::fs::File;
use std::io::Read;

fn setup_custom_fonts(ctx: &Context) {
    // Start with the default fonts (we will be adding to them rather than replacing them).
    let mut fonts = egui::FontDefinitions::default();

    // Install my own font (maybe supporting non-latin characters).
    // .ttf and .otf files supported.
    fonts.font_data.insert(
        "aquafont".to_owned(),
        egui::FontData::from_static(include_bytes!("aquafont.ttf")),
    );

    // Put my font first (highest priority) for proportional text:
    fonts
        .families
        .entry(egui::FontFamily::Proportional)
        .or_default()
        .push("aquafont".to_owned());

    // Put my font as last fallback for monospace:
    fonts
        .families
        .entry(egui::FontFamily::Monospace)
        .or_default()
        .push("aquafont".to_owned());

    // Tell egui to use these fonts:
    ctx.set_fonts(fonts);
}

#[derive(Default)]
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
    pub fn new(cc: &CreationContext<'_>) -> Self {
        setup_custom_fonts(&cc.egui_ctx);
        cc.egui_ctx.set_visuals(Visuals::dark());
        Self::default()
    }

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

impl App for MyApp {
    fn update(&mut self, ctx: &Context, frame: &mut Frame) {
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
}
