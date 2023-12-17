use crate::Register;

#[derive(Debug)]
pub(crate) enum COpType {
    CR { rd_rs1: Register, rs2: Register, funct4: u8 },
    CI { rd_rs1: Register, funct3: u8 },
    CSS { rs2: Register, funct3: u8 },
    CIW { rd: Register, funct3: u8 },
    CL { rd: Register, rs1: Register, funct3: u8 },
    CS { rs1: Register, rs2: Register, funct3: u8 },
    CB { rs1: Register, funct3: u8 },
    CJ { funct3: u8 },
}


impl COpType {
    fn funct3(full_opcode: u16) -> u8 {
        ((full_opcode & 0xE000) >> 13) as u8
    }
    pub fn new_cr(full_opcode: u16) -> Self {
        let funct4 = ((full_opcode & 0xF000) >> 12) as u8;
        let rd_rs1 = Register::from(((full_opcode >> 7) & 0x1F) as usize);
        let rs2 = Register::from(((full_opcode >> 2) & 0x1F) as usize);
        Self::CR { rd_rs1, rs2, funct4 }
    }
    pub fn new_ci(full_opcode: u16) -> Self {
        let funct3 = Self::funct3(full_opcode);
        let rd_rs1 = Register::from(((full_opcode >> 7 ) & 0x1F) as usize);
        Self::CI { rd_rs1, funct3 }
    }
    pub fn new_css(full_opcode: u16) -> Self {
        let funct3 = Self::funct3(full_opcode);
        let rs2 = Register::from(((full_opcode >> 2) & 0b11111) as usize);
        Self::CSS { rs2, funct3 }
    }
    pub fn new_ciw(full_opcode: u16) -> Self {
        let funct3 = Self::funct3(full_opcode);
        let rd = Register::from_rvc(((full_opcode >> 2) & 0b111) as u8);
        Self::CIW { rd, funct3 }
    }
    pub fn new_cl(full_opcode: u16) -> Self {
        let funct3 = Self::funct3(full_opcode);
        let rd = Register::from_rvc(((full_opcode >> 2) & 0b111) as u8);
        let rs1 = Register::from_rvc(((full_opcode >> 7) & 0b111) as u8);
        Self::CL { rd, rs1, funct3 }
    }
    pub fn new_cs(full_opcode: u16) -> Self {
        let funct3 = Self::funct3(full_opcode);
        let rs1 = Register::from_rvc(((full_opcode >> 7) & 0b111) as u8);
        let rs2 = Register::from_rvc(((full_opcode >> 2) & 0b111) as u8);
        Self::CS { rs1, rs2, funct3 }
    }
    pub fn new_cb(full_opcode: u16) -> Self {
        let funct3 = Self::funct3(full_opcode);
        let rs1 = Register::from_rvc(((full_opcode >> 7) & 0b111) as u8);
        Self::CB { rs1, funct3 }
    }
}