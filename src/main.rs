#![feature(str_mut_extras)]

extern crate url;
extern crate toml;
extern crate scraper;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate derive_new;
extern crate reqwest;

pub mod watcher;

use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::thread;
use std::sync::mpsc::channel;
use reqwest::header::{Cookie, Headers, SetCookie};
use scraper::{Html, Selector};
use std::str;
use watcher::*;

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

/// Retrieve the SetCookie for the rest of the HTTP transactions

fn request_sequence(c: Config) -> String {
    // Login in using credentials
    let client = reqwest::Client::new().expect("Initialization of the client failed");
    let uri: String = format!("https://{}/{}", &c.base_uri, &c.login_uri)
        .parse()
        .unwrap();
    let params = [("username", &c.username), ("password", &c.password)];
    let post_request = client
        .post(&uri)
        .expect("POST failed")
        .form(&params)
        .expect("Wraping of the forms parameters failed")
        .send()
        .expect("POST request failed to be sent"); 
    let uri_grade: String = format!("https://{}/{}", &c.base_uri, &c.grade_uri)
        .parse()
        .unwrap();
    let mut setcookie: Vec<String> = Vec::new();
    if let Some(&SetCookie(ref content)) = post_request.headers().get::<SetCookie>() {
        setcookie = content.clone();
    }
    let mut headers = Headers::new();
    headers.set(SetCookie(setcookie));
    let mut html_page_content = String::new();
    client
        .get(&uri_grade)
        .expect("GET request failed")
        .headers(headers)
        .send()
        .expect("GET request failed to be sent")
        .read_to_string(&mut html_page_content);

    html_page_content
}

/// Retrieve a vector of numbers corresponding to the IDs of all the courses

fn retreive_all_courses_id(c: Config) {
    let html_page_content = request_sequence(c);

    // Creating the parsed html page
    let html_page_content = Html::parse_document(&html_page_content);
    // Create the parser
    let selector = Selector::parse("overview-grade").expect("Initializing the parsing failed");
    // Parsing the html to find the table
    let grade_table = html_page_content
        .select(&selector)
        .collect::<Vec<_>>()
        .iter()
        .map(|&x| x.inner_html())
        .collect::<String>();
    println!("Table : {:?}", grade_table);
}

fn main() {
    let c = credentials_login();
    // Obtaining all the courses
    retreive_all_courses_id(c);
    // let coursesID: Vec<i32> = retreive_all_courses_id(c); // Array containing the courses ID

    // for nc in coursesID {
    // Spawn workers for each one of them
    // let worker = thread::spawn(move || {
    // watcher::watcher::new(nc, None, Some(3600));
    // watcher::watcher.run();
    // });
    // }
    // Catch all the threads in case of crashing
    // let result = worker.join();
}
