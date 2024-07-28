use serde::{Deserialize, Serialize};

use crate::application::{
  domain::model::gameboy::Gameboy,
  port::r#in::{
    emulate_cycle_usecase::EmulateCycleUseCase,
    joypad_input_usecase::JoypadInputUseCase,
    power_usecase::PowerUseCase, save_usecase::SaveUseCase, 
    serial_usecase::SerialUseCase
  }
};

#[derive(Clone, Serialize, Deserialize)]
pub struct GameBoyService(Option<Gameboy>);

impl GameBoyService {
  pub fn new() -> Self {
    Self(None)
  }
}

impl PowerUseCase for GameBoyService {
  fn power_on(&mut self) {
    self.0 = Some(Gameboy::new());
  }
  fn power_off(&mut self) {
    self.0 = None;
  }
}

impl EmulateCycleUseCase for GameBoyService {
  fn emulate_cycle(&mut self) {
    if let Some(gameboy) = self.0.as_mut() {
      gameboy.emulate_cycle();
    }
  }
}

impl JoypadInputUseCase for GameBoyService {
  fn button_down() {}
  fn button_up() {}
}

impl SerialUseCase for GameBoyService {
  fn connect() {}
  fn receive() {}
}

impl SaveUseCase for GameBoyService {
  fn save() {}
}