mod optype;

use crate::compressed::optype::COpType;
use crate::Instruction;

fn decode_addi4spn(full_opcode: u16) -> Result<Instruction, String> {
    todo!()
}

fn decode_ld(full_opcode: u16) -> Result<Instruction, String> {
    match COpType::new_cl(full_opcode) {
        COpType::CL { rd, rs1, funct3 } => {
            let imm5_3 = full_opcode >> 7 & 0b00111000;
            let imm7_6 = full_opcode << 1 & 0b11000000;
            let imm = (imm7_6 | imm5_3) as u64 as i64;
            Ok(Instruction::Ld { rd, rs1, imm, })
        }
        _ => unreachable!()
    }
}

// TODO: Should this be pub(crate)?
pub(crate) fn decode_compressed(full_opcode: u16) -> Result<Instruction, String> {
    let funct3 = (full_opcode & 0xE000) >> 11;
    match funct3 | (full_opcode & 0b11) {
        0b00000 => decode_addi4spn(full_opcode),
        0b01100 => decode_ld(full_opcode),
        op => Err(format!("TODO: Implement compressed opcode 0b{op:05b}"))
    }
}