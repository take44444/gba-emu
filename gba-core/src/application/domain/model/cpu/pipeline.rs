use serde::{Deserialize, Serialize};

use crate::application::domain::model::{
  bus::Bus,
  cpu::{
    Cpu,
    decode::Cond,
    registers::{
      Cpsr,
      Registers,
    },
  },
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
  pub fn fetch(&mut self, bus: &mut Bus, peripherals: &Peripherals, is_thumb_mode: bool) -> bool {
    assert!(!self.is_fetched());
    if is_thumb_mode {
      self.opcode = bus.read16(self.addr, peripherals)
                       .map(|opcode| Opcode::Thumb(opcode));
    } else {
      self.opcode = bus.read32(self.addr, peripherals)
                       .map(|opcode| Opcode::Arm(opcode));
    }
    self.is_fetched()
  }
}

#[derive(Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DecodedArmInstruction {
  Dummy,
  B(i32),
}

#[derive(Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DecodedThumbInstruction {
  B(i32),
}

#[derive(Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DecodedInstruction {
  Arm((Cond, DecodedArmInstruction)),
  Thumb(DecodedThumbInstruction),
}

#[derive(Clone, Serialize, Deserialize)]
pub struct DecodingInstruction {
  pub addr: u32,
  pub opcode: Opcode,
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
      inst: Some(DecodedInstruction::Arm((Cond::EQ, DecodedArmInstruction::Dummy))),
    }
  }
  pub fn is_decoded(&self) -> bool {
    self.inst.is_some()
  }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ExecutingInstruction {
  pub inst: DecodedInstruction,
  pub addr: u32,
  pub step: usize,
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
      inst: DecodedInstruction::Arm((Cond::EQ, DecodedArmInstruction::Dummy)),
      addr: 0,
      step: 0,
      r15_status: Some(R15Status::NotChanged),
    }
  }
  pub fn is_executed(&self) -> bool {
    self.r15_status.is_some()
  }
}

impl Cpu {
  fn control_hazard(&mut self) {
    self.decoding = DecodingInstruction::dummy();
    self.executing = ExecutingInstruction::dummy();
  }
  pub fn pipeline_next_stage(&mut self) {
    self.fetching = FetchingInstruction::new(self.regs.r15);
    if self.executing.r15_status.unwrap() == R15Status::Changed {
      self.control_hazard();
    } else {
      self.executing = ExecutingInstruction::new(
        self.decoding.addr, self.decoding.inst.unwrap()
      );
      self.decoding = DecodingInstruction::new(
        self.fetching.addr, self.fetching.opcode.unwrap()
      );
    }
  }
  pub fn pipeline_process(&mut self, peripherals: &mut Peripherals) {
    if !self.fetching.is_fetched() {
      if self.fetching.fetch(&mut self.bus, peripherals, self.regs.cpsr.t()) {
        self.regs.r15 += if self.regs.cpsr.t() { 2 } else { 4 };
      }
    }
    if !self.decoding.is_decoded() {
      self.decoding.decode();
    }
    if !self.executing.is_executed() {
      self.executing.execute(&mut self.regs, &mut self.bus, peripherals);
    }
  }
}