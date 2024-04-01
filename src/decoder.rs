use crate::instruction::{FenceFlags, Instruction};
use crate::optype::OpType;
use crate::Register;
use crate::compressed::decode_compressed;

fn decode_load(full_opcode: u32) -> Result<Instruction, String> {
    match OpType::new_load(full_opcode) {
        OpType::Load { rd, rs1, funct3, imm } => {
            Ok(match funct3 {
                0b000 => Instruction::Lb { rd, rs1, imm, },
                0b001 => Instruction::Lh { rd, rs1, imm, },
                0b010 => Instruction::Lw { rd, rs1, imm, },
                0b100 => Instruction::Lbu { rd, rs1, imm, },
                0b101 => Instruction::Lhu { rd, rs1, imm, },
                0b110 => Instruction::Lwu { rd, rs1, imm, },
                0b011 => Instruction::Ld { rd, rs1, imm, },
                _ => return Err(format!("Invalid LOAD funct3: 0b{funct3:03b}")),
            })
        }
        _ => unreachable!()
    }
}

fn decode_misc_mem(full_opcode: u32) -> Result<Instruction, String> {
    match OpType::new_i(full_opcode) {
        OpType::I { rd, rs1, funct3, imm } => {
            if rd != Register::Zero || rs1 != Register::Zero {
                return Err("MISC-MEM error".to_string());
            }
            match funct3 {
                0b000 => {
                    let succ = FenceFlags::from(((full_opcode >> 20) & 0b1111) as u8);
                    let pred = FenceFlags::from(((full_opcode >> 24) & 0b1111) as u8);
                    Ok(Instruction::Fence { pred, succ })
                }
                0b001 => {
                    Ok(Instruction::FenceI)
                }
                _ => Err(format!("Invalid MISC-MEM funct3: 0b{funct3:03b}")),
            }
        }
        _ => unreachable!(),
    }
}

fn decode_op_imm(full_opcode: u32) -> Result<Instruction, String> {
    match OpType::new_i(full_opcode) {
        OpType::I { rd, rs1, funct3, imm } => {
            Ok(match funct3 {
                0b000 => Instruction::Addi { rd, rs1, imm, },
                0b010 => Instruction::Slti { rd, rs1, imm, },
                0b011 => Instruction::Sltiu { rd, rs1, imm, },
                0b100 => Instruction::Xori { rd, rs1, imm, },
                0b110 => Instruction::Ori { rd, rs1, imm, },
                0b111 => Instruction::Andi { rd, rs1, imm, },
                0b001 => Instruction::Slli { rd, rs1, shamt: (imm & 0b11111) as u64 },
                0b101 => {
                    match imm&0b11111000000 {
                        0b00000000000 => Instruction::Srli { rd, rs1, shamt: (imm & 0b111111) as u64, },
                        0b01000000000 => Instruction::Srai { rd, rs1, shamt: (imm & 0b111111) as u64, },
                        _ => return Err(format!("Invalid immediate shift imm: 0b{imm:011b}"))
                    }
                },
                _ => unreachable!(),
            })
        }
        _ => unreachable!(),
    }
}

fn decode_op_imm_32(full_opcode: u32) -> Result<Instruction, String> {
    let opt_inst = decode_op_imm(full_opcode);
    if opt_inst.is_ok() {
        Ok(match opt_inst.unwrap() {
            Instruction::Addi { rd, rs1, imm } => Instruction::Addiw { rd, rs1, imm, },
            Instruction::Slli { rd, rs1, shamt } => {
                if shamt&0b100000 != 0 {
                    return Err("Reserved slliw shamt[5] == 1".to_string());
                }
                Instruction::Slliw { rd, rs1, shamt }
            }
            Instruction::Srli { rd, rs1, shamt } => {
                if shamt&0b100000 != 0 {
                    return Err("Reserved srliw shamt[5] == 1".to_string());
                }
                Instruction::Srliw { rd, rs1, shamt }
            }
            Instruction::Srai { rd, rs1, shamt } => {
                if shamt&0b100000 != 0 {
                    return Err("Reserved sraiw shamt[5] == 1".to_string());
                }
                Instruction::Sraiw { rd, rs1, shamt }
            }
            _ => return Err("Invalid OP-IMM-32".to_string()),
        })
    } else {
        opt_inst
    }
}

fn decode_store(full_opcode: u32) -> Result<Instruction, String> {
    match OpType::new_s(full_opcode) {
        OpType::S { rs1, rs2, funct3, imm } => {
            match funct3 {
                0b000 => Ok(Instruction::Sb { rs1, rs2, imm, }),
                0b001 => Ok(Instruction::Sh { rs1, rs2, imm, }),
                0b010 => Ok(Instruction::Sw { rs1, rs2, imm, }),
                0b011 => Ok(Instruction::Sd { rs1, rs2, imm, }),
                _ => return Err(format!("Invalid STORE funct3: 0b{funct3:03b}"))
            }
        }
        _ => unreachable!()
    }
}

fn decode_amo(full_opcode: u32) -> Result<Instruction, String> {
    match OpType::new_r(full_opcode) {
        OpType::R { rd, rs1, rs2, funct3, funct7 } => {
            let funct5 = (funct7>>2)&0b11111;
            let rl = funct7&1 == 1;
            let aq = funct7&2 == 2;
            match funct3 {
                0b010 => {
                    // AMO 32-bit W
                    match funct5 {
                        0b00000 => Ok(Instruction::Amoaddw { rd, rs1, rs2, aq, rl, }),
                        0b00001 => Ok(Instruction::Amoswapw { rd, rs1, rs2, aq, rl, }),
                        _ => Err(format!("Invalid AMO funct5: 0b{funct5:05b}"))
                    }
                }
                _ => Err(format!("Invalid AMO funct3: 0b{funct3:03b}"))
            }
        }
        _ => unreachable!(),
    }
}

fn decode_op(full_opcode: u32) -> Result<Instruction, String> {
    match OpType::new_r(full_opcode) {
        OpType::R { rd, rs1, rs2, funct3, funct7 } => {
            Ok(match funct7 {
                // Base ISA
                0b0000000 => {
                    match funct3 {
                        0b000 => Instruction::Add { rd, rs1, rs2, },
                        0b001 => Instruction::Sll { rd, rs1, rs2, },
                        0b010 => Instruction::Slt { rd, rs1, rs2, },
                        0b011 => Instruction::Sltu { rd, rs1, rs2, },
                        0b100 => Instruction::Xor { rd, rs1, rs2, },
                        0b101 => Instruction::Srl { rd, rs1, rs2, },
                        0b110 => Instruction::Or { rd, rs1, rs2, },
                        0b111 => Instruction::And { rd, rs1, rs2, },
                        _ => unreachable!(),
                    }
                },
                0b0100000 => {
                    match funct3 {
                        0b000 => Instruction::Sub { rd, rs1, rs2, },
                        0b101 => Instruction::Sra { rd, rs1, rs2, },
                        _ => return Err(format!("Invalid OP funct3 and funct7 combination: 0b{funct3:03b} and 0b{funct7:07b}")),
                    }
                },
                // Math ISA
                0b0000001 => {
                    match funct3 {
                        0b000 => Instruction::Mul { rd, rs1, rs2, },
                        0b001 => Instruction::Mulh { rd, rs1, rs2, },
                        0b010 => Instruction::Mulhsu { rd, rs1, rs2, },
                        0b011 => Instruction::Mulhu { rd, rs1, rs2, },
                        0b100 => Instruction::Div { rd, rs1, rs2, },
                        0b101 => Instruction::Divu { rd, rs1, rs2, },
                        0b110 => Instruction::Rem { rd, rs1, rs2, },
                        0b111 => Instruction::Remu { rd, rs1, rs2, },
                        _ => unreachable!(),
                    }
                }
                _ => return Err(format!("Invalid OP funct7: 0b{funct7:07b}")),
            })
        }
        _ => unreachable!(),
    }
}

fn decode_lui(full_opcode: u32) -> Result<Instruction, String> {
    match OpType::new_lui(full_opcode) {
        OpType::Lui { rd, imm } => Ok(Instruction::Lui { rd, uimm: imm }),
        _ => unreachable!()
    }
}

fn decode_op_32(full_opcode: u32) -> Result<Instruction, String> {
    let opt_inst = decode_op(full_opcode);
    if opt_inst.is_ok() {
        Ok(match opt_inst.unwrap() {
            Instruction::Add { rd, rs1, rs2 } => Instruction::Addw { rd, rs1, rs2 },
            Instruction::Sub { rd, rs1, rs2 } => Instruction::Subw { rd, rs1, rs2 },
            Instruction::Sll { rd, rs1, rs2 } => Instruction::Sllw { rd, rs1, rs2 },
            Instruction::Srl { rd, rs1, rs2 } => Instruction::Srlw { rd, rs1, rs2 },
            Instruction::Sra { rd, rs1, rs2 } => Instruction::Sraw { rd, rs1, rs2 },

            Instruction::Mul { rd, rs1, rs2 } => Instruction::Mulw { rd, rs1, rs2 },
            Instruction::Div { rd, rs1, rs2 } => Instruction::Divw { rd, rs1, rs2 },
            Instruction::Divu { rd, rs1, rs2 } => Instruction::Divuw { rd, rs1, rs2 },
            Instruction::Rem { rd, rs1, rs2 } => Instruction::Remw { rd, rs1, rs2 },
            Instruction::Remu { rd, rs1, rs2 } => Instruction::Remuw { rd, rs1, rs2 },

            _ => return Err("Invalid OP-32".to_string()),
        })
    } else {
        opt_inst
    }
}

fn decode_branch(full_opcode: u32) -> Result<Instruction, String> {
    match OpType::new_b(full_opcode) {
        OpType::B { rs1, rs2, funct3, imm } => {
            Ok(match funct3 {
                0b000 => Instruction::Beq { rs1, rs2, imm, },
                0b001 => Instruction::Bne { rs1, rs2, imm, },
                0b100 => Instruction::Blt { rs1, rs2, imm, },
                0b101 => Instruction::Bge { rs1, rs2, imm, },
                0b110 => Instruction::Bltu { rs1, rs2, imm, },
                0b111 => Instruction::Bgeu { rs1, rs2, imm, },
                _ => return Err(format!("Invalid branch funct3: 0b{funct3:03b}")),
            })
        }
        _ => unreachable!(),
    }
}

fn decode_jalr(full_opcode: u32) -> Result<Instruction, String> {
    match OpType::new_jalr(full_opcode) {
        OpType::Jalr { rd, rs1, funct3, imm } => {
            Ok(match funct3 {
                0b000 => Instruction::Jalr { rd, rs1, imm, },
                _ => return Err(format!("Invalid jalr funct3: 0b{funct3:03b}")),
            })
        }
        _ => unreachable!()
    }
}

fn decode_jal(full_opcode: u32) -> Result<Instruction, String> {
    match OpType::new_jal(full_opcode) {
        OpType::Jal { rd, imm } => Ok(Instruction::Jal { rd, imm }),
        _ => unreachable!()
    }
}

fn decode_system(full_opcode: u32) -> Result<Instruction, String> {
    match OpType::new_csr(full_opcode) {
        OpType::Csr { rd, rs1, funct3, csr } => {
            // Remove sign extension
            Ok(match funct3 {
                0b000 => {
                    if rd != Register::Zero || rs1 != Register::Zero {
                        return Err("Invalid rd and rs1 for ECALL/EBREAK".to_string())
                    }
                    match usize::from(csr) {
                        0 => Instruction::Ecall,
                        1 => Instruction::Ebreak,
                        _ => return Err("Invalid immediate for ECALL/EBREAK".to_string()),
                    }
                }
                0b001 => Instruction::Csrrw { rd, rs1, csr, },
                0b010 => Instruction::Csrrs { rd, rs1, csr, },
                0b011 => Instruction::Csrrc { rd, rs1, csr, },
                0b101 => Instruction::Csrrwi { rd, imm: rs1 as usize as i64, csr, },
                0b110 => Instruction::Csrrsi { rd, imm: rs1 as usize as i64, csr, },
                0b111 => Instruction::Csrrci { rd, imm: rs1 as usize as i64, csr, },
                _ => return Err(format!("Invalid system funct3: 0b{funct3:03b}")),
            })
        }
        _ => unreachable!()
    }
}

pub fn decode(full_opcode: u32) -> Result<Instruction, String> {
    let opcode = full_opcode&0x7F;
    match opcode&0b11 {
        0b11 => {
            // RV64G
            match (opcode&0b1111100)>>2 {
                0b00000 => decode_load(full_opcode),
                0b00001 => Err("TODO: Implement LOAD-FP".to_string()),
                0b00010 => Err("TODO: Implement custom-0".to_string()),
                0b00011 => decode_misc_mem(full_opcode),
                0b00100 => decode_op_imm(full_opcode),
                0b00101 => {
                    match OpType::new_auipc(full_opcode) {
                        OpType::Auipc { rd, imm } => Ok(Instruction::Auipc { rd, imm }),
                        _ => unreachable!(),
                    }
                },
                0b00110 => decode_op_imm_32(full_opcode),
                0b00111 => Err("TODO: Implement uhhhhhhhhhhhhhhh 48b".to_string()),
                0b01000 => decode_store(full_opcode),
                0b01001 => Err("TODO: Implement STORE-FP".to_string()),
                0b01010 => Err("TODO: Implement custom-1".to_string()),
                0b01011 => decode_amo(full_opcode),
                0b01100 => decode_op(full_opcode),
                0b01101 => decode_lui(full_opcode),
                0b01110 => decode_op_32(full_opcode),
                0b01111 => Err("TODO: Implement uhhhhhhhhhhhhhhh 64b".to_string()),
                0b10000 => Err("TODO: Implement MADD".to_string()),
                0b10001 => Err("TODO: Implement MSUB".to_string()),
                0b10010 => Err("TODO: Implement NMSUB".to_string()),
                0b10011 => Err("TODO: Implement NMADD".to_string()),
                0b10100 => Err("TODO: Implement OP-FP".to_string()),
                0b10101 => Err("TODO: Implement reserved".to_string()),
                0b10110 => Err("TODO: Implement custom-2".to_string()),
                0b10111 => Err("TODO: Implement uhhhhhhhhhhhhhhh 48b".to_string()),
                0b11000 => decode_branch(full_opcode),
                0b11001 => decode_jalr(full_opcode),
                0b11010 => Err("TODO: Implement reserved".to_string()),
                0b11011 => decode_jal(full_opcode),
                0b11100 => decode_system(full_opcode),
                0b11101 => Err("TODO: Implement reserved".to_string()),
                0b11110 => Err("TODO: Implement custom-3".to_string()),
                0b11111 => Err("TODO: Implement uhhhhhhhhhhhhhhh >=80b".to_string()),
                _ => unreachable!(),
            }
        }
        _ => {
            decode_compressed(full_opcode as u16)
        }
    }
}
