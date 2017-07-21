use std::{thread, time};
use std::env;

mod maildirqueue;
mod jsonsender;

use maildirqueue::MaildirQueue;
use jsonsender::JsonSender;

fn main() {
    let mut base_dir = "".to_string();
    let mut num_threads = 3;
    let args: Vec<_> = env::args().collect();
    if args.len() > 1 {
        println!("The first argument is {}", args[1]);
        let last = args.len();
        for x in 1..last {
            println!("{}", x);
            if x % 2 == 0 { continue; }    
            let first = args[x].to_string();
            let second = args[x+1].to_string();
            match &*first {
                "-threads" => num_threads = second.parse::<u32>().unwrap(),
                "-baseDir" => {  base_dir.push_str(&*second); println!("{}", second); },
                &_ => println!("wrong parameters use -threads 3 -baseDir queuedir"),
            }
        }
    }
   
    if base_dir.len() == 0 {
        base_dir.push_str(".");
    }
    println!("{}", num_threads);
    println!("{}", base_dir);
    let mut children = vec![];
    for _ in 0..num_threads {
        println!("start thread");
        let working_dir = base_dir.clone().to_owned();
        children.push(thread::spawn(move || {
        startServer(working_dir.as_str()); 
        }));
    }
    
    println!("main thread");
    for child in children {
       // Wait for the thread to finish. Returns a result.
       let _ = child.join();
    }
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
