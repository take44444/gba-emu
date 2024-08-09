use crate::application::domain::model::{
  bus::Bus,
  cpu::{
    pipeline::{
      ExecutingInstruction,
      R15Status,
    },
    registers::Registers,
  },
  mem::Mem,
};

impl ExecutingInstruction {
  pub fn arm_b(&mut self, regs: &mut Registers, bus: &mut Bus, mem: &mut impl Mem, nn: i32) -> Option<R15Status> {
    regs.r15 = (self.addr + 8).wrapping_add(nn as u32);
    Some(R15Status::Changed)
  }
}