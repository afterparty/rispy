extern crate clap;
extern crate libc;

use std::io::{self, Write};
use clap::App;

extern {
    fn double_input(input: libc::c_int) -> libc::c_int;
}

fn main() {
    //let _args: Vec<String> = env::args().collect();
    let _matches = App::new("Rispy Interpreter")
        .version("0.1.0")
        .author("https://github.com/afterparty")
        .about("Lisp-like interpreter made in Rust")
        .get_matches();
    let mut buffer = String::new();
    
    let input = 2;
    let output = unsafe { double_input(input) };
    println!("{} * 2 = {}", input, output);
    
    println!("rispy version: 0.1.0");
    
    loop {
        print!("rispy> ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut buffer).ok().expect("failed to read line");
        let input = format!("given instructions {}", buffer);
        print!("{}", input);
    }
}
