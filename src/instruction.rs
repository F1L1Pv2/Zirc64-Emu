pub struct Instruction {
    value: u16,
}

impl Instruction {
    pub const OPCODE: u16 = 0xF800;

    pub fn new(value: u16) -> Self {
        Self { value }
    }

    pub fn get_operand(&self, mask: u16) -> u16 {
        (self.value & mask) >> mask.trailing_zeros()
    }

    pub fn get_bit(&self, index: u8) -> bool {
        assert!(index < 16);
        self.value >> index & 1 == 1
    }
}
