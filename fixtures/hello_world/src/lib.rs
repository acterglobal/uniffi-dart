use std::sync::Arc;
use uniffi;

#[derive(uniffi::Object, Clone)]
pub struct World(Option<String>);

#[uniffi::export]
impl World {
    fn is_there(&self) -> bool {
        true
    }
    fn name(&self) -> Option<String> {
        self.0.clone()
    }

    fn prefixed_name(&self, inp: Option<String>) -> Option<String> {
        match (inp, &self.0) {
            (Some(e), Some(f)) => Some(format!("{e} {f}")),
            _ => None,
        }
    }
    fn set_name(self: Arc<Self>, inp: Option<String>) -> Arc<Self> {
        let mut me = Arc::try_unwrap(self).unwrap_or_else(|x| (*x).clone());
        me.0 = inp;
        Arc::new(me)
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
    format!("hello {input}")
}

include!(concat!(env!("OUT_DIR"), "/api.uniffi.rs"));

mod uniffi_types {
    pub use crate::World;
}
