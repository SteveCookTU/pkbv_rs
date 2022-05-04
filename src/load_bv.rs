use crate::battle_mode::BattleMode;
use crate::battle_style::BattleStyle;
use crate::bv6::Bv6;
use crate::bvid_parser::BVidParser;
use crate::my_app::MyApp;
use crate::pk6::Pk6;
use crate::util;

use egui_extras::RetainedImage;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use pkhex_rs::SpeciesForm;

pub fn load_bv(app: &mut MyApp, path: PathBuf) {
    let mut buf = vec![];
    let mut file = File::open(path).unwrap();
    file.read_to_end(&mut buf).unwrap();
    if buf.len() != 0x2E60 {
        return;
    }
    app.reset();
    let bvid = Bv6::new(buf.clone());

    let mut trainers = bvid.player_names();
    let pkms = bvid.battle_pkms();
    let mut teams = bvid.player_teams();

    load_battle_info(app, &bvid, &trainers);
    load_team_data(app, &pkms, &trainers);
    load_preview_pics(app, &pkms);
    let order = buf[0x0];
    let instruction_length = i32::from_le_bytes((&buf[0x210..0x214]).try_into().unwrap());
    let instructions = buf
        .clone()
        .into_iter()
        .skip(0x214)
        .take(instruction_length as usize)
        .collect::<Vec<u8>>();

    if order != 0x20 {
        util::reorder_teams(order, bvid.style(), &mut teams, &mut trainers);
    }

    let mut parser = BVidParser::new(bvid, teams, trainers);
    app.instruction_text = parser.parse(instructions).join("\n");
}

fn load_preview_pics(app: &mut MyApp, pkms: &Vec<Pk6>) {
    for i in 0..12 {
        if pkms[i].0.get_species() == 0 || pkms[i].0.get_species() > 722 {
            continue;
        }
        let mut buf = vec![];
        let mut file = File::open(format!("sprites/{}.png", pkms[i].0.get_species())).unwrap();
        file.read_to_end(&mut buf).unwrap();
        app.pkmn_images[i] =
            Some(RetainedImage::from_image_bytes(format!("pkm_{}", i), &buf).unwrap())
    }
}

fn load_team_data(app: &mut MyApp, pkms: &Vec<Pk6>, trainers: &Vec<String>) {
    let mut team = 0;
    for (i, pkm) in pkms.iter().enumerate() {
        if pkm.0.get_species() == 0 || pkm.0.get_species() >= 722 {
            continue;
        }
        if i % 6 == 0 {
            team += 1;
            app.summary_text = format!(
                "{}========\n{}'s Party - {}\n========\n",
                app.summary_text,
                trainers[team - 1],
                team
            );
        }
        app.summary_text = format!("{}{}\n\n", app.summary_text, pkm);
    }
}

fn load_battle_info(app: &mut MyApp, bvid: &Bv6, trainers: &Vec<String>) {
    app.mode = BattleMode::from(bvid.mode()).to_string();
    app.style = BattleStyle::from(bvid.style()).to_string();
    app.versus_text = if bvid.style() == 4 {
        format!(
            "{} && {} -VS- {} && {}",
            trainers[0], trainers[1], trainers[2], trainers[3]
        )
    } else {
        format!("{} -VS- {}", trainers[0], trainers[1])
    };

    app.ra = format!("{:0<8X}", bvid.rng_const_1());
    app.r0 = format!("{:0<8X}", bvid.rng_const_2());
    app.rm = format!("{:0<16X}", bvid.rng_seed_1());
    app.rn = format!("{:0<16X}", bvid.rng_seed_2());
    app.debug_1 = bvid.debug_1();
    app.debug_2 = bvid.debug_2();
    app.recorded_at = if let Some(stamp) = bvid.match_stamp() {
        stamp.to_string()
    } else {
        "None".to_string()
    };
    app.uploaded_at = if let Some(stamp) = bvid.upload_stamp() {
        stamp.to_string()
    } else {
        "None".to_string()
    };
}
