use std::sync::Arc;
use uniffi;

#[derive(uniffi::Record, Clone)]
pub struct WorldState {
    inhabitants: u8,
    name: Option<String>,
}

impl WorldState {
    fn new(name: Option<String>) -> WorldState {
        WorldState {
            inhabitants: 0,
            name,
        }
    }
}

#[derive(uniffi::Object, Clone)]
pub struct World(WorldState);

impl World {
    fn new(name: Option<String>) -> World {
        World(WorldState::new(name))
    }
}

#[uniffi::export]
impl World {
    fn is_there(&self) -> bool {
        true
    }

    fn name(&self) -> Option<String> {
        self.0.name.clone()
    }

    fn state(&self) -> WorldState {
        self.0.clone()
    }

    fn inc_inhabitants(self: Arc<Self>) -> u8 {
        let mut me = Arc::try_unwrap(self).unwrap_or_else(|x| (*x).clone());
        me.0.inhabitants += 1;
        me.0.inhabitants
    }

    fn prefixed_name(&self, inp: Option<String>) -> Option<String> {
        match (inp, &self.0.name) {
            (Some(e), Some(f)) => Some(format!("{e} {f}")),
            _ => None,
        }
    }
    fn set_name(self: Arc<Self>, inp: Option<String>) -> Arc<Self> {
        let mut me = Arc::try_unwrap(self).unwrap_or_else(|x| (*x).clone());
        me.0.name = inp;
        Arc::new(me)
    }
}

#[uniffi::export]
pub fn new_world() -> Arc<World> {
    Arc::new(World::new(None))
}

#[uniffi::export]
pub fn new_world_with_name(name: String) -> Arc<World> {
    Arc::new(World::new(Some(name)))
}

#[uniffi::export]
pub fn hello_world() -> String {
    format!("hello world")
}

#[uniffi::export]
pub fn hello(input: String) -> String {
    format!("hello {input}")
}

uniffi::include_scaffolding!("api");

mod uniffi_types {
    pub use crate::{World, WorldState};
}
