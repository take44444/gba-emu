use serde::{Deserialize, Serialize};

use crate::application::domain::model::mem::Mem;

#[derive(Copy, Clone, Serialize, Deserialize, Default)]
pub struct Bus {
  waitcnt: u16,
  guard_id: Option<char>,
  addr: u32,
  data_width: u32,
  direction: bool,
  cycle_cnt: usize,
}

// TODO: address alignment

impl Bus {
  pub fn read8(&mut self, id: char, addr: u32, mem: &impl Mem) -> Option<u8> {
    if self.try_lock(id)? {
      self.cycle_cnt = self.get_cycle(false, 1, addr);
    }
    self.cycle_cnt -= 1;
    if self.cycle_cnt == 0 {
      if addr & !1_u32 == 0x4000204 {
        if addr & 1 == 0 {
          Some((self.waitcnt & 0xff) as u8)
        } else {
          Some((self.waitcnt >> 8) as u8)
        }        
      } else {
        Some(mem.read8(addr))
      }
    } else {
      None
    }
  }
  pub fn write8(&mut self, id: char, addr: u32, val: u8, mem: &mut impl Mem) -> Option<()> {
    if self.try_lock(id)? {
      self.cycle_cnt = self.get_cycle(true, 1, addr);
    }
    self.cycle_cnt -= 1;
    if self.cycle_cnt == 0 {
      if addr & !1_u32 == 0x4000204 {
        if addr & 1 == 0 {
          Some({ self.waitcnt = (self.waitcnt & 0xff00) | (val as u16); })
        } else {
          Some({ self.waitcnt = (self.waitcnt & 0xff) | ((val as u16) << 8); })
        }        
      } else {
        Some(mem.write8(addr, val))
      }
    } else {
      None
    }
  }
  pub fn read16(&mut self, id: char, addr: u32, mem: &impl Mem) -> Option<u16> {
    if self.try_lock(id)? {
      self.cycle_cnt = self.get_cycle(false, 2, addr);
    }
    self.cycle_cnt -= 1;
    if self.cycle_cnt == 0 {
      if addr == 0x4000204 {
        Some(self.waitcnt)
      } else {
        Some(mem.read16(addr))
      }
    } else {
      None
    }
  }
  pub fn write16(&mut self, id: char, addr: u32, val: u16, mem: &mut impl Mem) -> Option<()> {
    if self.try_lock(id)? {
      self.cycle_cnt = self.get_cycle(true, 2, addr);
    }
    self.cycle_cnt -= 1;
    if self.cycle_cnt == 0 {
      if addr == 0x4000204 {
        Some({ self.waitcnt = val; })
      } else {
        Some(mem.write16(addr, val))
      }
    } else {
      None
    }
  }
  pub fn read32(&mut self, id: char, addr: u32, mem: &impl Mem) -> Option<u32> {
    if self.try_lock(id)? {
      self.cycle_cnt = self.get_cycle(false, 4, addr);
    }
    self.cycle_cnt -= 1;
    if self.cycle_cnt == 0 {
      if addr == 0x4000204 {
        Some(self.waitcnt as u32)
      } else {
        Some(mem.read32(addr))
      }
    } else {
      None
    }
  }
  pub fn write32(&mut self, id: char, addr: u32, val: u32, mem: &mut impl Mem) -> Option<()> {
    if self.try_lock(id)? {
      self.cycle_cnt = self.get_cycle(true, 4, addr);
    }
    self.cycle_cnt -= 1;
    if self.cycle_cnt == 0 {
      if addr == 0x4000204 {
        Some({ self.waitcnt = (val & 0xffff) as u16; })
      } else {
        Some(mem.write32(addr, val))
      }
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
  fn get_cycle(&mut self, direction: bool, data_width: u32, addr: u32) -> usize {
    let ret = if data_width > 1 &&
       (data_width == 4 || !direction) &&
       self.direction == direction &&
       self.data_width == data_width &&
       (self.addr == addr || self.addr + data_width == addr)
    {
      // s-cycle
      0
    } else {
      // n-cycle
      0
    };

    self.direction = direction;
    self.data_width = data_width;
    self.addr = addr;
    ret
  }
  fn width(addr: u32) -> u32 {
    4
  }
}