use serde::{Deserialize, Serialize};

use crate::application::{
  domain::model::gameboy_advance::GameboyAdvance,
  port::r#in::{
    emulate_cycle_usecase::EmulateCycleUseCase,
    joypad_input_usecase::JoypadInputUseCase,
    power_usecase::PowerUseCase, save_usecase::SaveUseCase, 
    serial_usecase::SerialUseCase
  }
};

#[derive(Clone, Serialize, Deserialize)]
pub struct GameboyAdvanceService(Option<GameboyAdvance>);

impl GameboyAdvanceService {
  pub fn new() -> Self {
    Self(None)
  }
}

impl PowerUseCase for GameboyAdvanceService {
  fn power_on(&mut self) {
    self.0 = Some(GameboyAdvance::new());
  }
  fn power_off(&mut self) {
    self.0 = None;
  }
}

impl EmulateCycleUseCase for GameboyAdvanceService {
  fn emulate_cycle(&mut self) {
    if let Some(gameboy_advance) = self.0.as_mut() {
      gameboy_advance.emulate_cycle();
    }
  }
}

impl JoypadInputUseCase for GameboyAdvanceService {
  fn button_down() {}
  fn button_up() {}
}

impl SerialUseCase for GameboyAdvanceService {
  fn connect() {}
  fn receive() {}
}

impl SaveUseCase for GameboyAdvanceService {
  fn save() {}
}