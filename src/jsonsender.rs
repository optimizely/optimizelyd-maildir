extern crate futures;
extern crate hyper;
extern crate tokio_core;
extern crate hyper_tls;
extern crate libc;
extern crate serde;
extern crate serde_json;
extern crate url;

use std::io::{self, Write};
use std::collections::HashMap;
use std::ffi::{CString, CStr};
use std::os::raw::c_char;

use jsonsender::hyper::Body;
use jsonsender::hyper::Chunk;
use jsonsender::futures::Stream;
use jsonsender::futures::Future;

use self::hyper::{Method, Request};
use self::hyper::header::{ContentLength, ContentType};
use self::hyper::client::Response;
use self::hyper::header::Headers;
use self::url::Url;
use self::hyper::Client;
use self::hyper::Uri;
use self::hyper_tls::HttpsConnector;
//use self::hyper_tls::HttpsConnector;
//use self::hyper::client::HttpConnector;
use self::tokio_core::reactor::Core;
use self::serde_json::{Value, Error};

pub struct JsonSender {

}

impl JsonSender {
    pub fn new() -> JsonSender {
        JsonSender {
        }
    }
    
    pub fn sendJson(&self, jsonString:&str) -> Result<bool, &str> {
    
        let mut core = Core::new().unwrap();
        let handle = core.handle();
        let client = Client::configure()
            .connector(HttpsConnector::new(4, &handle).unwrap())
            .build(&handle);
        let value:Value = serde_json::from_str(jsonString).unwrap();
        let uri =  format!("{}", value["url"]).parse();
        let mut req:Request = Request::new(Method::Post, uri.unwrap());
        if !value["requestBody"].is_null() {
            let body = format!("{}", value["requestBody"]);
            req.headers_mut().set(ContentType::json());
            req.headers_mut().set(ContentLength(body.len() as u64));
            req.set_body(body);
        }
        else {
            req.set_method(Method::Get);
        }
        println!("{:?}", req);
        let post = client.request(req);
        let res = core.run(post);
        match res {
            Ok(res) => {
                if res.status().is_success() {
                      println!("{:?}", res); 
                      println!("Getting body");
                      let bodyiter =  res.body().concat2().wait().unwrap();
                      io::stdout().write_all(&bodyiter);
                      return Ok(true);
                }
                else {
                        // retry
                        println!("Failed {:?}", res);
                        return Err("Request failed");
                }
            },
            Err(e) =>  {
                println!("{:?}", e); 
                return Err("Problem sending request");
            },
        };

    }
}
