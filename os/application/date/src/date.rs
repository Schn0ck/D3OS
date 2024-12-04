#![no_std]

extern crate alloc;

use capabilities::capabilities::{debug_caps, remove_caps, CapabilityType};
#[allow(unused_imports)]
use runtime::*;
use terminal::{print, println};
use time::date;
use concurrent::*;
use concurrent::thread::Thread;

#[unsafe(no_mangle)]
pub fn main() {
    let date = date();
    println!("{}", date.format("%Y-%m-%d %H:%M:%S"));
    debug_caps();

    //todo add caps??

    // Create a new thread
    if let Some(thread) = thread::create(|| { // create "startet" thread
        println!("Hello from second thread");
        println!("{}", time::date().format("%Y-%m-%d %H:%M:%S"));
    }) {
        remove_caps(thread.id(), CapabilityType::SysTerminal);
        thread.join();
    }


}