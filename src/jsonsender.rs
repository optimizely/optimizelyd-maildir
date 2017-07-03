extern crate hyper;
extern crate hyper_tls;
extern crate tokio_core;
extern crate libc;
extern crate serde;
extern crate serde_json;
extern crate url;

use std::io::Write;
use std::collections::HashMap;
use std::ffi::{CString, CStr};
use std::os::raw::c_char;

use self::hyper::{Method, Request};
use self::hyper::header::{ContentLength, ContentType};
use self::hyper::client::Response;
use self::hyper::header::Headers;
use self::url::Url;
use self::hyper::Client;
use self::hyper::Uri;
use self::hyper_tls::HttpsConnector;
use self::hyper::client::HttpConnector;
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
        let body = format!("{}", value["requestBody"]);
        let mut req = Request::new(Method::Post, uri.unwrap());
        req.headers_mut().set(ContentType::json());
        req.headers_mut().set(ContentLength(body.len() as u64));
        req.set_body(body);
        let post = client.request(req);
        let res = core.run(post);
        match res {
            Ok(res) => return Ok(true),
            Err(e) =>  return Err("Problem sending request"),
        };

    }
}
