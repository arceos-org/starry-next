#![no_std]
#![no_main]

#[macro_use]
extern crate log;
extern crate alloc;
extern crate axstd;

mod mm;
mod syscall;
mod task;

use alloc::sync::Arc;

use axhal::arch::UspaceContext;
use axsync::Mutex;

const USER_STACK_SIZE: usize = 4096;
const KERNEL_STACK_SIZE: usize = 0x40000; // 256 KiB

fn app_main(arg0: usize) {
    unsafe {
        core::arch::asm!(
            "2:",
            // "int3",
            "mov rax, r12",
            "push rax",
            "syscall",
            "add r12, 1",
            "jmp 2b",
            in("r12") arg0,
            in("rdi") 2,
            in("rsi") 3,
            in("rdx") 3,
            options(nostack, nomem)
        )
    }
}

#[no_mangle]
fn main() {
    let (entry_vaddr, ustack_top, uspace) = mm::load_user_app(app_main).unwrap();
    let user_task = task::spawn_user_task(
        Arc::new(Mutex::new(uspace)),
        UspaceContext::new(entry_vaddr.into(), ustack_top, 2333),
    );

    let (entry_vaddr, ustack_top, uspace) = mm::load_user_app(app_main).unwrap();
    let user_task2 = task::spawn_user_task(
        Arc::new(Mutex::new(uspace)),
        UspaceContext::new(entry_vaddr.into(), ustack_top, 2333),
    );
    let exit_code = user_task.join();
    let exit_code2 = user_task2.join();
    info!("User task exited with code: {:?}", exit_code);
    info!("User task2 exited with code: {:?}", exit_code2);
}
