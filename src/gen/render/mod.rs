use genco::lang::dart;
use uniffi_bindgen::interface::Type;

pub(crate) mod primitives;
pub(crate) mod enums;

/// This trait will be used by any type that generates dart code according to some logic, 
pub trait Renderer {
     fn render(&self) -> dart::Tokens;
}

// This trait contains helpful methods for rendering type helpers
pub trait TypeHelperRenderer {
     // Gives context about weather a type's helper code has already been included
     fn include_once_check(&self, name: &str) -> bool;
     // Helps type helper functions specify a required imports should be added
     fn add_import(&self, name: &str) -> bool;
     fn add_import_as(&self, name: &str, as_name: &str) -> bool;
}
/// This trait is used by types that should be generated. The idea is to pass any struct that implements 
/// this type to another struct that generates much larger portions of according to some internal logic code 
/// and implements `Renderer`.
pub trait Renderable {
     /// Renders the code that defines a type
     fn render_type(&self, ty: &Type) -> dart::Tokens;
     /// Renders code for type helpers for lifting, lowering, buffer conversion, etc...
     fn render_type_helpers(&self, ty: &Type, type_helper: &dyn TypeHelperRenderer) -> dart::Tokens;
}