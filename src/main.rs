use std::env;
use std::ffi::{CString, CStr};

fn main() {
    let path = env::args().nth(1).unwrap_or_else(|| ".".to_string());
    let c_path = CString::new(path).expect("CString conversion failed");

    let fd = unsafe {
        libc::openat(
            libc::AT_FDCWD, 
            c_path.as_ptr(),
            libc::O_RDONLY,
        )
    };
    if fd < 0 {
        let err = std::io::Error::last_os_error();
        eprintln!("openat failed: {}", err);
        return;
    }
    let dir = unsafe {
        libc::fdopendir(fd)
    };
    if dir.is_null(){
        eprintln!("fdopendir failed: {}", std::io::Error::last_os_error());
        unsafe { libc::close(fd)};
        return;
    }
    loop {
        let entry = unsafe { libc::readdir(dir)};
        if entry.is_null() {
            break;
        }
        let name = unsafe {
            CStr::from_ptr((*entry).d_name.as_ptr())
        };
        println!("{}", name.to_string_lossy());
    }
    unsafe {
        libc::closedir(dir);
    }
}
