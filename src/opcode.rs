use crate::opcode::Opcodes::*;

pub enum Opcodes {
    Lui,
    Unknown(u8),
}

impl TryFrom<Opcodes> for u8 {
    type Error = ();

    fn try_from(value: Opcodes) -> Result<Self, Self::Error> {
        Ok(match value {
            Lui => 0x37,
            Unknown(u) => u,
        })
    }
}

impl TryFrom<u8> for Opcodes {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Ok(match value {
            0x37 => Lui,
            _ => Unknown(value),
        })
    }
}