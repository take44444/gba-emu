use serde::{Deserialize, Serialize};

use crate::application::domain::model::peripherals::Peripherals;

#[derive(Clone, Serialize, Deserialize, Default)]
pub struct Bus {
  accessed: u32,
  size: u32,
  cycle: usize,
}

#[derive(Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
enum AccessType {
  N,
  S,
}

impl Bus {
  pub fn read8(&self, addr: u32, peripherals: &Peripherals) -> Option<u8> {
    None
  }
  pub fn write8(&self, addr: u32, val: u8, peripherals: &Peripherals) -> Option<()> {
    None
  }
  pub fn read16(&self, addr: u32, peripherals: &Peripherals) -> Option<u16> {
    None
  }
  pub fn write16(&self, addr: u32, val: u16, peripherals: &Peripherals) -> Option<()> {
    None
  }
  pub fn read32(&self, addr: u32, peripherals: &Peripherals) -> Option<u32> {
    None
  }
  pub fn write32(&self, addr: u32, val: u32, peripherals: &Peripherals) -> Option<()> {
    None
  }
  fn access_type(&self, size: u32, addr: u32) -> AccessType {
    if self.size == size && self.accessed + size == addr {
      AccessType::S
    } else {
      AccessType::N
    }
  }
}