#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::ffi::CString;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

fn main() {
    println!("Hello, world!");

    unsafe {
        let f = GoFunction(2);
        println!("{}", f);
    }

    t("halo".to_string());
}

fn t(t: String) {
    let c_ref = CString::new(t).unwrap();
    let go_str_ref = GoString {
        p: c_ref.as_ptr(),
        n: c_ref.as_bytes().len() as isize,
    };
    let result = unsafe {
        let ret = demoHello(go_str_ref);
        CString::from_raw(ret)
    };

    println!("{}", result.to_str().unwrap());
}
