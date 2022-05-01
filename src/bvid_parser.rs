use crate::battle_style::BattleStyle;
use crate::bv6::Bv6;
use crate::move_target::MoveTarget;
use crate::pk6::Pk6;
use crate::rotate_direction::RotateDirection;
use crate::turn_action_code::TurnActionCode;
use crate::util::{MOVES, MOVE_DATA};
use no_std_io::{Cursor, StreamContainer, StreamReader};
use std::fmt::{Display, Formatter};
use std::io::BufReader;

pub struct BVidParser {
    turn: u32,
    bvid: Bv6,
    teams: Vec<Vec<Pk6>>,
    trainers: Vec<String>,
    is_out: [bool; 4],
}

impl BVidParser {
    pub fn new(bvid: Bv6, teams: Vec<Vec<Pk6>>, trainers: Vec<String>) -> Self {
        Self {
            bvid,
            teams,
            trainers,
            turn: 0,
            is_out: [false, false, false, false],
        }
    }

    pub fn parse(&mut self, instructions: Vec<u8>) -> Vec<String> {
        self.turn = 0;
        let mut lines = vec![];
        self.add_lines(instructions, &mut lines);
        lines
    }

    fn add_lines(&mut self, data: Vec<u8>, lines: &mut Vec<String>) {
        let len = data.len();
        let mut reader = StreamContainer::new(data);
        let order = reader.read_stream::<u8>().unwrap();
        lines.push(format!("Start Battle: [0x{:0>2X}]", order));

        let style: BattleStyle = self.bvid.style().into();
        match style {
            BattleStyle::Single => {
                for (i, trainer) in self.trainers.iter().enumerate().take(2) {
                    lines.push(format!("{} leads {}", trainer, self.teams[i][0].nickname()));
                }
            }
            BattleStyle::Double => {
                for (i, trainer) in self.trainers.iter().enumerate().take(2) {
                    lines.push(format!(
                        "{} leads {} and {}",
                        trainer,
                        self.teams[i][0].nickname(),
                        self.teams[i][1].nickname()
                    ));
                }
            }
            BattleStyle::Triple => {
                for (i, trainer) in self.trainers.iter().enumerate().take(2) {
                    lines.push(format!(
                        "{} leads {}, {} and {}",
                        trainer,
                        self.teams[i][0].nickname(),
                        self.teams[i][1].nickname(),
                        self.teams[i][2].nickname()
                    ));
                }
            }
            _ => {
                for (i, trainer) in self.trainers.iter().enumerate() {
                    lines.push(format!("{} leads {}", trainer, self.teams[i][0].nickname()));
                }
            }
        }
        while reader.get_index() < len {
            let mut actions = self.perform_op(&mut reader);
            lines.append(&mut actions);
        }
    }

    fn perform_op(&mut self, reader: &mut StreamContainer<Vec<u8>>) -> Vec<String> {
        let op = (*reader).read_stream::<u8>().unwrap();
        let op_code = op >> 4;
        let op_len = op & 0xF;

        let mut lines = vec![];

        match op_code {
            1 => lines.push("Switching Required...".to_string()),
            9 => {
                lines.push("".to_string());
                self.turn += 1;
                lines.push(format!("Start Turn {}", self.turn));
            }
            _ => {}
        }

        for _ in 0..op_len {
            let op_2 = (*reader).read_stream::<u8>().unwrap();
            let player = op_2 >> 5;
            let op_len_2 = op_2 & 0xF;

            match op_code {
                1 => {
                    lines.push(self.perform_switch(
                        player,
                        (*reader).read_byte_stream(4 * op_len_2 as usize).unwrap(),
                    ));
                }
                9 => {
                    lines.push(self.perform_action(
                        player,
                        (*reader).read_byte_stream(4 * op_len_2 as usize).unwrap(),
                    ));
                }
                _ => {
                    lines.push(format!("Unknown Instruction Type: {}", op_code));
                }
            }
        }
        lines
    }

    fn perform_switch(&mut self, player: u8, data: Vec<u8>) -> String {
        let mut parse = format!("[{}]: ", self.trainers[player as usize]);
        for i in 0..(data.len() / 4) {
            let instruct = data.iter().skip(i * 4).take(4).collect::<Vec<_>>();
            if instruct.iter().all(|&&i| i == 0) {
                parse = format!("{}None", parse);
                continue;
            }
            let val = u16::from_le_bytes((&data[(i * 4)..(i * 4 + 2)]).try_into().unwrap());
            let op = val & 0x7F;
            let op_code = op & 0xF;
            let slot_in = (val >> 7) & 0x7;
            let slot_out = (val >> 4) & 0x7;
            let nothing = (val >> 10) == 1;
            let targeting = val >> 4;

            match op_code {
                3 => {
                    if self.is_out[player as usize] {
                        parse = format!("{}None**", parse);
                    } else if nothing {
                        parse = format!(
                            "{}Out: {}, In: Nothing",
                            parse,
                            self.teams[player as usize][slot_out as usize].nickname()
                        );
                    } else {
                        parse = format!(
                            "{}Out: {}, In: {}",
                            parse,
                            self.teams[player as usize][slot_out as usize].nickname(),
                            self.teams[player as usize][slot_in as usize].nickname()
                        );
                        let t1 = self.teams[player as usize][slot_out as usize].clone();
                        self.teams[player as usize][slot_out as usize] =
                            self.teams[player as usize][slot_in as usize].clone();
                        self.teams[player as usize][slot_in as usize] = t1;
                    }
                }
                6 => {
                    let cur_active = self.teams[player as usize][0].nickname();
                    for _ in 0..targeting {
                        let t0 = self.teams[player as usize][0].clone();
                        let t1 = self.teams[player as usize][1].clone();
                        let t2 = self.teams[player as usize][2].clone();

                        self.teams[player as usize][0] = t1;
                        self.teams[player as usize][1] = t2;
                        self.teams[player as usize][2] = t0;
                    }
                    let now_active = self.teams[player as usize][0].nickname();
                    parse = format!("{}Rotates Out: {}, In: {}", parse, cur_active, now_active);
                }
                _ => {
                    for byte in data.iter() {
                        parse = format!("{}{:0>2X} ", parse, byte);
                    }
                }
            }

            if (i + 1) * 4 != data.len() {
                parse = format!("{} & ", parse);
            }
        }
        parse
    }

    fn perform_action(&mut self, player: u8, data: Vec<u8>) -> String {
        let mut parse = format!("[{}]: ", self.trainers[player as usize]);
        let mut user = 0;
        for i in 0..(data.len() / 4) {
            let val = data[i * 4];
            let op_code = val & 0xF;
            let targeting = val >> 4;
            let mut target = MoveTarget::from(targeting).to_string();
            let action = TurnActionCode::from(op_code);
            match action {
                TurnActionCode::None => {
                    self.is_out[player as usize] = true;
                    parse = format!("{}Nothing", parse);
                }
                TurnActionCode::Fight => {
                    let move_val = u16::from_le_bytes(
                        (&data[(1 + (4 * i))..((1 + (4 * i)) + 2)])
                            .try_into()
                            .unwrap(),
                    );
                    let target_val = MOVE_DATA[move_val as usize][0x14];
                    if self.trainers[player as usize] == "NPC".to_string() && target_val == 0 {
                        target = "Opposite Enemy".to_string();
                    }
                    parse = format!(
                        "{}{}: {} uses {} @ {}{}",
                        parse,
                        action,
                        self.teams[player as usize][user].nickname(),
                        MOVES[move_val as usize],
                        target,
                        {
                            if data[3 + (4 * i)] > 0 {
                                format!(" - {}", data[3 + (4 * 1)])
                            } else {
                                "".to_string()
                            }
                        }
                    );
                    user += 1;
                }
                TurnActionCode::Switch => {
                    let val2 =
                        u16::from_le_bytes((&data[(4 * i)..((4 * i) + 2)]).try_into().unwrap());
                    let slot_in = (val2 >> 7) & 0x7;
                    let slot_out = val2 >> 10;
                    parse = format!(
                        "{}Out: {}, In: {}",
                        parse,
                        self.teams[player as usize][slot_out as usize].nickname(),
                        self.teams[player as usize][slot_in as usize].nickname()
                    );
                    let t1 = self.teams[player as usize][slot_out as usize].clone();
                    self.teams[player as usize][slot_out as usize] =
                        self.teams[player as usize][slot_in as usize].clone();
                    self.teams[player as usize][slot_in as usize] = t1;
                }
                TurnActionCode::Rotate => {
                    let rot = RotateDirection::from(targeting).to_string();
                    let cur_in = self.teams[player as usize][user].nickname();
                    for _ in 0..targeting {
                        let t0 = self.teams[player as usize][0].clone();
                        let t1 = self.teams[player as usize][1].clone();
                        let t2 = self.teams[player as usize][2].clone();

                        self.teams[player as usize][0] = t1;
                        self.teams[player as usize][1] = t2;
                        self.teams[player as usize][2] = t0;
                    }
                    parse = format!(
                        "{}{}: {} rotates {} to {}",
                        parse,
                        action,
                        cur_in,
                        rot,
                        self.teams[player as usize][user].nickname()
                    );
                    user += 1;
                }
                _ => {
                    let arg = format!(
                        "{:0>4X}{:0>2X}",
                        u16::from_le_bytes(
                            (&data[(1 + (4 * i))..((1 + (4 * i)) + 2)])
                                .try_into()
                                .unwrap()
                        ),
                        data[3 + (4 * 1)]
                    );
                    parse = format!(
                        "{}{}: {}{}",
                        parse,
                        action,
                        target,
                        if arg == "000000".to_string() {
                            "".to_string()
                        } else {
                            format!(" - {}", arg)
                        }
                    );
                }
            }
            if (i + 1) * 4 != data.len() {
                parse = format!("{} & ", parse);
            }
        }
        parse
    }
}
