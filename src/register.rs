pub struct EightBitRegister(u8);

pub trait MemoryTrait {
    fn set_bit(&mut self, position: usize);
    fn check_bit(&self, position: usize) -> bool;
    fn clear_bit(&mut self, position: usize);
    fn toggle_bit(&mut self, position: usize);
}

impl EightBitRegister {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        EightBitRegister(0)
    }

    pub fn init(val: u8) -> Self {
        EightBitRegister(val)
    }

    pub fn get_byte(&self) -> u8 {
        self.0
    }

    fn mask(&self, pos: usize) -> u8 {
        if pos > 7 {
            panic!("invalid position for 8bit register!");
        }
        1 << pos
    }
}

impl MemoryTrait for EightBitRegister {
    fn set_bit(&mut self, position: usize) {
        self.0 |= self.mask(position);
    }

    fn check_bit(&self, position: usize) -> bool {
        (self.0 & self.mask(position)) > 0
    }

    fn clear_bit(&mut self, position: usize) {
        self.0 &= !(self.mask(position));
    }

    fn toggle_bit(&mut self, position: usize) {
        self.0 ^= self.mask(position);
    }
}
