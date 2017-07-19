use std::{thread, time};
use std::env;

mod maildirqueue;
mod jsonsender;

use maildirqueue::MaildirQueue;
use jsonsender::JsonSender;

fn main() {
    let sender = Box::new(JsonSender::new());
    let mail_dir_que = Box::new(MaildirQueue::new(".".to_string()));
    if let Some(ref mail_dir_que) = mail_dir_que.init() {        
    let json = r#"{"url": "https://cdn.optimizely.com/json/8395320081.json"}"#;
        let mut is_client:bool = false;
        let args: Vec<_> = env::args().collect();
        if args.len() > 1 {
           println!("The first argument is {}", args[1]);
           is_client = true;
        }

        if is_client {
            let mut count = 0;
            loop {
                mail_dir_que.push(&json);
                println!("Pushed");
                count += 1;
                if count == 1000 {
                    break
                }
            }
        }
        else {
            let closure = |content:&str|-> bool { 
                println!("{:?}", content); 
                if let Err(err_str) = sender.sendJson(&json) {
                    println!("{}", err_str);
                    println!("Problem with sending json, requeuing");
                    mail_dir_que.push(&json); 
                }
                return true; 
            };
            let mut count = 0;
            loop {
                while mail_dir_que.pop(&closure) {
                    println!("Popped");
                }   
                count += 1;
                let ten_secs = time::Duration::from_secs(10);
                thread::sleep(ten_secs);
                if count > 1 {
                    break;
                }
            }
            println!("Decided to give up... no more in the queue");
        }
    }
}
