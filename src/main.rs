use std::ptr;
use std::fs;
use std::io::{Write, SeekFrom, Seek};
use std::os::unix::prelude::AsRawFd;
use mmap::{MemoryMap, MapOption};

mod maildirqueue;

use maildirqueue::MaildirQueue;

// from crates.io
extern crate mmap;
extern crate libc;

fn main() {
    let mailDirQue = MaildirQueue::new(".".to_string());
    if let Some(ref mailDirQue) = mailDirQue.init() {
        mailDirQue.push("This is example file");
        println!("Pushed");
        let closure = |content:&str|-> bool { println!("{:?}", content); return true; };

        while mailDirQue.pop(&closure) {
            println!("Popped");
        }
    }
}
