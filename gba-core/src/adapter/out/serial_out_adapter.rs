use serde::{Deserialize, Serialize};

use crate::application::port::out::serial_out_port::SerialOutPort;

#[derive(Clone, Serialize, Deserialize)]
pub struct SerialOutAdapter {}

impl SerialOutPort for SerialOutAdapter {
  fn write() {}
}