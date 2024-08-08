pub trait Mem {
  fn read8(&self, addr: u32) -> u8;
  fn write8(&mut self, addr: u32, val: u8);
  fn read16(&self, addr: u32) -> u16;
  fn write16(&mut self, addr: u32, val: u16);
  fn read32(&self, addr: u32) -> u32;
  fn write32(&mut self, addr: u32, val: u32);
  fn get_n_cycle(&self, addr: u32) -> usize;
  fn get_s_cycle(&self, addr: u32) -> usize;
}