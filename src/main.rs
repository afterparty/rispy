#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

extern crate clap;
extern crate libc;

use std::io::{self, Write};
use clap::App;

fn main() {
    //let _args: Vec<String> = env::args().collect();
    let _matches = App::new("Rispy Interpreter")
        .version("0.1.0")
        .author("https://github.com/afterparty")
        .about("Lisp-like interpreter made in Rust")
        .get_matches();
    let mut buffer = String::new();
    
    println!("rispy version: 0.1.0");
    
    loop {
        print!("rispy> ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut buffer).ok().expect("failed to read line");
        let input = format!("given instructions {}", buffer);
        print!("{}", input);
    }
}
