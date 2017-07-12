use std::{thread, time};
use std::env;

mod maildirqueue;
mod jsonsender;

use maildirqueue::MaildirQueue;
use jsonsender::JsonSender;

fn main() {
    let sender = JsonSender::new();
    let mailDirQue = MaildirQueue::new(".".to_string());
    if let Some(ref mailDirQue) = mailDirQue.init() {        
    let json = r#"{"url": "https://cdn.optimizely.com/json/8395320081.json"}"#;
        let mut isClient:bool = false;
        let args: Vec<_> = env::args().collect();
        if args.len() > 1 {
           println!("The first argument is {}", args[1]);
           isClient = true;
        }

        if isClient {
            let mut count = 0;
            loop {
                mailDirQue.push(&json);
                println!("Pushed");
                count += 1;
                if count == 1000 {
                    break
                }
            }
        }
        else {
            let closure = |content:&str|-> bool { println!("{:?}", content); sender.sendJson(&json); return true; };
            let mut count = 0;
            loop {
                while mailDirQue.pop(&closure) {
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
