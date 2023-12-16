use std::fmt::{Display, Formatter};
use crate::register::Register::*;

#[repr(usize)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Register {
    Zero = 0x00,
    ReturnAddress,
    StackPointer,
    GlobalPointer,
    ThreadPointer,
    Temp0,
    Temp1,
    Temp2,
    Saved0,
    Saved1,
    ArgumentRetval0,
    ArgumentRetval1,
    Argument2,
    Argument3,
    Argument4,
    Argument5,
    Argument6,
    Argument7,
    Saved2,
    Saved3,
    Saved4,
    Saved5,
    Saved6,
    Saved7,
    Saved8,
    Saved9,
    Saved10,
    Saved11,
    Temp3,
    Temp4,
    Temp5,
    Temp6,
}

impl Display for Register {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let _ = write!(f, "\x1B[32m");
        let retval = match self {
            Zero => write!(f, "zero"),
            ReturnAddress => write!(f, "ra"),
            StackPointer => write!(f, "sp"),
            GlobalPointer => write!(f, "gp"),
            ThreadPointer => write!(f, "tp"),
            Temp0 => write!(f, "t0"),
            Temp1 => write!(f, "t1"),
            Temp2 => write!(f, "t2"),
            Saved0 => write!(f, "s0"),
            Saved1 => write!(f, "s1"),
            ArgumentRetval0 => write!(f, "a0"),
            ArgumentRetval1 => write!(f, "a1"),
            Argument2 => write!(f, "a2"),
            Argument3 => write!(f, "a3"),
            Argument4 => write!(f, "a4"),
            Argument5 => write!(f, "a5"),
            Argument6 => write!(f, "a6"),
            Argument7 => write!(f, "a7"),
            Saved2 => write!(f, "s2"),
            Saved3 => write!(f, "s3"),
            Saved4 => write!(f, "s4"),
            Saved5 => write!(f, "s5"),
            Saved6 => write!(f, "s6"),
            Saved7 => write!(f, "s7"),
            Saved8 => write!(f, "s8"),
            Saved9 => write!(f, "s9"),
            Saved10 => write!(f, "s10"),
            Saved11 => write!(f, "s11"),
            Temp3 => write!(f, "t3"),
            Temp4 => write!(f, "t4"),
            Temp5 => write!(f, "t5"),
            Temp6 => write!(f, "t6"),
        };
        let _ = write!(f, "\x1B[0m/\x1B[32m");
        let _ = write!(f, "x{}", *self as usize);
        let _ = write!(f, "\x1B[0m");
        retval
    }
}

impl Register {
    pub fn from_rvc(num: u8) -> Self {
        if num <= 0b111 {
            Self::from(num as usize + 8)
        } else {
            Zero
        }
    }
}

impl From<Register> for usize {
    fn from(r: Register) -> Self {
        r as usize
    }
}

impl From<usize> for Register {
    fn from(num: usize) -> Self {
        match num {
            0 => Zero,
            1 => ReturnAddress,
            2 => StackPointer,
            3 => GlobalPointer,
            4 => ThreadPointer,
            5 => Temp0,
            6 => Temp1,
            7 => Temp2,
            8 => Saved0,
            9 => Saved1,
            10 => ArgumentRetval0,
            11 => ArgumentRetval1,
            12 => Argument2,
            13 => Argument3,
            14 => Argument4,
            15 => Argument5,
            16 => Argument6,
            17 => Argument7,
            18 => Saved2,
            19 => Saved3,
            20 => Saved4,
            21 => Saved5,
            22 => Saved6,
            23 => Saved7,
            24 => Saved8,
            25 => Saved9,
            26 => Saved10,
            27 => Saved11,
            28 => Temp3,
            29 => Temp4,
            30 => Temp5,
            31 => Temp6,
            _ => Zero,
        }
    }
}