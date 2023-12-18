use genco::lang::dart;
use uniffi_bindgen::interface::Type;

mod primitives;
mod enums;

pub trait Renderer {
     fn render(&self, r: &impl Renderable) -> dart::Tokens;
}

pub trait Renderable {
     fn render_type(&self, ty: &Type) -> dart::Tokens;
     fn render_type_helpers(&self, ty: &Type) -> dart::Tokens;
}