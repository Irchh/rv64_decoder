use std::fmt::{Display, Formatter};
use crate::csr::CsrRegister::*;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum CsrRegister {
    // idk
    SScratch, // 0x140
    SepC, // 0x141

    // Trap setup
    SStatus, // 0x100
    MStatus, // 0x300
    MISA, // 0x301
    MEDeleg, // 0x302
    MIDeleg, // 0x303
    MIE, // 0x304
    MTVec, // 0x305
    MCounterEn, // 0x306
    //MStatusH, // 0x310 RV32 ONLY

    SIE, // idk
    MPIE, // idk
    SPIE, // idk

    // Trap handling
    MScratch, // 0x340
    MEPC, // 0x341
    MCause, // 0x342
    MTVal, // 0x343
    MIP, // 0x344
    MTInst, // 0x34A
    MTVal2, // 0x34B

    // Memory protection
    PMPCfg(u8), // 0x3A0 - 0x3AF
    PMPAddr(u8), // 0x3B0 - 0x3EF

    // General machine info
    MVendorID, // 0xF11
    MArchID, // 0xF12
    MImpID, // 0xF13
    MHartID, // 0xF14

    // Unknowns
    Other(u16),
    Invalid(u64),
}

impl CsrRegister {
    /// Returns a mask of the register bits.
    pub fn mask(&self) -> u64 {
        match self {
            Other(_) => 0xFFFFFFFFFFFFFFFF_u64,
            Invalid(_) => 0,
            _ => {
                0xFFFFFFFFFFFFFFFF_u64
            }
        }
    }

    /// Returns true if the register is writable.
    pub fn is_writable(&self) -> bool {
        match self {
            MVendorID | MArchID | MImpID | MHartID => false,
            Invalid(_) => false,
            _ => true,
        }
    }
}

impl From<CsrRegister> for usize {
    fn from(csr: CsrRegister) -> Self {
        match csr {
            SScratch => 0x140,
            SepC => 0x141,
            SStatus => 0x100,
            MStatus => 0x300,
            MISA => 0x301,
            MEDeleg => 0x302,
            MIDeleg => 0x303,
            MIE => 0x304,
            MTVec => 0x305,
            MCounterEn => 0x306,
            //MStatusH => 0x307, // RV32 ONLY

            // TODO: uuhhhhhhh
            MPIE => 0xFFFFFFFFFFFFFFFF,
            SPIE => 0xFFFFFFFFFFFFFFFF,
            SIE => 0xFFFFFFFFFFFFFFFF,

            MScratch => 0x340,
            MEPC => 0x341,
            MCause => 0x342,
            MTVal => 0x343,
            MIP => 0x344,
            MTInst => 0x34A,
            MTVal2 => 0x34B,
            PMPCfg(num) => num as usize + 0x3A0,
            PMPAddr(num) => num as usize + 0x3B0,

            MVendorID => 0xF11,
            MArchID => 0xF12,
            MImpID => 0xF13,
            MHartID => 0xF14,

            Other(n) => n as usize,
            Invalid(_) => (-1_i64) as usize,
        }
    }
}

impl From<u64> for CsrRegister {
    fn from(num: u64) -> Self {
        match num {
            0x140 => SScratch,
            0x141 => SepC,
            0x100 => SStatus,
            0x300 => MStatus,
            0x301 => MISA,
            0x302 => MEDeleg,
            0x303 => MIDeleg,
            0x304 => MIE,
            0x305 => MTVec,
            0x306 => MCounterEn,
            //0x307 => MStatusH, // RV32 ONLY

            0x340 => MScratch,
            0x341 => MEPC,
            0x342 => MCause,
            0x343 => MTVal,
            0x344 => MIP,
            0x34A => MTInst,
            0x34B => MTVal2,

            0xF11 => MVendorID,
            0xF12 => MArchID,
            0xF13 => MImpID,
            0xF14 => MHartID,

            0x3A0..=0x3AF => PMPCfg((num - 0x3A0) as u8),
            0x3B0..=0x3EF => PMPAddr((num - 0x3B0) as u8),

            _ => {
                if num < 4096 {
                    Other(num as u16)
                } else {
                    Invalid(num)
                }
            }
        }
    }
}

impl Display for CsrRegister {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let _ = write!(f, "\x1B[35m");
        let retval = match self {
            Other(reg) => write!(f, "UNKNOWN(0x{:0X})", reg),
            Invalid(reg) => write!(f, "INVALID(0x{:0X})", reg),
            _ => write!(f, "{}", format!("{:?}", self).to_lowercase()),
        };
        let _ = write!(f, "\x1B[0m");
        retval
    }
}