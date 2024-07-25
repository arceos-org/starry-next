#![no_std]
#![no_main]

#[macro_use]
extern crate log;
extern crate alloc;
extern crate axstd;

use axhal::arch::UspaceContext;
use axhal::mem::virt_to_phys;
use axhal::paging::MappingFlags;
use memory_addr::VirtAddr;

const USER_STACK_SIZE: usize = 4096;

fn app_main(arg0: usize) {
    unsafe {
        core::arch::asm!(
            "2:",
            "int3",
            "mov rax, r12",
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
fn main() -> ! {
    let entry = VirtAddr::from(app_main as usize);
    let entry_paddr_align = virt_to_phys(entry.align_down_4k());
    let entry_vaddr_align = VirtAddr::from(0x1000);
    let entry_vaddr = entry_vaddr_align + entry.align_offset_4k();

    let layout = core::alloc::Layout::from_size_align(USER_STACK_SIZE, 4096).unwrap();
    let ustack = unsafe { alloc::alloc::alloc(layout) };
    let ustack_paddr = virt_to_phys(VirtAddr::from(ustack as _));

    let kstack_top: usize;
    unsafe { core::arch::asm!("mov {}, rsp", out(reg) kstack_top) };
    let kstack_top = VirtAddr::align_down(kstack_top.into(), 16usize);

    let mut uspace = axmm::new_user_aspace().unwrap();
    let ustack_top = uspace.end();
    let ustack_vaddr = ustack_top - USER_STACK_SIZE;
    uspace
        .map_linear(
            entry_vaddr_align,
            entry_paddr_align,
            4096,
            MappingFlags::READ | MappingFlags::EXECUTE | MappingFlags::USER,
        )
        .unwrap();
    uspace
        .map_linear(
            ustack_vaddr,
            ustack_paddr,
            4096,
            MappingFlags::READ | MappingFlags::WRITE | MappingFlags::USER,
        )
        .unwrap();

    info!("New user address space: {:#x?}", uspace);
    let ctx = UspaceContext::new(entry_vaddr.into(), ustack_top, 2333);
    info!(
        "Enter user space: entry={:#x}, ustack={:#x}, kstack={:#x}",
        entry_vaddr, ustack_top, kstack_top,
    );
    unsafe {
        axhal::arch::write_page_table_root(uspace.page_table_root());
        ctx.enter_uspace(kstack_top)
    }
}
