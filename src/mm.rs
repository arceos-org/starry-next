use axerrno::AxResult;
use memory_addr::VirtAddr;

use axhal::mem::phys_to_virt;
use axhal::paging::MappingFlags;
use axhal::trap::{register_trap_handler, PAGE_FAULT};
use axmm::AddrSpace;
use axtask::TaskExtRef;

use crate::loader;

pub fn load_user_app(app_name: &str) -> AxResult<(VirtAddr, VirtAddr, AddrSpace)> {
    let elf_info = loader::load_user_app(app_name);

    let mut uspace = axmm::new_user_aspace()?;
    for segement in elf_info.segments {
        debug!(
            "Mapping ELF segment: {:#x?} -> {:#x?} flags: {:#x?}",
            segement.start_vaddr,
            segement.start_vaddr + segement.size,
            segement.flags
        );
        uspace.map_alloc(segement.start_vaddr, segement.size, segement.flags, true)?;

        let (segement_start_paddr, _, _) = uspace
            .page_table()
            .query(segement.start_vaddr)
            .unwrap_or_else(|_| panic!("Mapping failed for segment: {:#x?}", segement.start_vaddr));

        // Copy data of the segment to the physical memory
        let segement_data = segement.data.as_slice();
        let segement_start_vaddr = phys_to_virt(segement_start_paddr).as_mut_ptr();
        let segement_data_ptr = segement_data.as_ptr();
        unsafe {
            core::ptr::copy_nonoverlapping(
                segement_data_ptr,
                segement_start_vaddr,
                segement_data.len(),
            );
        }
    }

    let ustack_top = uspace.end();
    let ustack_vaddr = ustack_top - crate::USER_STACK_SIZE;

    uspace.map_alloc(
        ustack_vaddr,
        crate::USER_STACK_SIZE,
        MappingFlags::READ | MappingFlags::WRITE | MappingFlags::USER,
        false,
    )?;
    info!("New user address space: {:#x?}", uspace);
    Ok((elf_info.entry, ustack_top, uspace))
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
