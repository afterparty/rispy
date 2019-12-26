#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

extern crate clap;
extern crate libc;

use clap::App;
use std::ffi::{CStr, CString};
use std::io::{self, Write};

fn main() {
    //add mpc parser code
    extern "C" {
        fn mpc_new(name: *const ::std::os::raw::c_char) -> *mut mpc_parser_t;
        fn mpca_lang(flags: ::std::os::raw::c_int, language: *const ::std::os::raw::c_char, ...) -> *mut mpc_err_t;
        fn mpc_cleanup(n: ::std::os::raw::c_int, ...);
        fn mpc_parse(
            filename: *const ::std::os::raw::c_char,
            string: *const ::std::os::raw::c_char,
            p: *mut mpc_parser_t,
            r: *mut mpc_result_t,
        ) -> ::std::os::raw::c_int;
    }

    /*pub union mpc_result_t {
        pub error: *mut mpc_err_t,
        pub output: *mut mpc_val_t,
        _bindgen_union_align: u64,
    }*/

    let num = CString::new("number").expect("CString::new failed");
    let op = CString::new("operator").expect("CString::new failed");
    let expr = CString::new("expr").expect("CString::new failed");
    let lispy = CString::new("lispy").expect("CString::new failed");

    let number = unsafe { mpc_new(num.as_ptr()) };
    let op = unsafe { mpc_new(op.as_ptr()) };
    let expr = unsafe { mpc_new(expr.as_ptr()) };
    let lispy = unsafe { mpc_new(lispy.as_ptr()) };

    let lang_expr = CString::new(
        " number    : /-?[0-9]+/ ;
      operator  : '+' | '-' | '*' | '/' ;
      expr      : <number> | '(' <operator> <expr>+ ')' ;
      lispy     : /^/ <operator> <expr>+ /$/ ;
    ",
    )
    .expect("CString::new failed");

    unsafe {
        mpca_lang(
            MPCA_LANG_DEFAULT as i32,
            lang_expr.as_ptr(),
            number,
            op,
            expr,
            lispy,
        );
    }

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
        let mut r: *mpc_result_t;
        let stdin_string = CString::new("<stdin>").expect("CString::new failed");
        let lispy_string = CString::new("lispy> ").expect("CString::new failed");
        unsafe {
            let result = mpc_parse(stdin_string.as_ptr(), lispy_string.as_ptr(), lispy, r);
            if (result < 0) {
                
            }
        }
        //io::stdin().read_line(&mut buffer).ok().expect("failed to read line");
        //let input = format!("given instructions {}", buffer);
        //print!("{}", input);
    }

    unsafe {
        mpc_cleanup(4, number, op, expr, lispy);
    }
}
