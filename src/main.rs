use std::{thread, time};
use std::env;

mod maildirqueue;
mod jsonsender;

use maildirqueue::MaildirQueue;
use jsonsender::JsonSender;

fn main() {
    let child1 = thread::spawn(move || {
       startServer("."); 
    });
    let child2 = thread::spawn(move || {
       startServer("."); 
    });

    startServer(".");
}

fn startServer(baseDir:&str) {
    let sender = Box::new(JsonSender::new());
    let mail_dir_que = Box::new(MaildirQueue::new(baseDir.to_string()));
    if let Some(ref mail_dir_que) = mail_dir_que.init() {        
        let closure = |content:&str|-> bool { 
            println!("{:?}", content); 
            if let Err(err_str) = sender.sendJson(&content, "./res/") {
                println!("{}", err_str);
                println!("Problem with sending json, requeuing");
                return false; 
            }
            return true; 
        };

        loop {
            while mail_dir_que.pop(&closure) {
                println!("Popped");
            }   
            let ten_secs = time::Duration::from_secs(10);
            thread::sleep(ten_secs);
        }
        println!("Decided to give up... no more in the queue");
    }
}
