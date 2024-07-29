use crate::application::domain::model::{
  cpu::{
    pipeline::{
      DecodedArmInstruction,
      DecodedInstruction,
      DecodedThumbInstruction,
      ExecutingInstruction,
      R15Status,
    },
    registers::Registers,
  },
  peripherals::Peripherals,
};

mod arm_execute;
mod thumb_execute;

impl ExecutingInstruction {
  pub fn execute(&mut self, regs: &mut Registers, peripherals: &mut Peripherals) -> bool {
    self.r15_status = match self.inst {
      DecodedInstruction::Arm((cond, inst)) => {
        if cond.check(regs.cpsr) {
          match inst {
            DecodedArmInstruction::B(nn) => self.arm_b(regs, peripherals, nn),
            DecodedArmInstruction::Dummy => Some(R15Status::NotChanged),
          }
        } else {
          Some(R15Status::NotChanged)
        }
      },
      DecodedInstruction::Thumb(inst) => {
        match inst {
          DecodedThumbInstruction::B(nn) => self.thumb_b(regs, peripherals, nn),
        }
      }
    };
    self.is_executed()
  }
}