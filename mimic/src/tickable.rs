pub trait Tickable {
    fn tick(&mut self, cycles: u8);
}
