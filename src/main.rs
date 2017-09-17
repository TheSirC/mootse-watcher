#[macro_use]
extern crate derive_new;
extern crate hyper;
extern crate reqwest;
extern crate scraper;
#[macro_use]
extern crate serde_derive;
extern crate toml;
extern crate url;

pub mod watcher;

use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::thread;
use std::sync::mpsc::channel;
use std::collections::HashMap;
use reqwest::header::{Connection, ConnectionOption, Cookie, Headers, SetCookie};
use scraper::{Html, Selector};
use std::str;
// use watcher::*;

#[derive(Debug, Deserialize, new, Eq, PartialEq)]
pub struct Config {
    username: String,
    password: String,
    anchor: String,
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
    let mut headers = Headers::new();
    let uri_login: String = format!("https://{}/{}", &c.base_uri, &c.login_uri)
        .parse()
        .unwrap();
    headers.set(Connection::keep_alive());
    let mut params = HashMap::new();
    params.insert("username", &c.username);
    params.insert("password", &c.password);
    params.insert("anchor", &c.anchor);
    let post_request = client
        .post(&uri_login)
        .expect("POST failed")
        .headers(headers)
        .form(&params)
        .expect("Wraping of the forms parameters failed")
        .send()
        .expect("POST request failed to be sent");
    let mut headers = Headers::new();
    let mut cookie = Cookie::new();
    if let Some(&SetCookie(ref content)) = post_request.headers().get::<SetCookie>() {
        for def in content {
            let fields = def.to_string();
            let fields_string = fields.split(';').collect::<Vec<&str>>();
            for f in fields_string {
                let fi = f.to_string();
                let mut values = fi.split('=');
                cookie.set(
                    values
                        .next()
                        .expect("Could not find the first field for the Cookie")
                        .to_string(),
                    values.next().unwrap_or("").to_string(),
                );
            }
        }
    }

    let mut html_page_content = String::new();
    let uri_grade: String = format!("https://{}/{}", &c.base_uri, &c.grade_uri)
        .parse()
        .unwrap();
    client
        .get(&uri_grade)
        .expect("GET request failed")
        .headers(headers)
        .send()
        .expect("GET request failed to be sent")
        .read_to_string(&mut html_page_content);
    println!("{}", html_page_content);
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
        .inspect(|x| println!("{:?}", x))
        .map(|&x| x.inner_html())
        .collect::<String>();
    println!("Table : {:?}", grade_table);
}

fn main() {
    let c = credentials_login();
    // Obtaining all the courses
    retreive_all_courses_id(c);
    // let coursesID: Vec<i32> = retreive_all_courses_id(c); // Array containing the courses ID
    // for nc in courses_id {
    // Spawn workers for each one of them
    // let worker = thread::spawn(move || {
    // watcher::watcher::new(nc, None, Some(3600));
    // watcher::watcher.run();
    // });
    // }
    // Catch all the threads in case of crashing
    // let result = worker.join();
}
