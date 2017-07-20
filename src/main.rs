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
    let json = r#"{"url": "https://p13nlog.dz.optimizely.com/log/event", "requestBody": {"visitorId": "", "clientVersion": "1.0.0", "clientEngine": "python-sdk", "userFeatures": [], "projectId": "8395320081", "isGlobalHoldback": false, "eventEntityId": "8398160520", "eventName": "gotvariation", "eventFeatures": [], "eventMetrics": [], "timestamp": 1500509066875, "layerStates": [{"decision": {"variationId": "8398471060", "isLayerHoldback": false, "experimentId": "8391751517"}, "actionTriggered": true, "layerId": "8394600779"}], "accountId": "8362480420"}}"#;
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
                if let Err(err_str) = sender.sendJson(&content) {
                    println!("{}", err_str);
                    println!("Problem with sending json, requeuing");
                    return false; 
                }
                // we always return true here which means remove the file.
                // that means that we keep track of it and when to move it to doa
                // false will cause it to be requeued.
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
                    //break;
                }
            }
            println!("Decided to give up... no more in the queue");
        }
    }
}
