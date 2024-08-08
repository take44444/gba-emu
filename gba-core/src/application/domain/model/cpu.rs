use serde::{Deserialize, Serialize};

use crate::application::domain::model::{
  bus::Bus,
  peripherals::Peripherals
};

use pipeline::{
  DecodingInstruction,
  ExecutingInstruction,
  FetchingInstruction,
  Opcode,
  R15Status,
};

use registers::Registers;

mod decode;
mod execute;
mod pipeline;
mod registers;

// ARM7TDMI
#[derive(Clone, Serialize, Deserialize)]
pub struct Cpu {
  regs: Registers,
  fetching: FetchingInstruction,
  decoding: DecodingInstruction,
  executing: ExecutingInstruction,
  bus: Bus,
}

impl Cpu {
  pub fn new() -> Self {
    Self {
      regs: Registers::default(),
      fetching: FetchingInstruction::dummy(),
      decoding: DecodingInstruction::dummy(),
      executing: ExecutingInstruction::dummy(),
      bus: Bus::default(),
    }
  }
  pub fn emulate_cycle(&mut self, peripherals: &mut Peripherals) {
    if self.fetching.is_fetched() &&
       self.decoding.is_decoded() &&
       self.executing.is_executed()
    {
      self.pipeline_next_stage();
    }
    self.pipeline_process(peripherals);
  }
}