pub trait Interruptable {
    fn get_interrupt(&self) -> u8;
    fn reset_interrupt(&mut self);
}
