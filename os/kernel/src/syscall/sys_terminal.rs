/* ╔═════════════════════════════════════════════════════════════════════════╗
   ║ Module: sys_terminal                                                    ║
   ╟─────────────────────────────────────────────────────────────────────────╢
   ║ Descr.: All system calls for terminal.                                  ║
   ╟─────────────────────────────────────────────────────────────────────────╢
   ║ Author: Fabian Ruhland, 30.8.2024, HHU                                  ║
   ╚═════════════════════════════════════════════════════════════════════════╝
*/
use core::ptr::slice_from_raw_parts;
use core::str::from_utf8;
use crate::{scheduler, terminal};
use crate::capabilities::{Capability, CapabilityFlags, CapabilityType};

fn check_caps(){
    let cur_thread = scheduler().current_thread();
    let cap = Capability::new(CapabilityType::SysTerminal, CapabilityFlags::empty());

    if !cur_thread.validate(cap){
        panic!("Current thread does not have the required Capability to access concurrency syscalls!");
    }
}

pub fn sys_terminal_read() -> isize {
    check_caps();

    let terminal = terminal();
    match terminal.read_byte() {
        -1 => panic!("Input stream closed!"),
        c => c as isize
    }
}

pub fn sys_terminal_write(buffer: *const u8, length: usize) -> isize {
    check_caps();

    let string = from_utf8(unsafe { slice_from_raw_parts(buffer, length).as_ref().unwrap() }).unwrap();
    let terminal = terminal();
    terminal.write_str(string);
    0
}
