#[derive(Debug, Clone, uniffi::Enum)]
pub enum Shape {
    Circle,
    Rectangle,
}

include!(concat!(env!("OUT_DIR"), "/api.uniffi.rs"));
