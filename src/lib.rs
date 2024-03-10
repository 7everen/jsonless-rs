// This crate is a library
#![crate_type = "lib"]
// The library is named "rary"
#![crate_name = "jsonless"]
pub fn encode() {
    println!("called `encode()`");
}

pub fn decode() {
    println!("called `decode()`");
}