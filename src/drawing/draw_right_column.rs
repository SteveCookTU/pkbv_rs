use crate::my_app::MyApp;

use eframe::egui::{ScrollArea, TextEdit, Ui};

pub fn draw_right_column(app: &mut MyApp, ui: &mut Ui) {
    ui.vertical(|ui| {
        ui.horizontal(|ui| {
            ui.label("Recorded:");
            ui.add(
                TextEdit::singleline(&mut app.recorded_at)
                    .desired_width(200.0)
                    .frame(false)
                    .interactive(false),
            );
            ui.add(
                TextEdit::singleline(&mut app.style)
                    .desired_width(150.0)
                    .frame(false)
                    .interactive(false),
            );
        });
        ui.horizontal(|ui| {
            ui.label("Uploaded:");
            ui.add(
                TextEdit::singleline(&mut app.recorded_at)
                    .desired_width(200.0)
                    .frame(false)
                    .interactive(false),
            );
            ui.add(
                TextEdit::singleline(&mut app.mode)
                    .desired_width(150.0)
                    .frame(false)
                    .interactive(false),
            );
        });
        ui.horizontal(|ui| {
            ui.label("Debug:");
            ui.add(
                TextEdit::singleline(&mut app.debug_1)
                    .desired_width(100.0)
                    .frame(false)
                    .interactive(false),
            );
            ui.add(
                TextEdit::singleline(&mut app.debug_2)
                    .desired_width(100.0)
                    .frame(false)
                    .interactive(false),
            );
            if ui.button("DL via BV Code").clicked() {}
        });

        ui.horizontal(|ui| {
            ui.label("rA");
            ui.add(
                TextEdit::singleline(&mut app.ra)
                    .desired_width(100.0)
                    .frame(false)
                    .interactive(false),
            );
            ui.label("r0");
            ui.add(
                TextEdit::singleline(&mut app.r0)
                    .desired_width(100.0)
                    .frame(false)
                    .interactive(false),
            );
            if ui.button("(Debug) Reload").clicked() {}
        });
        ui.horizontal(|ui| {
            ui.label("rM");
            ui.add(
                TextEdit::singleline(&mut app.rm)
                    .desired_width(150.0)
                    .frame(false)
                    .interactive(false),
            );
            ui.label("rN");
            ui.add(
                TextEdit::singleline(&mut app.rn)
                    .desired_width(150.0)
                    .frame(false)
                    .interactive(false),
            );
        });
        ui.add_space(40.0);
        ScrollArea::vertical()
            .min_scrolled_height(200.0)
            .max_height(200.0)
            .max_width(400.0)
            .id_source("instruction text")
            .show(ui, |ui| {
                ui.add(
                    TextEdit::multiline(&mut app.instruction_text)
                        .desired_width(400.0)
                        .desired_rows(10)
                        .interactive(false)
                        .frame(false),
                );
            });
    });
}
