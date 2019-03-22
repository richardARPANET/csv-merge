#![feature(intrinsics)]
#![feature(core_intrinsics)]
// use std::intrinsics;

use core::intrinsics;


fn main() {
    let stuff2 = "Stuff";
    unsafe {
        let stuff = "Stuff";
        intrinsics::breakpoint();
    }
    // pub unsafe extern "rust-intrinsic" fn breakpoint();
}
// // use std::intrinsics::breakpoint;
// use std::env;
// // use std::error::Error;
// use std::io;
// // use std::process;

// extern crate csv;

// fn main() {
//     println!("Hello, world!");
//     let args:Vec<String> = env::args().collect();
//     let csv_paths = &args[1..];
//     println!("{:?}", csv_paths);
//     println!("Loading csv.");
//     let mut rdr = csv::Reader::from_reader(io::stdin());
//     breakpoint!();
//     for result in rdr.records() {
//         println!("{:?}", result);
//     }
// }
