use uniffi;

#[uniffi::export]
pub fn hello_world() -> String {
    format!("hello world")
}

#[uniffi::export]
pub fn hello(input: String) -> String {
    let len = input.len();
    println!("received call: {len}");
    format!("hello {input}")
}

include!(concat!(env!("OUT_DIR"), "/api.uniffi.rs"));
