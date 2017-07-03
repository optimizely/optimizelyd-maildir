use std::{thread, time};

mod maildirqueue;
mod jsonsender;

use maildirqueue::MaildirQueue;
use jsonsender::JsonSender;

fn main() {
    let sender = JsonSender::new();
    let mailDirQue = MaildirQueue::new(".".to_string());
    if let Some(ref mailDirQue) = mailDirQue.init() {
        let json = r#"{"url": "https://www.google.com/search", "requestBody": "q=bill+material&output=xml&client=test&site=operations&access=p"}"#;
        mailDirQue.push(&json);
        println!("Pushed");
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
