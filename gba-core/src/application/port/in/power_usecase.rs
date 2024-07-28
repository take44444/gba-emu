pub trait PowerUseCase {
  fn power_on(&mut self);
  fn power_off(&mut self);
}