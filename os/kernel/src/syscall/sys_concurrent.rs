/* ╔═════════════════════════════════════════════════════════════════════════╗
   ║ Module: sys_concurrent                                                  ║
   ╟─────────────────────────────────────────────────────────────────────────╢
   ║ Descr.: All system calls related to processes and threads.              ║
   ╟─────────────────────────────────────────────────────────────────────────╢
   ║ Author: Fabian Ruhland, 30.8.2024, HHU                                  ║
   ╚═════════════════════════════════════════════════════════════════════════╝
*/
use alloc::vec::Vec;
use alloc::rc::Rc;
use core::cmp::PartialEq;
use core::ptr::slice_from_raw_parts;
use core::str::from_utf8;
use x86_64::VirtAddr;
use syscall::return_vals::Errno;
use crate::{initrd, panic, process_manager, scheduler};
use crate::capabilities::{Capability, CapabilityFlags, CapabilityType};
use crate::process::thread::Thread;

fn check_caps(){
    let cur_thread = scheduler().current_thread();
    let cap = Capability::new(CapabilityType::SysConcurrency, CapabilityFlags::empty());

    if !cur_thread.validate(cap){
        panic!("Current thread does not have the required Capability to access concurrency syscalls!");
    }
}

pub fn sys_process_id() -> isize {
    check_caps();

    process_manager().read().current_process().id() as isize
}

pub fn sys_process_exit() -> isize {
    check_caps();

    scheduler().current_thread().process().exit();
    scheduler().exit();
    0
}

pub fn sys_thread_create(kickoff_addr: u64, entry: fn()) -> isize {
    check_caps();

    let mut thread = Thread::new_user_thread(process_manager().read().current_process(), VirtAddr::new(kickoff_addr), entry);
    let id = thread.id();

    inherit_caps(&mut thread);

    scheduler().ready(thread);
    id as isize
}

pub fn sys_thread_id() -> isize {
    check_caps();

    scheduler().current_thread().id() as isize
}

pub fn sys_thread_switch() -> isize {
    check_caps();

    scheduler().switch_thread_no_interrupt();
    0
}

pub fn sys_thread_sleep(ms: usize) -> isize {
    check_caps();

    scheduler().sleep(ms);
    0
}

pub fn sys_thread_join(id: usize) -> isize {
    check_caps();

    scheduler().join(id);
    0
}

pub fn sys_thread_exit() -> isize {
    check_caps();

    scheduler().exit();
    0
}

pub fn sys_process_execute_binary(name_buffer: *const u8, name_length: usize, args: *const Vec<&str>) -> isize {
    check_caps();

    let app_name = from_utf8(unsafe { slice_from_raw_parts(name_buffer, name_length).as_ref().unwrap() }).unwrap();
    match initrd().entries().find(|entry| entry.filename().as_str().unwrap() == app_name) {
        Some(app) => {
            let mut thread = Thread::load_application(app.data(), app_name, unsafe { args.as_ref().unwrap() });

            //inherit capabilities
            inherit_caps(&mut thread);

            scheduler().ready(Rc::clone(&thread));
            thread.id() as isize
        }
        None => Errno::ENOENT.into(),
    }
}

fn inherit_caps(mut thread: &mut Rc<Thread>) {
    if let Some(thread) = Rc::get_mut(&mut thread) {
        let caps = scheduler().current_thread().get_capabilities().clone();
        for cap in caps {
            //only inherit capabilities where 'Exposed' is set and 'Primitive' isn't
            print!("cap: {:?}", cap);
            if (cap.get_flags() & (CapabilityFlags::Primitive | CapabilityFlags::Expose)) == CapabilityFlags::Expose {
                print!(" exposed");
                if cap.get_flags().contains(CapabilityFlags::Once) {
                    print!(" once");
                    // if flag "Once" is set remove 'Expose' flag //TODO: Also remove once flag??
                    thread.add_capability(Capability::new(cap.get_type(), cap.get_flags() & CapabilityFlags::Expose.complement()));
                } else {
                    thread.add_capability(cap);
                }
            }
            println!("");
        }
    }
}
