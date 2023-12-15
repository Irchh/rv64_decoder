use crate::csr::CsrRegister;
use crate::register::Register;

#[derive(Debug)]
pub(crate) enum OpType {
    R { rd: Register, rs1: Register, rs2: Register, funct3: u8, funct7: u8, },

    I { rd: Register, rs1: Register, funct3: u8, imm: i64, },
    Load { rd: Register, rs1: Register, funct3: u8, imm: i64, },
    Jalr { rd: Register, rs1: Register, funct3: u8, imm: i64, },
    Csr { rd: Register, rs1: Register, funct3: u8, csr: CsrRegister, },

    S { rs1: Register, rs2: Register, funct3: u8, imm: i64, },
    B { rs1: Register, rs2: Register, funct3: u8, imm: i64, },
    U { rd: Register, imm: i64, uimm: u64 },
    Auipc { rd: Register, imm: i64, },
    Lui { rd: Register, imm: u64, },

    J { rd: Register, imm: i64, },
    Jal { rd: Register, imm: i64, },
}

impl OpType {
    pub fn new_r(from: u32) -> Self {
        let rd = Register::from(((from >> 7) & 0b1_1111) as usize);
        let funct3 = ((from >> 12) & 0b0111) as u8;
        let rs1 = Register::from(((from >> 15) & 0b1_1111) as usize);
        let rs2 = Register::from(((from >> 20) & 0b1_1111) as usize);
        let funct7 = ((from >> 25) & 0b0111_1111) as u8;

        OpType::R { rd, rs1, rs2, funct3, funct7, }
    }
    pub fn new_i(from: u32) -> Self {
        let rd = Register::from(((from >> 7) & 0b1_1111) as usize);
        let funct3 = ((from >> 12) & 0b0111) as u8;
        let rs1 = Register::from(((from >>15)&0b1_1111) as usize);
        let imm11_0 = (from >>20)&0b1111_1111_1111;

        // Sign-extend if necessary
        let imm31_12 = if (imm11_0>>11)&1 == 1 { 0xFFFFF } else { 0b0 };

        let full_imm = (imm31_12 << 12) | imm11_0;
        OpType::I { rd, rs1, funct3, imm: full_imm as i32 as i64 }
    }

    pub fn new_csr(from: u32) -> Self {
        let i = Self::new_i(from);
        match i {
            OpType::I { rd, rs1, funct3, imm } => {
                OpType::Csr { rd, rs1, funct3, csr: CsrRegister::from(imm as u64 & 0x0FFF), }
            }
            _ => unreachable!(),
        }
    }
    pub fn new_load(from: u32) -> Self {
        let i = Self::new_i(from);
        // TODO: Check correctness of this
        match i {
            OpType::I { rd, rs1, funct3, imm } => {
                OpType::Load { rd, rs1, funct3, imm, }
            }
            _ => unreachable!(),
        }
    }
    pub fn new_jalr(from: u32) -> Self {
        let i = Self::new_i(from);
        match i {
            OpType::I { rd, rs1, funct3, imm } => {
                OpType::Jalr { rd, rs1, funct3, imm, }
            }
            _ => unreachable!(),
        }
    }
    pub fn new_s(from: u32) -> Self {
        let imm4_0 = (from >>7)&0b1_1111;
        let imm11_5 = (from >>25)&0b0111_1111;
        let funct3 = ((from >>12)&0b0111) as u8;
        let rs1 = Register::from(((from >> 15) & 0b1_1111) as usize);
        let rs2 = Register::from(((from >> 20) & 0b1_1111) as usize);

        // Sign-extend if necessary
        let imm31_12 = if (imm11_5>>6)&1 == 1 { 0xFFFFF } else { 0b0 };

        let full_imm = (imm31_12 << 12) | (imm11_5 << 5) | imm4_0;

        OpType::S { rs1, rs2, funct3, imm: full_imm as i32 as i64, }
    }
    pub fn new_b(from: u32) -> Self {
        let imm11 = (from >>7)&0b1;
        let imm4_1 = (from >>8)&0b1111;
        let imm10_5 = (from >>25)&0b0011_1111;
        let imm12 = (from >>31)&0b1;
        let funct3 = ((from >> 12) & 0b0111) as u8;
        let rs1 = Register::from(((from >> 15) & 0b1_1111) as usize);
        let rs2 = Register::from(((from >> 20) & 0b1_1111) as usize);

        // Sign-extend if necessary
        let imm31_13 = if imm12 == 1 { 0x7FFFF } else { 0b0 };

        let full_imm = (imm31_13 << 13) | (imm12 << 12) | (imm11 << 11) | (imm10_5 << 5) | (imm4_1<<1);
        OpType::B { rs1, rs2, funct3, imm: full_imm as i32 as i64, }
    }
    pub fn new_u(from: u32) -> Self {
        let rd = Register::from(((from >> 7) & 0b1_1111) as usize);
        let imm = (from & 0xFFFFF000) as i32 as i64 >> 12;
        OpType::U { rd, imm, uimm: (from & 0xFFFFF000) as u64 }
    }
    pub fn new_auipc(from: u32) -> Self {
        let i = Self::new_u(from);
        match i {
            OpType::U { rd, imm, uimm:_ } => {
                OpType::Auipc { rd, imm }
            }
            _ => unreachable!(),
        }
    }
    pub fn new_lui(from: u32) -> Self {
        let i = Self::new_u(from);
        match i {
            OpType::U { rd, imm, uimm } => {
                OpType::Lui { rd, imm: uimm }
            }
            _ => unreachable!(),
        }
    }
    pub fn new_j(from: u32) -> Self {
        let rd = Register::from(((from >> 7) & 0b1_1111) as usize);
        let imm19_12 = (from >>12)&0b1111_1111;
        let imm11 = (from >>20)&0b1;
        let imm10_1 = (from >>21)&0b11_1111_1111;
        let imm20 = (from >>31)&0b1;

        // Sign-extend if necessary
        let imm31_21 = if imm20 == 1 { 0x7FF } else { 0b0 };

        let full_imm = (imm31_21 << 21) | (imm20 << 20) | (imm20 << 20) | (imm19_12 << 12) | (imm11 << 11) | (imm10_1 << 1);
        OpType::J { rd, imm: full_imm as i32 as i64, }
    }
    pub fn new_jal(from: u32) -> Self {
        let i = Self::new_j(from);
        match i {
            OpType::J { rd, imm } => {
                OpType::Jal { rd, imm }
            }
            _ => unreachable!(),
        }
    }
}

// TODO: implement compressed instructions