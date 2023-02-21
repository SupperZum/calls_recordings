//use serde_json::{Result, Value};
use std::{fs::File, io::Write, process};

use curl::easy::{Easy, List};

fn main() {
    File::create("test").unwrap();
    let mut file = File::options()
        .write(true)
        .append(true)
        .open("test")
        .unwrap();
    let mut easy = Easy::new();
    easy.url("https://api.aircall.io/v1/calls").unwrap();
    let mut list = List::new();
    list.append("Authorization: Basic *********").unwrap();
    easy.http_headers(list).unwrap();
    easy.write_function(move |data| {
        if let Err(err) = file.write_all(data) {
            println!("{}", err);
            process::exit(-1);
        }

        Ok(data.len())
    })
    .unwrap();
    easy.perform().unwrap();
}
