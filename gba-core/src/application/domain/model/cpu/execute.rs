use crate::application::domain::model::{
  bus::Bus,
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
  mem::Mem,
};

mod arm_execute;
mod thumb_execute;

impl ExecutingInstruction {
  pub fn execute(&mut self, regs: &mut Registers, bus: &mut Bus, mem: &mut impl Mem) -> bool {
    self.r15_status = match self.inst {
      DecodedInstruction::Arm((cond, inst)) => {
        if cond.check(regs.cpsr) {
          match inst {
            DecodedArmInstruction::B(nn) => self.arm_b(regs, bus, mem, nn),
            DecodedArmInstruction::Dummy => Some(R15Status::NotChanged),
          }
        } else {
          Some(R15Status::NotChanged)
        }
      },
      DecodedInstruction::Thumb(inst) => {
        match inst {
          DecodedThumbInstruction::B(nn) => self.thumb_b(regs, bus, mem, nn),
        }
      }
    };
    self.is_executed()
  }
}