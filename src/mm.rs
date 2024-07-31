use axerrno::AxResult;
use memory_addr::VirtAddr;

use axhal::mem::virt_to_phys;
use axhal::paging::MappingFlags;
use axhal::trap::{register_trap_handler, PAGE_FAULT};
use axmm::AddrSpace;
use axtask::TaskExtRef;

pub fn load_user_app(entry_fn: fn(usize)) -> AxResult<(VirtAddr, VirtAddr, AddrSpace)> {
    let entry_fn_kvaddr = VirtAddr::from(entry_fn as usize);
    let load_vaddr = VirtAddr::from(0x1000);
    let load_paddr = virt_to_phys(entry_fn_kvaddr.align_down_4k());
    let entry_vaddr = load_vaddr + entry_fn_kvaddr.align_offset_4k();

    let mut uspace = axmm::new_user_aspace()?;
    let ustack_top = uspace.end();
    let ustack_vaddr = ustack_top - crate::USER_STACK_SIZE;
    uspace.map_linear(
        load_vaddr,
        load_paddr,
        4096,
        MappingFlags::READ | MappingFlags::EXECUTE | MappingFlags::USER,
    )?;
    uspace.map_alloc(
        ustack_vaddr,
        crate::USER_STACK_SIZE,
        MappingFlags::READ | MappingFlags::WRITE | MappingFlags::USER,
        false,
    )?;
    info!("New user address space: {:#x?}", uspace);
    Ok((entry_vaddr, ustack_top, uspace))
}

#[register_trap_handler(PAGE_FAULT)]
fn handle_page_fault(vaddr: VirtAddr, access_flags: MappingFlags, is_user: bool) -> bool {
    if is_user {
        if !axtask::current()
            .task_ext()
            .aspace
            .lock()
            .handle_page_fault(vaddr, access_flags)
        {
            warn!("{}: segmentation fault, exit!", axtask::current().id_name());
            axtask::exit(-1);
        }
        true
    } else {
        false
    }
}
