use std::sync::Arc;
use uniffi;

#[derive(uniffi::Object)]
pub struct World(Option<String>);

#[uniffi::export]
impl World {
    fn is_there(&self) -> bool {
        true
    }
    fn name(&self) -> Option<String> {
        self.0.clone()
    }
}

#[uniffi::export]
pub fn new_world() -> Arc<World> {
    Arc::new(World(None))
}

#[uniffi::export]
pub fn new_world_with_name(name: String) -> Arc<World> {
    Arc::new(World(Some(name)))
}

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

mod uniffi_types {
    pub use crate::World;
}
