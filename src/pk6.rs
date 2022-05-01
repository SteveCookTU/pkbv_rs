use crate::personal_table::PersonalInfo;
use crate::util;
use crate::util::{ABILITIES, EXP_TABLE, ITEMS, MOVES, NATURES, PERSONAL_TABLE, SPECIES};
use std::fmt::{Display, Formatter};
use std::ops::Deref;

const BLOCK_POSITION: [u8; 128] = [
    0, 1, 2, 3, 0, 1, 3, 2, 0, 2, 1, 3, 0, 3, 1, 2, 0, 2, 3, 1, 0, 3, 2, 1, 1, 0, 2, 3, 1, 0, 3, 2,
    2, 0, 1, 3, 3, 0, 1, 2, 2, 0, 3, 1, 3, 0, 2, 1, 1, 2, 0, 3, 1, 3, 0, 2, 2, 1, 0, 3, 3, 1, 0, 2,
    2, 3, 0, 1, 3, 2, 0, 1, 1, 2, 3, 0, 1, 3, 2, 0, 2, 1, 3, 0, 3, 1, 2, 0, 2, 3, 1, 0, 3, 2, 1, 0,
    // duplicates of 0-7 to eliminate modulus
    0, 1, 2, 3, 0, 1, 3, 2, 0, 2, 1, 3, 0, 3, 1, 2, 0, 2, 3, 1, 0, 3, 2, 1, 1, 0, 2, 3, 1, 0, 3, 2,
];

#[derive(Clone)]
pub struct Pk6 {
    data: Vec<u8>,
}

impl Display for Pk6 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} [{}] lv{} @ {} -- {}\n{} / {} / {} / {}\nIVs: {:0>2}/{:0>2}/{:0>2}/{:0>2}/{:0>2}/{:0>2}\
         || EVs: {:0>2}/{:0>2}/{:0>2}/{:0>2}/{:0>2}/{:0>2}",
               SPECIES[self.species() as usize].trim(),
               ABILITIES[self.ability() as usize].trim(), self.level(),
               ITEMS[self.held_item() as usize].trim(),
               NATURES[self.stat_nature() as usize].trim(),
               MOVES[self.move_1() as usize].trim(),
               MOVES[self.move_2() as usize].trim(),
               MOVES[self.move_3() as usize].trim(),
               MOVES[self.move_4() as usize].trim(),
               self.iv_hp(),
               self.iv_atk(),
               self.iv_def(),
               self.iv_spa(),
               self.iv_spd(),
               self.iv_spe(),
               self.ev_hp(),
               self.ev_atk(),
               self.ev_def(),
               self.ev_spa(),
               self.ev_spd(),
               self.ev_spe())
    }
}

impl Pk6 {
    pub fn new(data: Vec<u8>) -> Self {
        let mut pkm = Self { data };
        pkm.decrypt_if_encrypted();
        pkm
    }

    fn decrypt(&mut self) {
        let seed = u32::from_le_bytes((&self.data[0x00..0x04]).try_into().unwrap());
        let sv = seed >> 13 & 0x1F;
        self.crypt_pkm(seed as usize);
        self.shuffle(sv as usize);
    }

    fn crypt_pkm(&mut self, seed: usize) {
        self.crypt(seed, 8, 0xE8);
        if self.data.len() == 0x104 {
            self.crypt(seed, 0xE8, 0x104);
        }
    }

    fn crypt(&mut self, mut seed: usize, start: usize, end: usize) {
        let mut i = start;
        while i < end {
            seed = seed.wrapping_mul(0x41C64E6D).wrapping_add(0x00006073);
            self.data[i] ^= (seed >> 16) as u8;
            i += 1;
            self.data[i] ^= (seed >> 24) as u8;
            i += 1;
        }
    }

    fn shuffle(&mut self, sv: usize) {
        let idx = 4 * sv;
        let sdata = self.data.clone();
        for block in 0..4 {
            let ofs = BLOCK_POSITION[idx + block] as usize;
            self.data.splice(
                (8 + 56 * block)..(8 + 56 * (block + 1)),
                sdata[(8 + 56 * ofs)..(8 + 56 * (ofs + 1))].iter().cloned(),
            );
        }
    }

    fn decrypt_if_encrypted(&mut self) {
        if u16::from_le_bytes((&self.data[0xC8..0xCA]).try_into().unwrap()) != 0
            || u16::from_le_bytes((&self.data[0x58..0x5A]).try_into().unwrap()) != 0
        {
            self.decrypt();
        }
    }

    pub fn species(&self) -> u16 {
        u16::from_le_bytes((&self.data[0x08..0xA]).try_into().unwrap())
    }

    pub fn form(&self) -> u8 {
        self.data[0x1D] >> 3
    }

    pub fn nickname_trash(&self) -> &[u8] {
        &self.data[0x40..0x5A]
    }

    pub fn nickname(&self) -> String {
        util::get_string(self.nickname_trash().to_vec())
    }

    pub fn held_item(&self) -> u16 {
        u16::from_le_bytes((&self.data[0x0A..0x0C]).try_into().unwrap())
    }

    pub fn ability(&self) -> u8 {
        self.data[0x14]
    }

    pub fn ev_hp(&self) -> u8 {
        self.data[0x1E]
    }

    pub fn ev_atk(&self) -> u8 {
        self.data[0x1F]
    }

    pub fn ev_def(&self) -> u8 {
        self.data[0x20]
    }

    pub fn ev_spe(&self) -> u8 {
        self.data[0x21]
    }

    pub fn ev_spa(&self) -> u8 {
        self.data[0x22]
    }

    pub fn ev_spd(&self) -> u8 {
        self.data[0x23]
    }

    pub fn evs(&self) -> [u8; 6] {
        [
            self.ev_hp(),
            self.ev_atk(),
            self.ev_def(),
            self.ev_spe(),
            self.ev_spa(),
            self.ev_spd(),
        ]
    }

    fn iv32(&self) -> u32 {
        u32::from_le_bytes((&self.data[0x74..0x78]).try_into().unwrap())
    }

    pub fn iv_hp(&self) -> u8 {
        (self.iv32() & 0x1F) as u8
    }

    pub fn iv_atk(&self) -> u8 {
        ((self.iv32() >> 5) & 0x1F) as u8
    }

    pub fn iv_def(&self) -> u8 {
        ((self.iv32() >> 10) & 0x1F) as u8
    }

    pub fn iv_spe(&self) -> u8 {
        ((self.iv32() >> 15) & 0x1F) as u8
    }

    pub fn iv_spa(&self) -> u8 {
        ((self.iv32() >> 20) & 0x1F) as u8
    }

    pub fn iv_spd(&self) -> u8 {
        ((self.iv32() >> 25) & 0x1F) as u8
    }

    pub fn ivs(&self) -> [u8; 6] {
        [
            self.iv_hp(),
            self.iv_atk(),
            self.iv_def(),
            self.iv_spe(),
            self.iv_spa(),
            self.iv_spd(),
        ]
    }

    pub fn move_1(&self) -> u16 {
        u16::from_le_bytes((&self.data[0x5A..0x5C]).try_into().unwrap())
    }

    pub fn move_2(&self) -> u16 {
        u16::from_le_bytes((&self.data[0x5C..0x5E]).try_into().unwrap())
    }

    pub fn move_3(&self) -> u16 {
        u16::from_le_bytes((&self.data[0x5E..0x60]).try_into().unwrap())
    }

    pub fn move_4(&self) -> u16 {
        u16::from_le_bytes((&self.data[0x60..0x62]).try_into().unwrap())
    }

    pub fn moves(&self) -> [u16; 4] {
        [self.move_1(), self.move_2(), self.move_3(), self.move_4()]
    }

    pub fn stat_nature(&self) -> u8 {
        self.data[0x1C]
    }

    pub fn gender(&self) -> u8 {
        (self.data[0x1D] >> 1) & 0x3
    }

    pub fn current_handler(&self) -> u8 {
        self.data[0x92]
    }

    pub fn ot_friendship(&self) -> u8 {
        self.data[0xCA]
    }

    pub fn ht_friendship(&self) -> u8 {
        self.data[0xA2]
    }

    pub fn current_friendship(&self) -> u8 {
        if self.current_handler() == 0 {
            self.ot_friendship()
        } else {
            self.ht_friendship()
        }
    }

    pub fn exp(&self) -> u32 {
        u32::from_le_bytes((&self.data[0x10..0x14]).try_into().unwrap())
    }

    pub fn level(&self) -> u8 {
        let growth = self.personal_info().exp_growth() as usize;
        if self.exp() > EXP_TABLE[99][growth] {
            return 100;
        }
        let mut t1 = 1;
        while self.exp() >= EXP_TABLE[t1][growth] {
            t1 += 1;
        }
        t1 as u8
    }

    pub fn pid(&self) -> u32 {
        u32::from_le_bytes((&self.data[0x18..0x1C]).try_into().unwrap())
    }

    pub fn tid(&self) -> u16 {
        u16::from_le_bytes((&self.data[0x0C..0x0E]).try_into().unwrap())
    }

    pub fn sid(&self) -> u16 {
        u16::from_le_bytes((&self.data[0x0E..0x10]).try_into().unwrap())
    }

    pub fn tsv(&self) -> u16 {
        (self.tid() ^ self.sid()) >> 4
    }

    pub fn psv(&self) -> u16 {
        (((self.pid() >> 16) as u16) ^ (self.pid() as u16)) >> 4
    }

    pub fn is_shiny(&self) -> bool {
        self.tsv() == self.psv()
    }

    pub fn personal_info(&self) -> &PersonalInfo {
        PERSONAL_TABLE.get_form_entry(self.species() as usize, self.form() as usize)
    }
}
