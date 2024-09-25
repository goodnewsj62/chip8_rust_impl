mod memory;
mod register;

pub use register::EightBitRegister;

#[cfg(test)]
mod tests {
    use memory::Memory;
    use register::MemoryTrait;

    use super::*;
    #[test]
    fn test_register_check_operation() {
        let reg = EightBitRegister::init(4);
        assert!(reg.check_bit(2));
        assert!(!reg.check_bit(7));
    }
    #[test]
    fn test_register_set_bits() {
        let mut reg = EightBitRegister::new();
        reg.set_bit(1);

        assert_eq!(reg.get_byte(), 2);

        reg.set_bit(2);
        assert_eq!(reg.get_byte(), 6);
    }

    #[test]
    fn test_register_toggle_operation() {
        let mut reg = EightBitRegister::new();
        reg.toggle_bit(1);

        assert_eq!(reg.get_byte(), 2);
    }

    #[test]
    fn test_register_clear_operation() {
        let mut reg = EightBitRegister::init(7);
        reg.clear_bit(0);
        assert_eq!(reg.get_byte(), 6);
    }

    #[test]
    fn test_create_memory_of_n_size() {
        let mem = Memory::create(4095);
        assert_eq!(mem.data.len(), 512);
    }

    #[test]
    fn test_mem_set_and_check() {
        let mut mem = Memory::create(4095);
        assert!(!mem.check_bit(100));
        mem.set_bit(2);
        assert!(mem.check_bit(2));
        assert_eq!(mem.data[0].get_byte(), 4);
    }
}
