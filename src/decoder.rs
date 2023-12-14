use crate::instruction::Instruction;
use crate::instruction::Instruction::*;
use crate::optype::OpType;
use crate::Register;

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
                    match imm&0b11111100000 {
                        0b00000000000 => Instruction::Srli { rd, rs1, shamt: (imm & 0b11111) as u64, },
                        0b01000000000 => Instruction::Srai { rd, rs1, shamt: (imm & 0b11111) as u64, },
                        _ => return Err(format!("Invalid immediate shift imm: 0b{imm:011b}"))
                    }
                },
                _ => unreachable!(),
            })
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

pub fn decode(full_opcode: u32) -> Result<Instruction, String> {
    let opcode = full_opcode&0x7F;
    match opcode&0b11 {
        0b11 => {
            // RV64G
            match (opcode&0b1111100)>>2 {
                0b00000 => Err("TODO: Implement LOAD".to_string()),
                0b00001 => Err("TODO: Implement LOAD-FP".to_string()),
                0b00010 => Err("TODO: Implement custom-0".to_string()),
                0b00011 => {
                    match OpType::new_i(full_opcode) {
                        OpType::I { rd, rs1, funct3, imm } => {
                            if rd != Register::Zero || rs1 != Register::Zero {
                                return Err("MISC-MEM error".to_string());
                            }
                            match funct3 {
                                0b000 => {
                                    Err("Fence instruction not implemented".to_string())
                                }
                                0b001 => {
                                    if imm != 0 {
                                        Err("MISC-MEM fence.i error".to_string())
                                    } else {
                                        Ok(FenceI)
                                    }
                                }
                                _ => Err(format!("Invalid MISC-MEM funct3: 0b{funct3:03b}")),
                            }
                        }
                        _ => unreachable!(),
                    }
                },
                0b00100 => decode_op_imm(full_opcode),
                0b00101 => {
                    match OpType::new_auipc(full_opcode) {
                        OpType::Auipc { rd, imm } => Ok(Instruction::Auipc { rd, imm }),
                        _ => unreachable!(),
                    }
                },
                0b00110 => Err("TODO: Implement OP-IMM-32".to_string()),
                0b00111 => Err("TODO: Implement uhhhhhhhhhhhhhhh 48b".to_string()),
                0b01000 => Err("TODO: Implement STORE".to_string()),
                0b01001 => Err("TODO: Implement STORE-FP".to_string()),
                0b01010 => Err("TODO: Implement custom-1".to_string()),
                0b01011 => Err("TODO: Implement AMO".to_string()),
                0b01100 => decode_op(full_opcode),
                0b01101 => Err("TODO: Implement LUI".to_string()),
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
                0b11001 => Err("TODO: Implement JALR".to_string()),
                0b11010 => Err("TODO: Implement reserved".to_string()),
                0b11011 => Err("TODO: Implement JAL".to_string()),
                0b11100 => Err("TODO: Implement SYSTEM".to_string()),
                0b11101 => Err("TODO: Implement reserved".to_string()),
                0b11110 => Err("TODO: Implement custom-3".to_string()),
                0b11111 => Err("TODO: Implement uhhhhhhhhhhhhhhh >=80b".to_string()),
                _ => unreachable!(),
            }
        }
        _ => {
            Err("TODO: Implement other ISAs".to_string())
        }
    }
}

pub fn decode_old(full_opcode: u32) -> Result<Instruction, String> {
    let optype = Instruction::opcode_type(full_opcode)?;
    let opcode = full_opcode&0x7F;

    match optype {
        OpType::R { rd, rs1, rs2, funct3, funct7 } => {
            match funct7 {
                0b0000_000 => {
                    match funct3 {
                        0b000 => Ok(Add { rd, rs1, rs2 }),
                        0b100 => Ok(Xor { rd, rs1, rs2 }),
                        0b110 => Ok(Or { rd, rs1, rs2 }),
                        0b111 => Ok(And { rd, rs1, rs2 }),
                        _ => Err(format!("Unknown I-ext R-type funct3: 0b{funct3:03b}"))
                    }
                }
                0b0100_000 => {
                    match funct3 {
                        0b000 => Ok(Sub { rd, rs1, rs2 }),
                        _ => Err(format!("Unknown neg I-ext R-type funct3: 0b{funct3:03b}"))
                    }
                }
                // M ext
                0b0000_001 => {
                    match funct3 {
                        0b000 => Ok(Mul { rd, rs1, rs2 }),
                        0b100 => Ok(Div { rd, rs1, rs2 }),
                        _ => Err(format!("Unknown M-ext R-type funct3: 0b{funct3:03b}"))
                    }
                }
                _ => Err(format!("Unknown 0b000 R-type funct7: 0b{funct7:07b}"))
            }
        }
        OpType::RW { .. } => {
            Err(format!("Unimplemented optype: {optype:?}"))
        }
        OpType::I { rd, rs1, funct3, imm } => {
            match opcode {
                0x13 | 0x1b => {
                    match funct3 {
                        0b000 => Ok(Addi {rd, rs1, imm}),
                        0b001 => {
                            let shtyp = (imm&0xFC0)>>6;
                            match shtyp {
                                0b0000_00 => Ok(Slli {rd, rs1, shamt: (imm & 0x1F) as u64 }),
                                _ => Err(format!("Unknown 0b001 shtyp type: 0b{shtyp:07b}"))
                            }
                        }
                        0b101 => {
                            let shtyp = (imm&0xFC0)>>6;
                            match shtyp {
                                0b0000_00 => Ok(Srli {rd, rs1, shamt: (imm & 0x1F) as u64 }),
                                _ => Err(format!("Unknown 0b101 shtyp type: 0b{shtyp:07b}"))
                            }
                        }
                        0b110 => Ok(Ori { rd, rs1, imm }),
                        0b111 => Ok(Andi { rd, rs1, imm }),
                        _ => Err(format!("Unknown immediate type funct3: 0b{funct3:03b}"))
                    }
                }
                0x0F => {
                    match funct3 {
                        0b001 => {
                            Ok(FenceI)
                        }
                        _ => Err(format!("Unknown fence type funct3: 0b{funct3:03b}"))
                    }
                }
                _ => Err(format!("Unknown opcode: 0b{opcode:07b} (0x{opcode:02X})")),
            }
        }
        OpType::Load { rd, rs1, funct3, imm } => {
            match funct3 {
                0b011 => {
                    Ok(Ld {rd, rs1, imm})
                }
                _ => Err(format!("Unknown load type funct3: 0b{funct3:03b}"))
            }
        }
        OpType::Jalr { rd, rs1, funct3, imm } => {
            match funct3 {
                0b000 => {
                    Ok(Jalr {rd, rs1, imm})
                }
                _ => Err(format!("Unknown jalr type funct3: 0b{funct3:03b}"))
            }
        }
        OpType::Csr { rd, rs1, funct3, csr } => {
            match funct3 {
                0b000 => {
                    let sys = usize::from(csr);
                    match sys {
                        0b00000_00_00000 => Ok(Ecall),
                        0b00000_00_00010 => Ok(Uret),
                        0b00010_00_00010 => Ok(Sret),
                        0b00010_00_00101 => Ok(Wfi),
                        0b00110_00_00010 => Ok(Mret),
                        _ => Err(format!("Unknown system sys: 0b{sys:012b}")),
                    }
                },
                0b001 => Ok(Csrrw { rd, rs1, csr }),
                0b010 => Ok(Csrrs { rd, rs1, csr }),
                0b011 => Ok(Csrrc { rd, rs1, csr }),
                0b101 => Ok(Csrrwi { rd, csr, imm: rs1 as usize as i64}),
                0b110 => Ok(Csrrsi { rd, csr, imm: rs1 as usize as i64}),
                0b111 => Ok(Csrrci { rd, csr, imm: rs1 as usize as i64}),
                _ => Err(format!("Unknown csr type funct3: 0b{funct3:03b}"))
            }
        }
        OpType::IW { rd, rs1, funct3, imm } => {
            match funct3 {
                0b000 => Ok(Addiw {rd, rs1, imm}),
                _ => Err(format!("Unknown immediate64 type funct3: 0b{funct3:03b}"))
            }
        }
        OpType::S { rs1, rs2, funct3, imm } => {
            match funct3 {
                0b011 => Ok(Sd { rs1, rs2, imm, }),
                _ => Err(format!("Unknown S-type funct3: 0b{funct3:03b}"))
            }
        }
        OpType::B { rs1, rs2, funct3, imm } => {
            match funct3 {
                0b000 => Ok(Beq{ rs1, rs2, imm }),
                0b001 => Ok(Bne{ rs1, rs2, imm }),
                0b100 => Ok(Blt{ rs1, rs2, imm }),
                0b101 => Ok(Bge{ rs1, rs2, imm }),
                0b110 => Ok(Bltu{ rs1, rs2, imm }),
                0b111 => Ok(Bgeu{ rs1, rs2, imm }),
                _ => Err(format!("Unknown B-type funct3: 0b{funct3:03b}"))
            }
        }
        OpType::U { .. } => {
            Err(format!("Unimplemented optype: {optype:?}"))
        }
        OpType::Auipc { rd, imm } => {
            Ok(Auipc {rd, imm})
        }
        OpType::Lui { rd, imm } => {
            Ok(Lui {rd, uimm: imm })
        }
        OpType::J { .. } => {
            Err(format!("Unimplemented optype: {optype:?}"))
        }
        OpType::Jal { rd, imm } => {
            Ok(Jal {rd, imm})
        }
    }
}