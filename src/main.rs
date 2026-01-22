use std::env;
use std::ffi::{CString, CStr};

#[derive(Default)]
struct Flags {
    all: bool,
    long: bool
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let mut flags = Flags::default();
    let mut paths = Vec::new();

    for arg in args {
        if arg.starts_with("-") {
            for ch in arg.chars().skip(1){
                match ch {
                    'a' => flags.all = true,
                    'l' => flags.long = true,
                    _ => {
                        eprintln!("mini-ls: invalid option {}", ch);
                        return;
                    }
                }
            }
        }else {
            paths.push(arg);
        }
    }
    if paths.is_empty(){
        paths.push(".".to_string());
    }
    for path in paths {
        list_dir(&path, &flags);
    }
}

fn list_dir(path: &str, flags: &Flags) {
    let c_path = CString::new(path).expect("CString conversion failed");

    let fd = unsafe {
        libc::openat(libc::AT_FDCWD, c_path.as_ptr(), libc::O_RDONLY)
    };
    if fd < 0 {
        eprintln!("openat failed for {}: {}", path, std::io::Error::last_os_error());
        return;
    }

    let dir = unsafe { libc::fdopendir(fd) };
    if dir.is_null() {
        eprintln!("fdopendir failed: {}", std::io::Error::last_os_error());
        unsafe { libc::close(fd) };
        return;
    }
    println!("{}", path);

    loop {
        let entry = unsafe { libc::readdir(dir) };
        if entry.is_null() {
            break;
        }

        let name = unsafe {
            CStr::from_ptr((*entry).d_name.as_ptr())
        };
        let name = name.to_string_lossy();

        if !flags.all && (name == "." || name == "..") {
            continue;
        }
        
        println!("{}", name);
    }
    println!("\n");

    unsafe {
        libc::closedir(dir);
    }
}
