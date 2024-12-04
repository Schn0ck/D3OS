/* ╔═════════════════════════════════════════════════════════════════════════╗
   ║ Module: capabilities                                                    ║
   ╟─────────────────────────────────────────────────────────────────────────╢
   ║ Descr.: Syscalls for Capability functions.                              ║
   ╟─────────────────────────────────────────────────────────────────────────╢
   ║ Author: Fabian Ruhland, Michael Schoettner, 31.8.2024, HHU              ║
   ╚═════════════════════════════════════════════════════════════════════════╝
*/

/*
pub struct Capability {
    cap_type: CapabilityType,
    cap_flags: CapabilityFlags
}
pub struct CapabilityFlags(usize);
pub enum CapabilityType {
    SysTerminal,
    SysVmem,
    SysConcurrency,
    SysTime
}
 */

pub enum CapabilityType {
    SysTerminal,
    SysVmem,
    SysConcurrency,
    SysTime,
    SysCapabilities
}

use concurrent::thread::Thread;
use syscall::{syscall, SystemCall};

pub fn debug_caps() {
    syscall(SystemCall::DebugCapabilities, &[]).expect("Failed to debug capabilities");
}

pub fn remove_caps(thread_id: usize, capability_type: CapabilityType) {
    let type_num: usize = match capability_type {
        CapabilityType::SysTerminal => 0,
        CapabilityType::SysVmem => 1,
        CapabilityType::SysConcurrency => 2,
        CapabilityType::SysTime => 3,
        CapabilityType::SysCapabilities => 4
    };

    syscall(SystemCall::RemoveCapabilities, &[thread_id, type_num]).expect("Failed to remove capability");
}

