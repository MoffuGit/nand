use crate::computer::gate::{and, or, xor};

pub fn half_adder(a: bool, b: bool) -> (bool, bool) {
    (xor(a, b), and(a, b))
}

pub fn full_adder(a: bool, b: bool, c: bool) -> (bool, bool) {
    let (sum0, carry0) = half_adder(a, b);
    let (sum, carry1) = half_adder(sum0, c);
    (sum, or(carry0, carry1))
}

pub fn add_16(a: [bool; 16], b: [bool; 16]) -> [bool; 16] {
    let mut carry = false;
    let mut out = [false; 16];
    for (idx, (a, b)) in a.into_iter().zip(b.into_iter()).enumerate() {
        let (sum, carry0) = full_adder(a, b, carry);
        out[idx] = sum;
        carry = carry0
    }
    out
}

pub fn inc_16(input: [bool; 16]) -> [bool; 16] {
    let mut carry = true;
    let mut out = [false; 16];

    for (idx, a) in input.into_iter().enumerate() {
        let (sum, carry0) = half_adder(a, carry);
        out[idx] = sum;
        carry = carry0;
    }
    out
}

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum AluOp {
    Zero = 0b101010,     // 0
    One = 0b111111,      // 1
    NegOne = 0b111010,   // -1
    X = 0b001100,        // x
    Y = 0b110000,        // y
    NotX = 0b001101,     // !x
    NotY = 0b110001,     // !y
    NegX = 0b001111,     // -x
    NegY = 0b110011,     // -y
    IncX = 0b011111,     // x + 1
    IncY = 0b110111,     // y + 1
    DecX = 0b001110,     // x - 1
    DecY = 0b110010,     // y - 1
    Add = 0b000010,      // x + y
    Sub = 0b010011,      // x - y
    SubFromY = 0b000111, // y - x
    And = 0b000000,      // x & y
    Or = 0b010101,       // x | y
}

pub fn alu(x: u16, y: u16, op: AluOp) -> (u16, bool, bool) {
    let flags = op as u8;

    let zx = (flags & 0b100000) != 0;
    let nx = (flags & 0b010000) != 0;
    let zy = (flags & 0b001000) != 0;
    let ny = (flags & 0b000100) != 0;
    let f = (flags & 0b000010) != 0;
    let no = (flags & 0b000001) != 0;

    let x0 = if zx { 0 } else { x };
    let y0 = if zy { 0 } else { y };

    let x1 = if nx { !x0 } else { x0 };
    let y1 = if ny { !y0 } else { y0 };

    let computed = if f { x1.wrapping_add(y1) } else { x1 & y1 };

    let out = if no { !computed } else { computed };

    let zr = out == 0;
    let ng = (out & 0x8000) != 0;

    (out, zr, ng)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_operations() {
        // Test 0
        let (result, zr, ng) = alu(42, 24, AluOp::Zero);
        assert_eq!(result, 0);
        assert!(zr);
        assert!(!ng);

        // Test 1
        let (result, zr, ng) = alu(42, 24, AluOp::One);
        assert_eq!(result, 1);
        assert!(!zr);
        assert!(!ng);

        // Test -1
        let (result, zr, ng) = alu(42, 24, AluOp::NegOne);
        assert_eq!(result, u16::MAX); // -1 in two's complement
        assert!(!zr);
        assert!(ng);
    }

    #[test]
    fn test_pass_through() {
        // Test X
        let (result, zr, ng) = alu(42, 24, AluOp::X);
        assert_eq!(result, 42);
        assert!(!zr);
        assert!(!ng);

        // Test Y
        let (result, zr, ng) = alu(42, 24, AluOp::Y);
        assert_eq!(result, 24);
        assert!(!zr);
        assert!(!ng);
    }

    #[test]
    fn test_increment_operations() {
        // Test IncX
        let (result, zr, ng) = alu(42, 24, AluOp::IncX);
        assert_eq!(result, 43);
        assert!(!zr);
        assert!(!ng);

        // Test IncY
        let (result, zr, ng) = alu(42, 24, AluOp::IncY);
        assert_eq!(result, 25);
        assert!(!zr);
        assert!(!ng);

        // Test increment of max value (should wrap)
        let (result, zr, ng) = alu(u16::MAX, 0, AluOp::IncX);
        assert_eq!(result, 0);
        assert!(zr);
        assert!(!ng);
    }

    #[test]
    fn test_decrement_operations() {
        // Test DecX
        let (result, zr, ng) = alu(42, 24, AluOp::DecX);
        assert_eq!(result, 41);
        assert!(!zr);
        assert!(!ng);

        // Test DecY
        let (result, zr, ng) = alu(42, 24, AluOp::DecY);
        assert_eq!(result, 23);
        assert!(!zr);
        assert!(!ng);

        // Test decrement of 0 (should wrap)
        let (result, zr, ng) = alu(0, 0, AluOp::DecX);
        assert_eq!(result, u16::MAX);
        assert!(!zr);
        assert!(ng);
    }

    #[test]
    fn test_arithmetic_operations() {
        // Test Add
        let (result, zr, ng) = alu(42, 24, AluOp::Add);
        assert_eq!(result, 66);
        assert!(!zr);
        assert!(!ng);

        // Test Sub
        let (result, zr, ng) = alu(42, 24, AluOp::Sub);
        assert_eq!(result, 18);
        assert!(!zr);
        assert!(!ng);

        // Test SubFromY (y - x)
        let (result, zr, ng) = alu(42, 24, AluOp::SubFromY);
        assert_eq!(result, 65518); // 24 - 42 = -18 in two's complement
        assert!(!zr);
        assert!(ng);
    }

    #[test]
    fn test_logical_operations() {
        // Test And
        let (result, zr, ng) = alu(0b1100, 0b1010, AluOp::And);
        assert_eq!(result, 0b1000);
        assert!(!zr);
        assert!(!ng);

        // Test Or
        let (result, zr, ng) = alu(0b1100, 0b1010, AluOp::Or);
        assert_eq!(result, 0b1110);
        assert!(!zr);
        assert!(!ng);
    }

    #[test]
    fn test_negative_operations() {
        // Test NotX
        let (result, zr, ng) = alu(0b1100, 0, AluOp::NotX);
        assert_eq!(result, !0b1100);
        assert!(!zr);
        assert!(ng);

        // Test NotY
        let (result, zr, ng) = alu(0, 0b1100, AluOp::NotY);
        assert_eq!(result, !0b1100);
        assert!(!zr);
        assert!(ng);

        // Test NegX
        let (result, zr, ng) = alu(42, 0, AluOp::NegX);
        assert_eq!(result, (-42i16) as u16);
        assert!(!zr);
        assert!(ng);

        // Test NegY
        let (result, zr, ng) = alu(0, 42, AluOp::NegY);
        assert_eq!(result, (-42i16) as u16);
        assert!(!zr);
        assert!(ng);
    }

    #[test]
    fn test_edge_cases() {
        // Test adding numbers that cause overflow
        let (result, zr, ng) = alu(u16::MAX, 1, AluOp::Add);
        assert_eq!(result, 0);
        assert!(zr);
        assert!(!ng);

        // Test subtracting larger from smaller
        let (result, zr, ng) = alu(5, 10, AluOp::Sub);
        assert_eq!(result, (-5i16) as u16);
        assert!(!zr);
        assert!(ng);

        // Test zero results from different operations
        let (result, zr, ng) = alu(0, 0, AluOp::And);
        assert_eq!(result, 0);
        assert!(zr);
        assert!(!ng);
    }
}
