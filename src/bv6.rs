use crate::pk6::Pk6;
use crate::util;
use std::fmt::{Display, Formatter};
use time::{Date, Month, PrimitiveDateTime, Time};

#[derive(Clone)]
pub struct Bv6 {
    data: Vec<u8>,
}

impl Bv6 {
    const PLAYER_COUNT: usize = 4;
    const NPC: &'static str = "NPC";

    pub fn new(data: Vec<u8>) -> Self {
        Self { data }
    }

    pub fn mode(&self) -> u8 {
        self.data[0x00]
    }

    pub fn style(&self) -> u8 {
        self.data[0x01]
    }

    pub fn battle_pkms(&self) -> Vec<Pk6> {
        self.player_teams()
            .into_iter()
            .flatten()
            .collect::<Vec<Pk6>>()
    }

    pub fn debug_1(&self) -> String {
        util::get_string(self.data[0x6..0x20].to_vec())
    }

    pub fn debug_2(&self) -> String {
        util::get_string(self.data[0x50..0x6A].to_vec())
    }

    pub fn rng_const_1(&self) -> u32 {
        u32::from_le_bytes((&self.data[0x1A0..0x1A4]).try_into().unwrap())
    }

    pub fn rng_const_2(&self) -> u32 {
        u32::from_le_bytes((&self.data[0x1A4..0x1A8]).try_into().unwrap())
    }

    pub fn rng_seed_1(&self) -> u64 {
        u64::from_le_bytes((&self.data[0x1A8..0x1B0]).try_into().unwrap())
    }

    pub fn rng_seed_2(&self) -> u64 {
        u64::from_le_bytes((&self.data[0x1B0..0x1B8]).try_into().unwrap())
    }

    pub fn background(&self) -> u32 {
        u32::from_le_bytes((&self.data[0x1BC..0x1C0]).try_into().unwrap())
    }

    pub fn unk_1_ce(&self) -> u16 {
        u16::from_le_bytes((&self.data[0x1CE..0x1D0]).try_into().unwrap())
    }

    pub fn intro_id(&self) -> u16 {
        u16::from_le_bytes((&self.data[0x1E4..0x1E6]).try_into().unwrap())
    }

    pub fn music_id(&self) -> u16 {
        u16::from_le_bytes((&self.data[0x1F0..0x1F2]).try_into().unwrap())
    }

    pub fn player_names(&self) -> Vec<String> {
        let mut trainers = Vec::with_capacity(Bv6::PLAYER_COUNT);
        for i in 0..Bv6::PLAYER_COUNT {
            let trainer_name =
                util::get_string(self.data[0xEC + (0x1A * i)..(0xEC + (0x1A * i) + 0x1A)].to_vec());
            if trainer_name.trim().is_empty() {
                trainers.push(Bv6::NPC.to_string())
            } else {
                trainers.push(trainer_name);
            }
        }
        trainers
    }

    pub fn player_teams(&self) -> Vec<Vec<Pk6>> {
        let mut teams = Vec::with_capacity(Bv6::PLAYER_COUNT);
        for i in 0..Bv6::PLAYER_COUNT {
            teams.push(self.get_team(i));
        }
        teams
    }

    pub fn get_team(&self, t: usize) -> Vec<Pk6> {
        let mut team = Vec::with_capacity(6);
        let start: usize = 0xE18;
        for p in 0..6 {
            let mut offset = start + (0x104 * ((t * 6) + p));
            offset += 8 * (((t * 6) + p) / 6);
            team.push(Pk6::new(self.data[offset..(offset + 0x104)].to_vec()));
        }
        team
    }

    pub fn match_year(&self) -> u16 {
        u16::from_le_bytes((&self.data[0x2E50..0x2E52]).try_into().unwrap())
    }

    pub fn match_day(&self) -> u8 {
        self.data[0x2E52]
    }

    pub fn match_month(&self) -> u8 {
        self.data[0x2E53]
    }

    pub fn match_hour(&self) -> u8 {
        self.data[0x2E54]
    }

    pub fn match_min(&self) -> u8 {
        self.data[0x2E55]
    }

    pub fn match_second(&self) -> u8 {
        self.data[0x2E56]
    }

    pub fn match_flags(&self) -> u8 {
        self.data[0x2E57]
    }

    pub fn upload_year(&self) -> u16 {
        u16::from_le_bytes((&self.data[0x2E58..0x2E5A]).try_into().unwrap())
    }

    pub fn upload_day(&self) -> u8 {
        self.data[0x2E5A]
    }

    pub fn upload_month(&self) -> u8 {
        self.data[0x2E5B]
    }

    pub fn upload_hour(&self) -> u8 {
        self.data[0x2E5C]
    }

    pub fn upload_min(&self) -> u8 {
        self.data[0x2E5D]
    }

    pub fn upload_second(&self) -> u8 {
        self.data[0x2E5E]
    }

    pub fn upload_flags(&self) -> u8 {
        self.data[0x2E5F]
    }

    pub fn match_stamp(&self) -> Option<PrimitiveDateTime> {
        if let Ok(month) = Month::try_from(self.match_month()) {
            if let Ok(date) =
                Date::from_calendar_date(self.match_year() as i32, month, self.match_day())
            {
                if let Ok(time) =
                    Time::from_hms(self.match_hour(), self.match_min(), self.match_second())
                {
                    return Some(PrimitiveDateTime::new(date, time));
                }
            }
        }

        None
    }

    pub fn upload_stamp(&self) -> Option<PrimitiveDateTime> {
        if let Ok(month) = Month::try_from(self.upload_month()) {
            if let Ok(date) =
                Date::from_calendar_date(self.upload_year() as i32, month, self.upload_day())
            {
                if let Ok(time) =
                    Time::from_hms(self.upload_hour(), self.upload_min(), self.upload_second())
                {
                    return Some(PrimitiveDateTime::new(date, time));
                }
            }
        }

        None
    }
}
