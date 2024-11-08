use std::ffi::CString;
use std::process;
use std::ptr;
use std::{thread, time::Duration};
use x11::xlib;

mod def;
use def::*;

pub struct Block {
    icon: &'static str,
    command: &'static str,
}

fn setup_x() -> (*mut x11::xlib::_XDisplay, u64) {
    let dpy = unsafe { xlib::XOpenDisplay(ptr::null()) };
    let root = unsafe { xlib::XDefaultRootWindow(dpy) };

    (dpy, root)
}

fn get_commands() -> Vec<String> {
    let mut outs = Vec::new();
    for block in BLOCKS {
        let cmd: Vec<&str> = block.command.split_whitespace().collect();
        if let Ok(out) = process::Command::new(cmd.first().unwrap())
            .args(cmd.into_iter().skip(1))
            .output()
        {
            outs.push(String::from(block.icon) + &String::from_utf8(out.stdout).unwrap().trim());
        } else {
            eprintln!("could not execute {}", block.command);
        }
    }
    outs
}

fn write_commands(cmds: Vec<String>, dpy: *mut x11::xlib::_XDisplay, root: u64) {
    let c_str = CString::new(cmds.join(DELIMITER)).unwrap();
    unsafe {
        xlib::XStoreName(dpy, root, c_str.as_ptr());
        xlib::XFlush(dpy);
    }
}

fn statusloop(dpy: *mut x11::xlib::_XDisplay, root: u64) {
    loop {
        let cmds = get_commands();
        write_commands(cmds, dpy, root);

        thread::sleep(Duration::from_secs(1));
    }
}

fn main() {
    let (dpy, root) = setup_x();
    statusloop(dpy, root);
}
