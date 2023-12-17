mod instruction;
mod register;
mod csr;
mod decoder;
mod optype;
mod opcode;
mod compressed;

pub use decoder::decode;
pub use instruction::Instruction;
pub use register::Register;
pub use csr::CsrRegister;

pub fn opcode_size(full_opcode: u32) -> usize {
    if full_opcode&0b11 == 0b11 {
        4
    } else {
        2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add() {
        assert_eq!(
            decode(0x00050433),
            Ok(Instruction::Add { rd: Register::Saved0, rs1: Register::ArgumentRetval0, rs2: Register::Zero })
        );
    }

    #[test]
    fn addi() {
        assert_eq!(
            decode(0x12358513),
            Ok(Instruction::Addi { rd: Register::ArgumentRetval0, rs1: Register::ArgumentRetval1, imm: 0x123 })
        );
    }

    #[test]
    fn and() {
        assert_eq!(
            decode(0x002372b3),
            Ok(Instruction::And { rd: Register::Temp0, rs1: Register::Temp1, rs2: Register::StackPointer })
        );
    }

    #[test]
    fn andi() {
        assert_eq!(
            decode(0xfff1f493),
            Ok(Instruction::Andi { rd: Register::Saved1, rs1: Register::GlobalPointer, imm: -1 })
        );
    }

    #[test]
    fn auipc() {
        assert_eq!(
            decode(0x12345517),
            Ok(Instruction::Auipc { rd: Register::ArgumentRetval0, imm: 0x12345 })
        );
    }

    #[test]
    fn beq() {
        assert_eq!(
            decode(0x00b50663),
            Ok(Instruction::Beq { rs1: Register::ArgumentRetval0, rs2: Register::ArgumentRetval1, imm: 12 })
        );
    }

    #[test]
    fn bge() {
        assert_eq!(
            decode(0x00b55263),
            Ok(Instruction::Bge { rs1: Register::ArgumentRetval0, rs2: Register::ArgumentRetval1, imm: 4 })
        );
    }

    #[test]
    fn bgeu() {
        assert_eq!(
            decode(0x00b57263),
            Ok(Instruction::Bgeu { rs1: Register::ArgumentRetval0, rs2: Register::ArgumentRetval1, imm: 4 })
        );
    }

    #[test]
    fn blt() {
        assert_eq!(
            decode(0x00b54263),
            Ok(Instruction::Blt { rs1: Register::ArgumentRetval0, rs2: Register::ArgumentRetval1, imm: 4 })
        );
    }

    #[test]
    fn bltu() {
        assert_eq!(
            decode(0x00b56263),
            Ok(Instruction::Bltu { rs1: Register::ArgumentRetval0, rs2: Register::ArgumentRetval1, imm: 4 })
        );
    }

    #[test]
    fn bne() {
        assert_eq!(
            decode(0x00b51263),
            Ok(Instruction::Bne { rs1: Register::ArgumentRetval0, rs2: Register::ArgumentRetval1, imm: 4 })
        );
    }
    #[test]
    fn fencei() {
        assert_eq!(
            decode(0x0000100f),
            Ok(Instruction::FenceI)
        );
    }

    #[test]
    fn store() {
        assert_eq!(
            decode(0x0062B023),
            Ok(Instruction::Sd {
                rs1: Register::Temp0,
                rs2: Register::Temp1,
                imm: 0,
            })
        );
        assert_eq!(
            decode(0x0062A023),
            Ok(Instruction::Sw {
                rs1: Register::Temp0,
                rs2: Register::Temp1,
                imm: 0,
            })
        );
        assert_eq!(
            decode(0x00629023),
            Ok(Instruction::Sh {
                rs1: Register::Temp0,
                rs2: Register::Temp1,
                imm: 0,
            })
        );
        assert_eq!(
            decode(0x00628023),
            Ok(Instruction::Sb {
                rs1: Register::Temp0,
                rs2: Register::Temp1,
                imm: 0,
            })
        );
    }
}
