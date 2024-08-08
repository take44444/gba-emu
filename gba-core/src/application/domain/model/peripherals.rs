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

#[derive(Clone, Serialize, Deserialize)]
pub struct Peripherals {
  pub wait_state: WaitState,
  pub ppu: Ppu,
  pub sound_generator: SoundGenerator,
  pub timer: Timer,
  pub serial: Serial,
  pub ewram: Ewram,
  pub iwram: Iwram,
  pub bios: Bios,
  pub game_pak: GamePak,
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