use alloc::sync::Arc;
use core::ops::Deref;

use axhal::arch::UspaceContext;
use axmm::AddrSpace;
use axns::{AxNamespace, AxNamespaceIf, AxResource};
use axsync::Mutex;
use axtask::{AxTaskRef, TaskExtRef, TaskInner};

/// Task extended data for the monolithic kernel.
pub struct TaskExt {
    /// The process ID.
    pub proc_id: usize,
    /// The user space context.
    pub uctx: UspaceContext,
    /// The virtual memory address space.
    pub aspace: Arc<Mutex<AddrSpace>>,
    /// The resouece namespace.
    pub ns: AxNamespace,
}

impl TaskExt {
    pub fn new(uctx: UspaceContext, aspace: Arc<Mutex<AddrSpace>>) -> Self {
        Self {
            proc_id: 233,
            uctx,
            aspace,
            ns: AxNamespace::new_thread_local(),
        }
    }
}

axtask::def_task_ext!(TaskExt);

pub fn spawn_user_task(aspace: Arc<Mutex<AddrSpace>>, uctx: UspaceContext) -> AxTaskRef {
    let mut task = TaskInner::new(
        || {
            let curr = axtask::current();
            let kstack_top = curr.kernel_stack_top().unwrap();

            error!("{:?} {:#x}", *FOO, FOO.deref() as *const _ as usize);
            error!("{:?} {:#x}", *BAR, BAR.deref() as *const _ as usize);

            info!(
                "Enter user space: entry={:#x}, ustack={:#x}, kstack={:#x}",
                curr.task_ext().uctx.get_ip(),
                curr.task_ext().uctx.get_sp(),
                kstack_top,
            );
            unsafe { curr.task_ext().uctx.enter_uspace(kstack_top) };
        },
        "userboot".into(),
        crate::KERNEL_STACK_SIZE,
    );
    task.ctx_mut()
        .set_page_table_root(aspace.lock().page_table_root());
    task.init_task_ext(TaskExt::new(uctx, aspace));

    let new_ns = &task.task_ext().ns;
    FOO.deref_from(new_ns).init_new("sdfjsjdkf".into());
    BAR.deref_from(new_ns)
        .init_new(Mutex::new("deadbeef".into()));

    axtask::spawn_task(task)
}

struct AxNamespaceImpl;

#[crate_interface::impl_interface]
impl AxNamespaceIf for AxNamespaceImpl {
    #[inline(never)]
    fn current_namespace_base() -> *mut u8 {
        axtask::current().task_ext().ns.base()
    }
}

use alloc::string::String;

axns::def_resource! {
    pub static FOO: AxResource<String> = AxResource::new();
    pub static BAR: AxResource<Mutex<String>> = AxResource::new();
}
