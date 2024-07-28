use serde::{Deserialize, Serialize};

use crate::application::port::out::audio_port::AudioPort;

#[derive(Clone, Serialize, Deserialize)]
pub struct AudioAdapter {}

impl AudioPort for AudioAdapter {}