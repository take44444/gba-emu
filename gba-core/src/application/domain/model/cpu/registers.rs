use bitfield::bitfield;
use serde::{Deserialize, Serialize};


bitfield! {
  #[derive(Clone, Serialize, Deserialize, Default)]
  pub struct Cpsr(u32);
  pub m, _: 4, 0;
  pub t, _: 5;
  pub f, _: 6;
  pub i, _: 7;
  reserved, _: 26, 8;
  pub q, _: 27;
  pub v, _: 28;
  pub c, _: 29;
  pub z, _: 30;
  pub n, _: 31;
}

#[derive(Clone, Serialize, Deserialize, Default)]
pub struct Registers {
  pub r0: u32,
  pub r1: u32,
  pub r2: u32,
  pub r3: u32,
  pub r4: u32,
  pub r5: u32,
  pub r6: u32,
  pub r7: u32,
  pub r8: u32,
  pub r9: u32,
  pub r10: u32,
  pub r11: u32,
  pub r12: u32,
  pub r13: u32,
  pub r14: u32,
  pub r15: u32,
  pub cpsr: Cpsr,

  pub r8_fiq: u32,
  pub r9_fiq: u32,
  pub r10_fiq: u32,
  pub r11_fiq: u32,
  pub r12_fiq: u32,
  pub r13_fiq: u32,
  pub r14_fiq: u32,
  pub spsr_fiq: u32,

  pub r13_svc: u32,
  pub r14_svc: u32,
  pub spsr_svc: u32,

  pub r13_abt: u32,
  pub r14_abt: u32,
  pub spsr_abt: u32,

  pub r13_irq: u32,
  pub r14_irq: u32,
  pub spsr_irq: u32,

  pub r13_und: u32,
  pub r14_und: u32,
  pub spsr_und: u32,
}