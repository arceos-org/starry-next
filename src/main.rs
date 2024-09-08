#![no_std]
#![no_main]

#[macro_use]
extern crate log;
extern crate alloc;
#[macro_use]
extern crate axstd as std;

mod loader;
mod mm;
mod syscall;
mod task;
use alloc::sync::Arc;

use std::thread;
use std::time::Duration;

use axhal::arch::UspaceContext;
use axsync::Mutex;

const USER_STACK_SIZE: usize = 0x10000;
const KERNEL_STACK_SIZE: usize = 0x40000; // 256 KiB

#[no_mangle]
fn main() {
    loader::list_apps();
    let testcases = option_env!("AX_TESTCASES_LIST")
        .unwrap_or_else(|| "Please specify the testcases list by making user_apps")
        .split(',')
        .filter(|&x| !x.is_empty());
    for testcase in testcases {
        let (entry_vaddr, ustack_top, uspace) = mm::load_user_app(testcase).unwrap();
        let user_task = task::spawn_user_task(
            Arc::new(Mutex::new(uspace)),
            UspaceContext::new(entry_vaddr.into(), ustack_top, 2333),
        );
        let exit_code = user_task.join();
        info!("User task {} exited with code: {:?}", testcase, exit_code);
    }

    loop {
        println!("Starry tick...");
        thread::sleep(Duration::from_secs(1));
    }
}
