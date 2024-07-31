//! Loader for loading apps.
//!
//! It will read and parse ELF files.
//!
//! Now these apps are loaded into memory as a part of the kernel image.
use core::arch::global_asm;

global_asm!(include_str!(concat!(env!("OUT_DIR"), "/link_app.S")));

extern "C" {
    fn _app_count();
}

/// Get the number of apps.
pub(crate) fn get_app_count() -> usize {
    unsafe { (_app_count as *const u64).read() as usize }
}

/// Get the name of an app by a given app ID.
pub(crate) fn get_app_name(app_id: usize) -> &'static str {
    unsafe {
        let app_0_start_ptr = (_app_count as *const u64).add(1);
        assert!(app_id < get_app_count());
        let app_name = app_0_start_ptr.add(app_id * 2).read() as *const u8;
        let mut len = 0;
        while app_name.add(len).read() != b'\0' {
            len += 1;
        }
        let slice = core::slice::from_raw_parts(app_name, len);
        core::str::from_utf8(slice).unwrap()
    }
}

/// Get the data of an app by a given app ID.
pub(crate) fn get_app_data(app_id: usize) -> &'static [u8] {
    unsafe {
        let app_0_start_ptr = (_app_count as *const u64).add(1);
        assert!(app_id < get_app_count());
        let app_start = app_0_start_ptr.add(app_id * 2 + 1).read() as usize;
        let app_end = app_0_start_ptr.add(app_id * 2 + 2).read() as usize;
        let app_size = app_end - app_start;
        core::slice::from_raw_parts(app_start as *const u8, app_size)
    }
}

/// Get the data of an app by the given app name.
pub(crate) fn get_app_data_by_name(name: &str) -> Option<&'static [u8]> {
    let app_count = get_app_count();
    (0..app_count)
        .find(|&i| get_app_name(i) == name)
        .map(get_app_data)
}

/// List all apps.
pub(crate) fn list_apps() {
    info!("/**** APPS ****");
    let app_count = get_app_count();
    for i in 0..app_count {
        info!("{}", get_app_name(i));
    }
    info!("**************/");
}
