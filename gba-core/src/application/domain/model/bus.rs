use serde::{Deserialize, Serialize};

use crate::application::domain::model::peripherals::Peripherals;

#[derive(Copy, Clone, Serialize, Deserialize, Default)]
pub struct Bus {
  addr: u32,
  data_width: u32,
  direction: bool,
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
  fn access_type(&mut self, direction: bool, data_width: u32, addr: u32) -> AccessType {
    let mut ret = AccessType::N;
    if data_width > 1 &&
       (data_width == 4 || direction) &&
       self.direction == direction &&
       self.data_width == data_width &&
       (self.addr == addr || self.addr + data_width == addr) {
      ret = AccessType::S;
    }
    self.direction = direction;
    self.data_width = data_width;
    self.addr = addr;
    ret
  }
}