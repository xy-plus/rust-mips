//! MIPS CP0 Cause register

use core::convert::TryFrom;
use numeric_enum_macro::numeric_enum;

#[derive(Clone, Copy, Debug)]
pub struct Cause {
    pub bits: u32,
}

register_rw!(13, 0);
register_struct_rw!(Cause);
register_set_reset_bit!(set_soft_int0, reset_soft_int0, 8);

register_set_reset_bit!(set_soft_int1, reset_soft_int1, 9);

register_set_reset_bit!(set_iv, reset_iv, 23);

numeric_enum! {
    #[repr(u32)]
    #[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
    pub enum Exception {
        Interrupt = 0,
        TLBModification = 1,
        TLBLoadMiss = 2,
        TLBStoreMiss = 3,
        AddressLoadError = 4,
        AddressStoreError = 5,
        BusLoadError = 6,
        BusStoreError = 7,
        Syscall = 8,
        Breakpoint = 9,
        ReservedInstruction = 10,
        CoprocessorUnusable = 11,
        Overflow = 12,
        TrapException = 13,
        MSAFloatingPoint = 14,
        FloatingPoint = 15,
        Reserved1 = 16,
        Reserved2 = 17,
        Coprocessor2 = 18,
        TLBReadInhibit = 19,
        TLBExecutionInhibit = 20,
        MSADisabled = 21,
        MDMX = 22,
        Watch = 23,
        MachineCheck = 24,
        ThreadException = 25,
        DSPDisabled = 26,
        VirtualizedGuest = 27,
        Reserved3 = 28,
        Reserved4 = 29,
        CacheError = 30,
        Reserved5 = 31,
        Unknown = 32,
    }
}

#[derive(Clone, Copy, Debug)]
pub enum SoftwareInterrupt {
    SoftInt0,
    SoftInt1,
}

impl Cause {
    #[inline]
    pub fn cause(&self) -> Exception {
        // exc_code = cause_reg[6..2]
        Exception::try_from((self.bits >> 2) & 0x1f).unwrap_or(Exception::Unknown)
    }

    #[inline]
    pub fn pending_interrupt(&self) -> u32 {
        // IP = cause_reg[15..10, 9..8]
        let soft_int = (self.bits >> 8) & 0b11;
        let hard_int = (self.bits >> 10) & 0b111_111;
        soft_int | (hard_int << 2)
    }

    #[inline]
    pub fn hard_int5(&self) -> bool {
        ((self.bits >> 14) & 1) == 1
    }
}
