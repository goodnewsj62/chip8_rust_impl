use crate::register::{self, EightBitRegister};

pub struct Memory {
    pub data: Vec<register::EightBitRegister>,
    len: usize,
}

impl Memory {
    fn create(bytes: usize) -> Self {
        let slots = (bytes as f32 / 8_f32).round() as usize;

        Memory {
            data: (0..slots)
                .map(|_| register::EightBitRegister::new())
                .collect(),
            len: slots,
        }
    }

    fn check_range(&self, position: usize) {
        if position > self.len {
            panic!("Invalid memory space max bit {}", self.len);
        }
    }
}

impl register::MemoryTrait for Memory {
    fn check_bit(&self, position: usize) -> bool {
        self.check_range(position);
        self.data[position / 8].check_bit(position % 8)
    }

    fn clear_bit(&mut self, position: usize) {
        self.check_range(position);
        self.data[position / 8].check_bit(position % 8);
    }

    fn set_bit(&mut self, position: usize) {
        self.check_range(position);
        self.data[position / 8].set_bit(position % 8);
    }
    fn toggle_bit(&mut self, position: usize) {
        self.check_range(position);
        self.data[position / 8].toggle_bit(position % 8);
    }
}
