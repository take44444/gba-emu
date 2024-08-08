use serde::{Deserialize, Serialize};

use crate::application::domain::model::{
  mem::Mem,
  peripherals::Peripherals,
};

#[derive(Copy, Clone, Serialize, Deserialize, Default)]
pub struct Bus {
  guard_id: Option<char>,
  addr: u32,
  data_width: u32,
  direction: bool,
  cycle_cnt: usize,
}

impl Bus {
  pub fn read8(&mut self, id: char, addr: u32, peripherals: &Peripherals) -> Option<u8> {
    self.try_lock(id)?;
    None
  }
  pub fn write8(&mut self, id: char, addr: u32, val: u8, peripherals: &Peripherals) -> Option<()> {
    self.try_lock(id)?;
    None
  }
  pub fn read16(&mut self, id: char, addr: u32, peripherals: &Peripherals) -> Option<u16> {
    self.try_lock(id)?;
    None
  }
  pub fn write16(&mut self, id: char, addr: u32, val: u16, peripherals: &Peripherals) -> Option<()> {
    self.try_lock(id)?;
    None
  }
  pub fn read32(&mut self, id: char, addr: u32, peripherals: &Peripherals) -> Option<u32> {
    self.try_lock(id)?;
    None
  }
  pub fn write32(&mut self, id: char, addr: u32, val: u32, mem: &impl Mem) -> Option<()> {
    if self.try_lock(id)? {
      self.cycle_cnt = self.get_cycle(peripherals, true, 4, addr);
    }
    self.cycle_cnt -= 1;
    if self.cycle_cnt == 0 {
      Some(peripherals.write(addr, val))
    } else {
      None
    }
  }
  fn try_lock(&mut self, id: char) -> Option<bool> {
    match self.guard_id {
      Some(gid) => if gid == id {
        Some(false)
      } else {
        None
      },
      None => {
        self.guard_id = Some(id);
        Some(true)
      }
    }
  }
  // fn get_cycle(&mut self, peripherals: &Peripherals, direction: bool, data_width: u32, addr: u32) -> usize {
  //   let ret = if data_width > 1 &&
  //      (data_width == 4 || direction) &&
  //      self.direction == direction &&
  //      self.data_width == data_width &&
  //      (self.addr == addr || self.addr + data_width == addr) {
  //   } else {
  //   };

  //   self.direction = direction;
  //   self.data_width = data_width;
  //   self.addr = addr;
  //   ret
  // }
  fn width(addr: u32) -> u32 {
    4
  }
}