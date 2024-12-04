use alloc::string::String;
use alloc::vec::Vec;
use core::cmp::PartialEq;
use core::ffi::FromBytesUntilNulError;
use bitflags::bitflags;

// Todo: Feingranularer Zugriff bei Speicher/Dateien(Read, Write)

/// Enum for the different types of Capabilities
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum CapabilityType {
    SysTerminal,
    SysVmem,
    SysConcurrency,
    SysTime,
    SysCapabilities
}

///Flags used for Inheritance of Capabilities to other threads
 #[derive(Debug, Clone, Copy, PartialEq)]
pub struct CapabilityFlags(usize);

bitflags! {
    impl CapabilityFlags: usize {
        const Primitive = 1 << 0; // Only receive in boot process (no inheritance);
        const Expose = 1 << 1; // Can be inherited by child threads
        const Once = 1 << 2; // Can only be inherited once //todo: necessary?
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Capability {
    cap_type: CapabilityType,
    cap_flags: CapabilityFlags
}

impl PartialEq for Capability {
    fn eq(&self, other: &Self) -> bool {
        self.cap_type == other.cap_type
    }
}

impl Capability {
    pub fn new(cap_type: CapabilityType, cap_flags: CapabilityFlags) -> Capability {
        Capability {
            cap_type,
            cap_flags
        }
    }

    pub fn get_type(&self) -> CapabilityType {
        self.cap_type
    }

    pub fn get_flags(&self) -> CapabilityFlags {
        self.cap_flags
    }

    pub fn validate(&self, cap_type: CapabilityType) -> bool {
        self.cap_type == cap_type
    }
}