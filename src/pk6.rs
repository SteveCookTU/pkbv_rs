use crate::util;
use std::fmt::{Display, Formatter};
use std::ops::Deref;
use pkhex_rs::game_strings::{ABILITIES_EN, ITEMS_EN, MOVES_EN, NATURES_EN, SPECIES_EN};
use pkhex_rs::{Pkm, SpeciesForm};

#[derive(Clone)]
pub struct Pk6(pub pkhex_rs::PK6);

impl Display for Pk6 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} [{}] lv{} @ {} -- {}\n{} / {} / {} / {}\nIVs: {:0>2}/{:0>2}/{:0>2}/{:0>2}/{:0>2}/{:0>2}\
         || EVs: {:0>2}/{:0>2}/{:0>2}/{:0>2}/{:0>2}/{:0>2}",
               SPECIES_EN[self.0.get_species() as usize].trim(),
               ABILITIES_EN[self.0.get_ability() as usize].trim(), self.0.get_current_level(),
               ITEMS_EN[self.0.get_held_item() as usize].trim(),
               NATURES_EN[self.0.get_stat_nature() as usize].trim(),
               MOVES_EN[self.0.get_move_1() as usize].trim(),
               MOVES_EN[self.0.get_move_2() as usize].trim(),
               MOVES_EN[self.0.get_move_3() as usize].trim(),
               MOVES_EN[self.0.get_move_4() as usize].trim(),
               self.0.get_iv_hp(),
               self.0.get_iv_atk(),
               self.0.get_iv_def(),
               self.0.get_iv_spa(),
               self.0.get_iv_spd(),
               self.0.get_iv_spe(),
               self.0.get_ev_hp(),
               self.0.get_ev_atk(),
               self.0.get_ev_def(),
               self.0.get_ev_spa(),
               self.0.get_ev_spd(),
               self.0.get_ev_spe())
    }
}
