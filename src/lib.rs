//! A simple crate that returns information about processes on OS X.
//!
//! In particular this can return the parent pid of a process, the name
//! of a process or the loaded image.
use std::ffi::{CStr, OsStr};
use std::io;
use std::os::unix::ffi::OsStrExt;
use std::path::PathBuf;

const PROC_PIDPATHINFO_MAXSIZE: usize = 4096;

extern "C" {
    fn macprocinfo_getppid(pid: i32) -> i32;
    fn macprocinfo_getpidpath(pid: i32, buf: *mut i8, bufsize: u32) -> i32;
    fn macprocinfo_getpidname(pid: i32, buf: *mut i8, bufsize: u32) -> i32;
}

/// Returns the parent PID of a process.
///
/// Given a process ID this returns the process ID of the parent.  This
/// returns `None` if the process does not exist.
pub fn get_parent_pid(id: u32) -> Option<u32> {
    unsafe {
        let parent_id = macprocinfo_getppid(id as i32);
        if parent_id == 0 {
            None
        } else {
            Some(parent_id as u32)
        }
    }
}

/// Returns the path to the process.
///
/// This effectively returns the loaded executable for a given process.
pub fn get_process_path(id: u32) -> io::Result<PathBuf> {
    let mut buf = [0i8; PROC_PIDPATHINFO_MAXSIZE];
    unsafe {
        if macprocinfo_getpidpath(id as i32, buf.as_mut_ptr(), PROC_PIDPATHINFO_MAXSIZE as u32) != 0
        {
            Ok(PathBuf::from(OsStr::from_bytes(
                CStr::from_ptr(buf.as_ptr()).to_bytes(),
            )))
        } else {
            Err(io::Error::last_os_error())
        }
    }
}

/// Returns the name to the process.
pub fn get_process_name(id: u32) -> io::Result<String> {
    let mut buf = [0i8; PROC_PIDPATHINFO_MAXSIZE];
    unsafe {
        if macprocinfo_getpidname(id as i32, buf.as_mut_ptr(), PROC_PIDPATHINFO_MAXSIZE as u32) != 0
        {
            Ok(String::from_utf8_lossy(CStr::from_ptr(buf.as_ptr()).to_bytes()).to_string())
        } else {
            Err(io::Error::last_os_error())
        }
    }
}
