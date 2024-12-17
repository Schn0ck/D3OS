use alloc::rc::Rc;
use log::{error, warn};
use syscall::return_vals::{convert_syscall_result_to_ret_code, Errno, SyscallResult};
use crate::capabilities::{Capability, CapabilityFlags, CapabilityType};
use crate::process::thread::Thread;
use crate::scheduler;

fn check_caps(){
    let cur_thread = scheduler().current_thread();
    let cap = Capability::new(CapabilityType::SysCapabilities, CapabilityFlags::empty());

    if !cur_thread.validate(cap){
        panic!("Current thread does not have the required Capability to access concurrency syscalls!");
    }
}

pub fn sys_debug_caps() -> isize {
    let cur_thread = scheduler().current_thread();

    for cap in cur_thread.get_capabilities().iter() {
        println!("{:?}", cap);
    }

    0
}

pub fn sys_remove_caps(id: usize, cap_type_id: usize) -> isize {
    check_caps();

    let cap_type = match cap_type_id {
        0 => CapabilityType::SysTerminal,
        1 => CapabilityType::SysVmem,
        2 => CapabilityType::SysConcurrency,
        3 => CapabilityType::SysTime,
        4 => CapabilityType::SysCapabilities,
        _ => return convert_syscall_result_to_ret_code(Err(Errno::EINVAL))
    };

    println!("Removing capability {:?} from thread {}", cap_type, id);

    match scheduler().thread(id) {
        Some(ref thread) => {
            thread.as_ref().remove_capability(cap_type);
        }
        None => {return convert_syscall_result_to_ret_code(Err(Errno::EINVAL));}
    }

    0
}