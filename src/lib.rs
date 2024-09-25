mod memory;
mod register;

pub use register::EightBitRegister;

#[cfg(test)]
mod tests {
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
}
