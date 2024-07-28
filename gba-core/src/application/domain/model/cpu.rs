use serde::{Deserialize, Serialize};

// ARM7TDMI
#[derive(Clone, Serialize, Deserialize)]
pub struct Cpu {}

impl Cpu {
  pub fn emulate_cycle(&mut self) {}
}