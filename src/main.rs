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
    mailDirQue.init();
}
