use serde::{Deserialize, Serialize};

use instructions::{
  DecodingInstruction,
  ExecutingInstruction,
  FetchingInstruction,
  Opcode,
  R15Status,
};
use registers::Registers;

use crate::application::domain::model::{
  bus::Bus,
  peripherals::Peripherals
};

mod instructions;
mod registers;

// ARM7TDMI
#[derive(Clone, Serialize, Deserialize)]
pub struct Cpu {
  regs: Registers,
  fetching: FetchingInstruction,
  decoding: DecodingInstruction,
  executing: ExecutingInstruction,
}

impl Cpu {
  pub fn new() -> Self {
    Self {
      regs: Registers::default(),
      fetching: FetchingInstruction::dummy(),
      decoding: DecodingInstruction::dummy(),
      executing: ExecutingInstruction::dummy(),
    }
  }
  pub fn emulate_cycle(&mut self, peripherals: &Peripherals) {
    if self.fetching.is_fetched() &&
       self.decoding.is_decoded() &&
       self.executing.is_executed()
    {
      self.pipeline_next_stage();
    }
    self.pipeline_process(peripherals);
  }
  fn control_hazard(&mut self) {
    self.decoding = DecodingInstruction::dummy();
    self.executing = ExecutingInstruction::dummy();
  }
  fn pipeline_next_stage(&mut self) {
    self.fetching = FetchingInstruction::new(self.executing.bus, self.regs.r15);
    if self.executing.r15_status.unwrap() == R15Status::Changed {
      self.control_hazard();
    } else {
      self.executing = ExecutingInstruction::new(
        self.decoding.bus, self.decoding.addr, self.decoding.inst.unwrap()
      );
      self.decoding = DecodingInstruction::new(
        self.fetching.bus, self.fetching.addr, self.fetching.opcode.unwrap()
      );
    }
  }
  fn pipeline_process(&mut self, peripherals: &Peripherals) {
    if !self.fetching.is_fetched() {
      if self.fetching.fetch(peripherals, self.regs.cpsr.t()) {
        self.regs.r15 += if self.regs.cpsr.t() { 2 } else { 4 };
      }
    }
    if !self.decoding.is_decoded() {
      self.decoding.decode();
    }
    if !self.executing.is_executed() {
      self.executing.execute(&mut self.regs, peripherals);
    }
  }
}