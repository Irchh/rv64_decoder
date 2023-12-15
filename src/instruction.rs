use std::fmt::{Display, Formatter};
use crate::csr::CsrRegister;
use crate::instruction::Instruction::*;
use crate::optype::OpType;
use crate::register::Register;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Instruction {
    Add{rd: Register, rs1: Register, rs2: Register},
    Sub{rd: Register, rs1: Register, rs2: Register},
    Sll{rd: Register, rs1: Register, rs2: Register},
    Slt{rd: Register, rs1: Register, rs2: Register},
    Sltu{rd: Register, rs1: Register, rs2: Register},
    Xor{rd: Register, rs1: Register, rs2: Register},
    Srl{rd: Register, rs1: Register, rs2: Register},
    Sra{rd: Register, rs1: Register, rs2: Register},
    Or{rd: Register, rs1: Register, rs2: Register},
    And{rd: Register, rs1: Register, rs2: Register},

    Addw{rd: Register, rs1: Register, rs2: Register},
    Subw{rd: Register, rs1: Register, rs2: Register},
    Sllw{rd: Register, rs1: Register, rs2: Register},
    Srlw{rd: Register, rs1: Register, rs2: Register},
    Sraw{rd: Register, rs1: Register, rs2: Register},

    Mul{rd: Register, rs1: Register, rs2: Register},
    Mulh{rd: Register, rs1: Register, rs2: Register},
    Mulhsu{rd: Register, rs1: Register, rs2: Register},
    Mulhu{rd: Register, rs1: Register, rs2: Register},
    Div{rd: Register, rs1: Register, rs2: Register},
    Divu{rd: Register, rs1: Register, rs2: Register},
    Rem{rd: Register, rs1: Register, rs2: Register},
    Remu{rd: Register, rs1: Register, rs2: Register},

    Mulw{rd: Register, rs1: Register, rs2: Register},
    Divw{rd: Register, rs1: Register, rs2: Register},
    Divuw{rd: Register, rs1: Register, rs2: Register},
    Remw{rd: Register, rs1: Register, rs2: Register},
    Remuw{rd: Register, rs1: Register, rs2: Register},


    Addi{rd: Register, rs1: Register, imm: i64},
    Slti{rd: Register, rs1: Register, imm: i64},
    Sltiu{rd: Register, rs1: Register, imm: i64},
    Xori{rd: Register, rs1: Register, imm: i64},
    Ori{rd: Register, rs1: Register, imm: i64},
    Andi{rd: Register, rs1: Register, imm: i64},
    Slli{rd: Register, rs1: Register, shamt: u64},
    Srli{rd: Register, rs1: Register, shamt: u64},
    Srai{rd: Register, rs1: Register, shamt: u64},

    Addiw{rd: Register, rs1: Register, imm: i64},
    Slliw{rd: Register, rs1: Register, shamt: u64},
    Srliw{rd: Register, rs1: Register, shamt: u64},
    Sraiw{rd: Register, rs1: Register, shamt: u64},

    Sb{rs1: Register, rs2: Register, imm: i64},
    Sh{rs1: Register, rs2: Register, imm: i64},
    Sw{rs1: Register, rs2: Register, imm: i64},
    Sd{rs1: Register, rs2: Register, imm: i64},

    Lb{rd: Register, rs1: Register, imm: i64},
    Lh{rd: Register, rs1: Register, imm: i64},
    Lw{rd: Register, rs1: Register, imm: i64},
    Lbu{rd: Register, rs1: Register, imm: i64},
    Lhu{rd: Register, rs1: Register, imm: i64},
    Lwu{rd: Register, rs1: Register, imm: i64},
    Ld{rd: Register, rs1: Register, imm: i64},

    Beq{rs1: Register, rs2: Register, imm: i64},
    Bne{rs1: Register, rs2: Register, imm: i64},
    Blt{rs1: Register, rs2: Register, imm: i64},
    Bltu{rs1: Register, rs2: Register, imm: i64},
    Bge{rs1: Register, rs2: Register, imm: i64},
    Bgeu{rs1: Register, rs2: Register, imm: i64},

    Auipc{rd: Register, imm: i64},
    Lui{rd: Register, uimm: u64},
    Jalr{rd: Register, rs1: Register, imm: i64},
    Jal{rd: Register, imm: i64},

    Ecall,
    Ebreak,
    Uret,
    Sret,
    Wfi,
    Mret,
    Csrrw {rd: Register, rs1: Register, csr: CsrRegister},
    Csrrs {rd: Register, rs1: Register, csr: CsrRegister},
    Csrrc {rd: Register, rs1: Register, csr: CsrRegister},
    Csrrwi {rd: Register, csr: CsrRegister, imm: i64},
    Csrrsi {rd: Register, csr: CsrRegister, imm: i64},
    Csrrci {rd: Register, csr: CsrRegister, imm: i64},

    FenceI,
}

struct Num(i64);

impl Display for Num {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "\x1B[94m{}\x1B[0m(\x1B[94m0x{:0X}\x1B[0m)", self.0, self.0 as u64)
    }
}

impl Display for Instruction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let _ = write!(f, "\x1B[38;5;196m");
        let retval = match self {
            Mul { rd, rs1, rs2 } => write!(f, "mul {}, {}, {}", rd, rs1, rs2),
            Div { rd, rs1, rs2 } => write!(f, "div {}, {}, {}", rd, rs1, rs2),
            Add { rd, rs1, rs2 } => write!(f, "add {}, {}, {}", rd, rs1, rs2),
            Sub { rd, rs1, rs2 } => write!(f, "sub {}, {}, {}", rd, rs1, rs2),
            Or { rd, rs1, rs2 } => write!(f, "or {}, {}, {}", rd, rs1, rs2),
            Xor { rd, rs1, rs2 } => write!(f, "xor {}, {}, {}", rd, rs1, rs2),
            And { rd, rs1, rs2 } => write!(f, "and {}, {}, {}", rd, rs1, rs2),
            Sll { rd, rs1, rs2 } => write!(f, "sll {}, {}, {}", rd, rs1, rs2),
            Slt { rd, rs1, rs2 } => write!(f, "slt {}, {}, {}", rd, rs1, rs2),
            Sltu { rd, rs1, rs2 } => write!(f, "sltu {}, {}, {}", rd, rs1, rs2),
            Srl { rd, rs1, rs2 } => write!(f, "srl {}, {}, {}", rd, rs1, rs2),
            Sra { rd, rs1, rs2 } => write!(f, "sra {}, {}, {}", rd, rs1, rs2),
            Addi { rd, rs1, imm } => write!(f, "addi {}, {}, {}", rd, rs1, Num(*imm)),
            Slli { rd, rs1, shamt } => write!(f, "slli {}, {}, {}", rd, rs1, Num(*shamt as i64)),
            Srli { rd, rs1, shamt } => write!(f, "srli {}, {}, {}", rd, rs1, Num(*shamt as i64)),
            Ori { rd, rs1, imm } => write!(f, "ori {}, {}, {}", rd, rs1, Num(*imm)),
            Andi { rd, rs1, imm } => write!(f, "andi {}, {}, {}", rd, rs1, Num(*imm)),
            Addiw { rd, rs1, imm } => write!(f, "addiw {}, {}, {}", rd, rs1, Num(*imm)),
            Sb { rs1, rs2, imm } => write!(f, "sb {}, {}({})", rs2, Num(*imm), rs1),
            Sh { rs1, rs2, imm } => write!(f, "sh {}, {}({})", rs2, Num(*imm), rs1),
            Sw { rs1, rs2, imm } => write!(f, "sw {}, {}({})", rs2, Num(*imm), rs1),
            Sd { rs1, rs2, imm } => write!(f, "sd {}, {}({})", rs2, Num(*imm), rs1),
            Lb { rd, rs1, imm } => write!(f, "lb {}, {}({})", rd, Num(*imm), rs1),
            Lh { rd, rs1, imm } => write!(f, "lh {}, {}({})", rd, Num(*imm), rs1),
            Lw { rd, rs1, imm } => write!(f, "lw {}, {}({})", rd, Num(*imm), rs1),
            Ld { rd, rs1, imm } => write!(f, "ld {}, {}({})", rd, Num(*imm), rs1),
            Beq { rs1, rs2, imm } => write!(f, "beq {}, {}, {}", rs1, rs2, Num(*imm)),
            Bne { rs1, rs2, imm } => write!(f, "bne {}, {}, {}", rs1, rs2, Num(*imm)),
            Blt { rs1, rs2, imm } => write!(f, "blt {}, {}, {}", rs1, rs2, Num(*imm)),
            Bltu { rs1, rs2, imm } => write!(f, "bltu {}, {}, {}", rs1, rs2, Num(*imm)),
            Bge { rs1, rs2, imm } => write!(f, "bge {}, {}, {}", rs1, rs2, Num(*imm)),
            Bgeu { rs1, rs2, imm } => write!(f, "bgeu {}, {}, {}", rs1, rs2, Num(*imm)),
            Auipc { rd, imm } => write!(f, "auipc {}, {}", rd, Num(*imm)),
            Lui { rd, uimm } => write!(f, "lui {}, {}", rd, Num(*uimm as i64)),
            Jalr { rd, rs1, imm } => {
                if rd == &Register::Zero && rs1 == &Register::ReturnAddress && *imm == 0 {
                    write!(f, "ret")
                } else {
                    write!(f, "jalr {}, {}, {}", rd, rs1, Num(*imm))
                }
            }
            Jal { rd, imm } => write!(f, "jal {}, {}", rd, Num(*imm)),
            Ecall => write!(f, "ecall"),
            Ebreak => write!(f, "Ebreak"),
            Uret => write!(f, "uret"),
            Sret => write!(f, "sret"),
            Wfi => write!(f, "wfi"),
            Mret => write!(f, "mret"),
            Csrrw { rd, rs1, csr } => write!(f, "csrrw {}, {}, {}", rd, csr, rs1),
            Csrrs { rd, rs1, csr } => write!(f, "csrrs {}, {}, {}", rd, csr, rs1),
            Csrrc { rd, rs1, csr } => write!(f, "csrrc {}, {}, {}", rd, csr, rs1),
            Csrrwi { rd, csr, imm } => write!(f, "csrrwi {}, {}, {}", rd, csr, Num(*imm)),
            Csrrsi { rd, csr, imm } => write!(f, "csrrsi {}, {}, {}", rd, csr, Num(*imm)),
            Csrrci { rd, csr, imm } => write!(f, "csrrci {}, {}, {}", rd, csr, Num(*imm)),
            FenceI => write!(f, "fence.i"),
            _ => write!(f, "TODO: impl display for {:?}", self)
        };
        let _ = write!(f, "\x1B[0m");
        retval
    }
}

impl Instruction {
    pub(crate) fn opcode_type(full_opcode: u32) -> Result<OpType, String> {
        let opcode = full_opcode & 0b0111_1111;
        Ok(match opcode {
            0x37 => OpType::new_lui(full_opcode),
            0x17 => OpType::new_auipc(full_opcode),
            0x6f => OpType::new_jal(full_opcode),
            0x67 => OpType::new_jalr(full_opcode),

            0x73 => OpType::new_csr(full_opcode),

            0x0F | 0x13 | 0x1b => OpType::new_i(full_opcode),

            0x03 => OpType::new_load(full_opcode),
            0x23 => OpType::new_s(full_opcode),
            0x63 => OpType::new_b(full_opcode),
            0x33 | 0x3b => OpType::new_r(full_opcode),
            _ => {
                return Err(format!("Unknown opcode: 0b{opcode:07b} (0x{opcode:02X})"))
            }
        })
    }
}