mod optype;

use crate::compressed::optype::COpType;
use crate::{Instruction, Register};

fn decode_addi4spn(full_opcode: u16) -> Result<Instruction, String> {
    match COpType::new_ciw(full_opcode) {
        COpType::CIW { rd, .. } => {
            let imm2   = (full_opcode>>4) & 0b0000000100;
            let imm3   = (full_opcode>>2) & 0b0000001000;
            let imm5_4 = (full_opcode>>7) & 0b0000110000;
            let imm9_6 = (full_opcode>>1) & 0b1111000000;
            let imm = (imm9_6 | imm5_4 | imm3 | imm2) as u64 as i64;
            if imm == 0 {
                return Err("Addi4spn requires non-zero immediate.".to_string());
            }
            Ok(Instruction::Addi { rd, rs1: Register::StackPointer, imm, })
        }
        _ => unreachable!(),
    }
}

fn decode_sd(full_opcode: u16) -> Result<Instruction, String> {
    match COpType::new_cs(full_opcode) {
        COpType::CS { rs1, rs2, .. } => {
            let imm5_3 = full_opcode >> 7 & 0b00111000;
            let imm7_6 = full_opcode << 1 & 0b11000000;
            let imm = (imm7_6 | imm5_3) as u64 as i64;
            Ok(Instruction::Sd { rs1, rs2, imm, })
        }
        _ => unreachable!(),
    }
}

fn decode_addi(full_opcode: u16) -> Result<Instruction, String> {
    match COpType::new_ci(full_opcode) {
        COpType::CI { rd_rs1, .. } => {
            let imm = ((full_opcode as i16 >> 2) & 0b11111 | ((full_opcode << 3) as i16 >> 10)) as i64;
            Ok(Instruction::Addi { rd: rd_rs1, rs1: rd_rs1, imm, })
        }
        _ => unreachable!(),
    }
}

fn decode_li_lui_addi16spn(full_opcode: u16) -> Result<Instruction, String> {
    match COpType::new_ci(full_opcode) {
        COpType::CI { rd_rs1, funct3 } => {
            let imm4_0 = (full_opcode >> 2 & 0b11111) as u8;
            let imm5 = (full_opcode >> 7 & 0b100000) as u8;
            let sign_ext = if imm5 != 0 {0b11000000u8} else {0};
            let imm = (sign_ext | imm5 | imm4_0) as i8 as i64;
            match funct3 {
                0b010 => {
                    if rd_rs1 == Register::Zero {
                        return Err("c.li rd can not be zero!".to_string());
                    }
                    Ok(Instruction::Addi { rd: rd_rs1, rs1: Register::Zero, imm })
                }
                0b011 => {
                    if rd_rs1 == Register::Zero {
                        return Err("c.lui rd can not be zero!".to_string());
                    }
                    if rd_rs1 == Register::StackPointer {
                        let imm4   = (full_opcode>>2) & 0b0000010000;
                        let imm5   = (full_opcode<<3) & 0b0000100000;
                        let imm6   = (full_opcode<<1) & 0b0001000000;
                        let imm8_7 = (full_opcode<<4) & 0b0110000000;
                        let imm9   = (full_opcode>>3) & 0b1000000000;
                        let sign_ext = if imm9 == 0 { 0 } else { 0b1111110000000000u16 };
                        let imm = (sign_ext | imm9 | imm8_7 | imm6 | imm5 | imm4) as i16 as i64;
                        Ok(Instruction::Addi { rd: Register::StackPointer, rs1: Register::StackPointer, imm, })
                    } else {
                        Ok(Instruction::Lui { rd: rd_rs1, uimm: imm as u64 })
                    }
                }
                _ => todo!("TODO: Implement decode_li funct3: 0b{:03b}", funct3)
            }
        }
        _ => unreachable!()
    }
}

fn decode_ld(full_opcode: u16) -> Result<Instruction, String> {
    match COpType::new_cl(full_opcode) {
        COpType::CL { rd, rs1, .. } => {
            let imm5_3 = full_opcode >> 7 & 0b00111000;
            let imm7_6 = full_opcode << 1 & 0b11000000;
            let imm = (imm7_6 | imm5_3) as u64 as i64;
            Ok(Instruction::Ld { rd, rs1, imm, })
        }
        _ => unreachable!()
    }
}

fn decode_jr(full_opcode: u16) -> Result<Instruction, String> {
    match COpType::new_cr(full_opcode) {
        COpType::CR { rd_rs1, rs2, funct4 } => {
            match funct4 {
                0b1000 => {
                    if rd_rs1 == Register::Zero {
                        Err("c.jr/c.mv/etc. rs1 has to be non-zero".to_string())
                    }
                    else if rs2 == Register::Zero {
                        Ok(Instruction::Jalr { rd: Register::Zero, rs1: rd_rs1, imm: 0 })
                    } else {
                        Ok(Instruction::Add { rd: rd_rs1, rs1: Register::Zero, rs2, })
                    }
                },
                0b1001 => {
                    if rd_rs1 == Register::Zero {
                        Ok(Instruction::Ebreak)
                    } else {
                        println!("LOL\n\n\n\n");
                        if rs2 == Register::Zero {
                            Ok(Instruction::Jalr { rd: Register::ReturnAddress, rs1: rd_rs1, imm: 0 })
                        } else {
                            Ok(Instruction::Add { rd: rd_rs1, rs1: rd_rs1, rs2, })
                        }
                    }
                }
                _ => Err(format!("Unknown cr funct4: 0b{funct4:04b}"))
            }
        }
        _ => unreachable!(),
    }
}

fn decode_j(full_opcode: u16) -> Result<Instruction, String> {
    let imm3_1 = (full_opcode>>2) &0b000000001110;
    let imm4 = (full_opcode>>7)   &0b000000010000;
    let imm5 = (full_opcode<<3)   &0b000000100000;
    let imm6 = (full_opcode>>1)   &0b000001000000;
    let imm7 = (full_opcode<<1)   &0b000010000000;
    let imm9_8 = (full_opcode>>1) &0b001100000000;
    let imm10 = (full_opcode<<2)  &0b010000000000;
    let imm11 = (full_opcode>>1)  &0b1000_0000_0000;
    let sign_ext = if imm11 > 0 {0xF000} else {0};
    let imm = (sign_ext | imm3_1 | imm4 | imm5 | imm6 | imm7 | imm9_8 | imm10 | imm11) as i16 as i64;
    Ok(Instruction::Jal { rd: Register::Zero, imm })
}

// TODO: Should this be pub(crate)?
pub(crate) fn decode_compressed(full_opcode: u16) -> Result<Instruction, String> {
    let funct3 = (full_opcode & 0xE000) >> 11;
    match funct3 | (full_opcode & 0b11) {
        0b00000 => decode_addi4spn(full_opcode),
        0b11100 => decode_sd(full_opcode),
        0b00001 => decode_addi(full_opcode),
        0b01001 | 0b01101 => decode_li_lui_addi16spn(full_opcode),
        0b01100 => decode_ld(full_opcode),
        0b10010 => decode_jr(full_opcode),
        0b10101 => decode_j(full_opcode),
        op => Err(format!("TODO: Implement compressed opcode 0b{op:05b}"))
    }
}