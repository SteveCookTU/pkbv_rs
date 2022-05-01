use eframe::egui::{Context, ScrollArea, Sense, TextEdit, Ui, vec2, Vec2};
use crate::load_bv::load_bv;
use crate::my_app::MyApp;
use crate::pokemon_image::PokemonImage;

pub fn draw_left_column(app: &mut MyApp, ui: &mut Ui, ctx: &Context) {
    ui.vertical(|ui| {
        ui.spacing_mut().item_spacing = vec2(0.0, 10.0);
        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing = vec2(0.0, 0.0);
            let padding_size = Vec2::new(50.0 * 2.0, 0.0);
            let (rect, _) = ui.allocate_exact_size(padding_size, Sense::hover());
            ui.painter()
                .rect_filled(rect, 0.0, ui.style().noninteractive().bg_fill);
            if let Some(image) = app.pkmn_images[6].as_ref() {
                ui.add(PokemonImage::new(
                    Some(image.texture_id(ctx)),
                    image.size_vec2(),
                ));
            } else {
                ui.add(PokemonImage::new(None, Vec2::new(0.0, 0.0)));
            }
            if let Some(image) = app.pkmn_images[7].as_ref() {
                ui.add(PokemonImage::new(
                    Some(image.texture_id(ctx)),
                    image.size_vec2(),
                ));
            } else {
                ui.add(PokemonImage::new(None, Vec2::new(0.0, 0.0)));
            }
            if let Some(image) = app.pkmn_images[8].as_ref() {
                ui.add(PokemonImage::new(
                    Some(image.texture_id(ctx)),
                    image.size_vec2(),
                ));
            } else {
                ui.add(PokemonImage::new(None, Vec2::new(0.0, 0.0)));
            }
            if let Some(image) = app.pkmn_images[9].as_ref() {
                ui.add(PokemonImage::new(
                    Some(image.texture_id(ctx)),
                    image.size_vec2(),
                ));
            } else {
                ui.add(PokemonImage::new(None, Vec2::new(0.0, 0.0)));
            }
            if let Some(image) = app.pkmn_images[10].as_ref() {
                ui.add(PokemonImage::new(
                    Some(image.texture_id(ctx)),
                    image.size_vec2(),
                ));
            } else {
                ui.add(PokemonImage::new(None, Vec2::new(0.0, 0.0)));
            }
            if let Some(image) = app.pkmn_images[11].as_ref() {
                ui.add(PokemonImage::new(
                    Some(image.texture_id(ctx)),
                    image.size_vec2(),
                ));
            } else {
                ui.add(PokemonImage::new(None, Vec2::new(0.0, 0.0)));
            }
        });
        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing = vec2(0.0, 0.0);
            let padding_size = Vec2::new((50.0 * 2.0) + 25.0, 0.0);
            let (rect, _) = ui.allocate_exact_size(padding_size, Sense::hover());
            ui.painter()
                .rect_filled(rect, 0.0, ui.style().noninteractive().bg_fill);
            ui.label(&app.versus_text);
        });
        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing = vec2(0.0, 5.0);
            if let Some(image) = app.pkmn_images[0].as_ref() {
                ui.add(PokemonImage::new(
                    Some(image.texture_id(ctx)),
                    image.size_vec2(),
                ));
            } else {
                ui.add(PokemonImage::new(None, Vec2::new(0.0, 0.0)));
            }
            if let Some(image) = app.pkmn_images[1].as_ref() {
                ui.add(PokemonImage::new(
                    Some(image.texture_id(ctx)),
                    image.size_vec2(),
                ));
            } else {
                ui.add(PokemonImage::new(None, Vec2::new(0.0, 0.0)));
            }
            if let Some(image) = app.pkmn_images[2].as_ref() {
                ui.add(PokemonImage::new(
                    Some(image.texture_id(ctx)),
                    image.size_vec2(),
                ));
            } else {
                ui.add(PokemonImage::new(None, Vec2::new(0.0, 0.0)));
            }
            if let Some(image) = app.pkmn_images[3].as_ref() {
                ui.add(PokemonImage::new(
                    Some(image.texture_id(ctx)),
                    image.size_vec2(),
                ));
            } else {
                ui.add(PokemonImage::new(None, Vec2::new(0.0, 0.0)));
            }
            if let Some(image) = app.pkmn_images[4].as_ref() {
                ui.add(PokemonImage::new(
                    Some(image.texture_id(ctx)),
                    image.size_vec2(),
                ));
            } else {
                ui.add(PokemonImage::new(None, Vec2::new(0.0, 0.0)));
            }
            if let Some(image) = app.pkmn_images[5].as_ref() {
                ui.add(PokemonImage::new(
                    Some(image.texture_id(ctx)),
                    image.size_vec2(),
                ));
            } else {
                ui.add(PokemonImage::new(None, Vec2::new(0.0, 0.0)));
            }
        });
        ScrollArea::vertical()
            .min_scrolled_height(200.0)
            .max_height(200.0)
            .max_width(400.0)
            .show(ui, |ui| {
                ui.add(
                    TextEdit::multiline(&mut app.summary_text)
                        .desired_width(400.0)
                        .desired_rows(10)
                        .interactive(false)
                        .frame(false),
                );
            });
        if ui.button("Load File").clicked() {
            if let Some(path) = rfd::FileDialog::new().pick_file() {
                load_bv(app, path);
            }
        };
    });
}
