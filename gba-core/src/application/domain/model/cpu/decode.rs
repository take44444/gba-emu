use bitutils::bits;
use serde::{Deserialize, Serialize};

use crate::application::domain::model::cpu::{
  pipeline::{
    DecodedArmInstruction,
    DecodedInstruction,
    DecodedThumbInstruction,
    DecodingInstruction,
    Opcode,
  },
  registers::Cpsr,
};

#[repr(u32)]
#[derive(Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Cond {
  EQ = 0x0, NE = 0x1, CS = 0x2, CC = 0x3, MI = 0x4, PL = 0x5, VS = 0x6, VC = 0x7,
  HI = 0x8, LS = 0x9, GE = 0xA, LT = 0xB, GT = 0xC, LE = 0xD, AL = 0xE, NV = 0xF,
}

impl From<u32> for Cond {
  fn from(value: u32) -> Self {
    match value {
      0x0 => Cond::EQ,
      0x1 => Cond::NE,
      0x2 => Cond::CS,
      0x3 => Cond::CC,
      0x4 => Cond::MI,
      0x5 => Cond::PL,
      0x6 => Cond::VS,
      0x7 => Cond::VC,
      0x8 => Cond::HI,
      0x9 => Cond::LS,
      0xA => Cond::GE,
      0xB => Cond::LT,
      0xC => Cond::GT,
      0xD => Cond::LE,
      0xE => Cond::AL,
      0xF => Cond::NV,
      _ => panic!(),
    }
  }
}

impl Cond {
  pub fn check(&self, cpsr: Cpsr) -> bool {
    match self {
      Cond::EQ => cpsr.z(),
      Cond::NE => !cpsr.z(),
      Cond::CS => cpsr.c(),
      Cond::CC => !cpsr.c(),
      Cond::MI => cpsr.n(),
      Cond::PL => !cpsr.n(),
      Cond::VS => cpsr.v(),
      Cond::VC => !cpsr.v(),
      Cond::HI => cpsr.c() && !cpsr.z(),
      Cond::LS => !cpsr.c() && cpsr.z(),
      Cond::GE => cpsr.n() == cpsr.v(),
      Cond::LT => cpsr.n() != cpsr.v(),
      Cond::GT => !cpsr.z() && cpsr.n() == cpsr.v(),
      Cond::LE => cpsr.z() && cpsr.n() != cpsr.v(),
      Cond::AL => true,
      Cond::NV => false,
    }
  }
}

impl DecodingInstruction {
  pub fn decode(&mut self) {
    self.inst = match self.opcode {
      Opcode::Arm(opcode) => {
        let cond: Cond = (opcode >> 28).into();
        let mut inst: Option<DecodedArmInstruction> = None;
        inst = inst.or(decode_arm_b(opcode));
        inst.map(|i| DecodedInstruction::Arm((cond, i)))
      },
      Opcode::Thumb(opcode) => {
        let mut inst: Option<DecodedThumbInstruction> = None;
        inst = inst.or(decode_thumb_b(opcode));
        inst.map(|i| DecodedInstruction::Thumb(i))
      },
    };
    assert!(self.is_decoded());
  }
}

fn decode_arm_b(opcode: u32) -> Option<DecodedArmInstruction> {
  if bits!(opcode, 24:27) == 0b1010 {
    return Some(DecodedArmInstruction::B(((opcode as i32) << 8) >> 6));
  }
  None
}

fn decode_thumb_b(opcode: u16) -> Option<DecodedThumbInstruction> {
  if bits!(opcode, 11:15) == 0b11100 {
    return Some(DecodedThumbInstruction::B(((opcode as i32) << 21) >> 20));
  }
  None
}