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
  wait_state::WaitState
};

trait IO<T: Copy> {
  fn read(peripherals: &Peripherals, addr: u32) -> Option<T>;
  fn write(peripherals: &Peripherals, addr: u32, val: T) -> Option<()>;
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Peripherals {
  wait_state: WaitState,
  ppu: Ppu,
  sound_generator: SoundGenerator,
  timer: Timer,
  serial: Serial,
  ewram: Ewram,
  iwram: Iwram,
  bios: Bios,
  game_pak: GamePak,
}

impl IO<u8> for Peripherals {
  fn read(peripherals: &Peripherals, addr: u32) -> Option<u8> {
    None
  }
  fn write(peripherals: &Peripherals, addr: u32, val: u8) -> Option<()> {
    None
  }
}

impl IO<u16> for Peripherals {
  fn read(peripherals: &Peripherals, addr: u32) -> Option<u16> {
    None
  }
  fn write(peripherals: &Peripherals, addr: u32, val: u16) -> Option<()> {
    None
  }
}

impl IO<u32> for Peripherals {
  fn read(peripherals: &Peripherals, addr: u32) -> Option<u32> {
    None
  }
  fn write(peripherals: &Peripherals, addr: u32, val: u32) -> Option<()> {
    None
  }
}

impl Peripherals {
  pub fn new() -> Self {
    Self {
      wait_state: WaitState {},
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