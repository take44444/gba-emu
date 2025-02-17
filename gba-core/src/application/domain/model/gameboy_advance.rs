use serde::{Deserialize, Serialize};

use crate::application::domain::model::{
  cpu::Cpu,
  peripherals::Peripherals
};

#[derive(Clone, Serialize, Deserialize)]
pub struct GameboyAdvance {
  cpu: Cpu,
  peripherals: Peripherals,
}

impl GameboyAdvance {
  pub fn new() -> Self {
    Self {
      cpu: Cpu {},
      peripherals: Peripherals::new(),
    }
  }

  pub fn emulate_cycle(&mut self) {
    self.cpu.emulate_cycle();
    self.peripherals.emulate_cycle();
  }
}