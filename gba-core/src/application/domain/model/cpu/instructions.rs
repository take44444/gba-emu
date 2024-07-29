use serde::{Deserialize, Serialize};

use crate::application::domain::model::{
  bus::Bus,
  cpu::registers::Registers,
  peripherals::Peripherals,
};

#[derive(Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum R15Status {
  Changed,
  NotChanged,
}

#[derive(Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Opcode {
  Arm(u32),
  Thumb(u16),
}

#[derive(Clone, Serialize, Deserialize, Default)]
pub struct FetchingInstruction {
  pub addr: u32,
  pub opcode: Option<Opcode>,
}

impl FetchingInstruction {
  pub fn new(addr: u32) -> Self {
    Self {
      addr,
      opcode: None,
    }
  }
  pub fn dummy() -> Self {
    Self {
      addr: 0,
      opcode: Some(Opcode::Arm(0)),
    }
  }
  pub fn is_fetched(&self) -> bool {
    self.opcode.is_some()
  }
  pub fn fetch(&mut self, bus: &Bus, peripherals: &Peripherals, is_thumb_mode: bool) {
    assert!(!self.is_fetched());
    if is_thumb_mode {
      self.opcode = bus.read16(self.addr, peripherals)
                       .map(|opcode| Opcode::Thumb(opcode));
    } else {
      self.opcode = bus.read32(self.addr, peripherals)
                       .map(|opcode| Opcode::Arm(opcode));
    }
  }
}

#[derive(Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DecodedInstruction {
  Dummy,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct DecodingInstruction {
  pub addr: u32,
  opcode: Opcode,
  pub inst: Option<DecodedInstruction>,
}

impl DecodingInstruction {
  pub fn new(addr: u32, opcode: Opcode) -> Self {
    Self {
      addr,
      opcode,
      inst: None,
    }
  }
  pub fn dummy() -> Self {
    Self {
      addr: 0,
      opcode: Opcode::Arm(0),
      inst: Some(DecodedInstruction::Dummy),
    }
  }
  pub fn is_decoded(&self) -> bool {
    self.inst.is_some()
  }
  pub fn decode(&mut self) {
    // TODO
    self.inst = Some(DecodedInstruction::Dummy);
  }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ExecutingInstruction {
  inst: DecodedInstruction,
  addr: u32,
  step: usize,
  pub r15_status: Option<R15Status>,
}

impl ExecutingInstruction {
  pub fn new(addr: u32, inst: DecodedInstruction) -> Self {
    Self {
      inst,
      addr,
      step: 0,
      r15_status: None,
    }
  }
  pub fn dummy() -> Self {
    Self {
      inst: DecodedInstruction::Dummy,
      addr: 0,
      step: 0,
      r15_status: Some(R15Status::NotChanged),
    }
  }
  pub fn is_executed(&self) -> bool {
    self.r15_status.is_some()
  }
  pub fn execute(&mut self, regs: &mut Registers, bus: &Bus, peripherals: &Peripherals) {
    // TODO
    self.r15_status = Some(R15Status::NotChanged);
  }
}