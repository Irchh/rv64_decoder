use crate::instruction::Instruction;
use crate::instruction::Instruction::*;
use crate::optype::OpType;

pub fn decode(full_opcode: u32) -> Result<Instruction, String> {
    let optype = Instruction::opcode_type(full_opcode)?;
    let opcode = full_opcode&0x7F;
    match optype {
        OpType::R { rd, rs1, rs2, funct3, funct7 } => {
            match funct7 {
                0b0000_000 => {
                    match funct3 {
                        0b000 => Ok(Add { rd, rs1, rs2 }),
                        0b100 => Ok(XOr { rd, rs1, rs2 }),
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
                                0b0000_00 => Ok(Slli {rd, rs1, shamt: imm & 0x1F}),
                                _ => Err(format!("Unknown 0b001 shtyp type: 0b{shtyp:07b}"))
                            }
                        }
                        0b101 => {
                            let shtyp = (imm&0xFC0)>>6;
                            match shtyp {
                                0b0000_00 => Ok(Srli {rd, rs1, shamt: imm & 0x1F}),
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