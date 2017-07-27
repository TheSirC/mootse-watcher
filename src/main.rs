#![feature(str_mut_extras)]

extern crate url;
extern crate toml;
extern crate scraper;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate derive_new;
extern crate hyper;
extern crate futures;
extern crate tokio_core;
extern crate hyper_tls;

pub mod watcher;

use std::io::{self, Write};
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use hyper::{Client, Method, Request};
use hyper::header::{Headers, ContentLength, ContentType};
use hyper::header::SetCookie;
use hyper_tls::HttpsConnector;
use futures::{Stream, Future};
use tokio_core::reactor::Core;
use scraper::{Html, Selector};
use url::form_urlencoded;
use std::str;

#[derive(Debug, Deserialize, new, Eq, PartialEq)]
pub struct Config {
    username: String,
    password: String,
    base_uri: String,
    login_uri: String,
    grade_uri: String,
}

/// Using a toml file to load from configuration file
/// containing the parameters for the session

fn credentials_login() -> Config {
    let f = File::open("Config.toml").expect("Could not open the configuration file!");
    let mut reader = BufReader::new(&f);
    let s = &mut String::new();
    reader.read_to_string(s);
    let decoded: Config = toml::from_str(&s).expect("Parsing the configuration file failed!");
    decoded
}

/// Retrieve a vector of numbers corresponding to the IDs of all the courses
fn retreive_all_courses_id(c: Config) {
    // Login in using credentials
    let mut core = Core::new().unwrap();
    let handle = &core.handle();
    let client = Client::configure().connector(HttpsConnector::new(4,&handle).unwrap()).build(&handle);
    let uri = format!("https://{}/{}", &c.base_uri, &c.login_uri).parse().unwrap();
    let mut req = Request::new(Method::Post, uri);
    let params = format!("username={}&password={}", &c.username, &c.password);
    req.set_body(params);
    let cookie: &Vec<String> = &Vec::new();
    let post_request = client.request(req).and_then(|res| {
        println!("POST : {}", res.status());
        if let Some(&SetCookie(ref content)) = res.headers().get() {
            let mut cookie = &content.clone();
        }
        res.body().concat2()
    });
    let result_from_post = core.run(post_request).unwrap();
    println!("Result : {}", str::from_utf8(&result_from_post).unwrap());

    // Getting the page with the marks
    let mut req = Request::new(Method::Get,
                       format!("https://{}/{}", &c.base_uri, &c.grade_uri).parse().unwrap());
    req.headers_mut().set(SetCookie(cookie.to_vec()));
    let get_request = client.request(req).and_then(|res| {
        println!("GET : {}", res.status());
        res.body().concat2()
    });
    let mut result_from_get = core.run(get_request).unwrap().to_vec();
    let html_page_content = str::from_utf8_mut(&mut result_from_get).unwrap();
    // Creating the parsed html page
    let html_page_content = Html::parse_document(&html_page_content);
    // Create the parser
    let selector = Selector::parse("overview-grade").expect("Initializing the parsing failed");

    // Parsing the html to find the table
    let grade_table = html_page_content.select(&selector)
        .collect::<Vec<_>>()
        .iter()
        .map(|&x| x.inner_html())
        .collect::<String>();
    println!(" Parsed : {:?}", selector);

}

fn main() {
    let c = credentials_login();
    retreive_all_courses_id(c);
    let coursesID : Vec<i32> = Vec::new(); // Array containing the courses ID
    // Obtaining all the courses
}
