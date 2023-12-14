use crate::opcode::Opcodes::*;

#[repr(u8)]
pub enum Opcodes {
    Lui = 0x37,
    Unknown(u8),
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