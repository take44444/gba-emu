use serde::{Deserialize, Serialize};

use crate::application::domain::model::{
  bios::Bios,
  ewram::Ewram,
  game_pak::GamePak,
  iwram::Iwram,
  ppu::Ppu,
  serial::Serial,
  sound_generator::SoundGenerator,
  timer::Timer,
};

use super::mem::Mem;


#[derive(Clone, Serialize, Deserialize)]
pub struct  SystemControl {
  waitcnt: u32,
  postflg: u32,
  memcnt: u32,
}

impl SystemControl {
  fn new() -> Self {
    Self {
      waitcnt: 0,
      postflg: 0,
      memcnt: 0x0D00_0020,
    }
  }
  pub fn get_n_cycle(&self) -> usize {
    0
  }
  pub fn get_s_cycle(&self) -> usize {
    0
  }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Peripherals {
  pub system_control: SystemControl,
  pub ppu: Ppu,
  pub sound_generator: SoundGenerator,
  pub timer: Timer,
  pub serial: Serial,
  pub ewram: Ewram,
  pub iwram: Iwram,
  pub bios: Bios,
  pub game_pak: GamePak,
}

impl Mem for Peripherals {
  fn read8(&self, addr: u32) -> u8 {
    0
  }
  fn write8(&mut self, addr: u32, val: u8) {
    ()
  }
  fn read16(&self, addr: u32) -> u16 {
    0
  }
  fn write16(&mut self, addr: u32, val: u16) {
    ()
  }
  fn read32(&self, addr: u32) -> u32 {
    0
  }
  fn write32(&mut self, addr: u32, val: u32) {
    ()
  }
}

impl Peripherals {
  pub fn new() -> Self {
    Self {
      system_control: SystemControl::new(),
      ppu: Ppu {},
      sound_generator: SoundGenerator {},
      timer: Timer {},
      serial: Serial {},
      ewram: Ewram {},
      iwram: Iwram {},
      bios: Bios {},
      game_pak: GamePak {}
    }
  }

  pub fn emulate_cycle(&mut self) {}
}