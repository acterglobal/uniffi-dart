use super::{compounds, enums, primitives, records};
use super::{objects, oracle::AsCodeType};
use genco::{lang::dart, quote};
use uniffi_bindgen::interface::{AsType, Enum, Object, Record, Type};
use uniffi_bindgen::ComponentInterface;

pub trait Renderer<T> {
    fn render(&self) -> T;
}

pub trait TypeHelperRenderer {
    fn get_ci(&self) -> &ComponentInterface;
    fn include_once_check(&self, name: &str, ty: &Type) -> bool;
    fn check(&self, name: &str) -> bool;
    fn add_import(&self, name: &str) -> bool;
    fn add_import_as(&self, name: &str, as_name: &str) -> bool;
    fn get_object(&self, name: &str) -> Option<&Object>;
    fn get_enum(&self, name: &str) -> Option<&Enum>;
    fn get_record(&self, name: &str) -> Option<&Record>;
    fn ffi_type_label(&self, ffi_type: &uniffi_bindgen::interface::FfiType) -> dart::Tokens;
    fn ffi_native_type_label(&self, ffi_type: &uniffi_bindgen::interface::FfiType) -> dart::Tokens;
}

pub trait Renderable {
    fn render(&self) -> dart::Tokens {
        quote!()
    }

    fn render_type(&self, ty: &Type, type_helper: &dyn TypeHelperRenderer) -> dart::Tokens {
        let type_name = match ty {
            Type::UInt8 | Type::Int8 | Type::UInt16 | Type::Int16 |
            Type::UInt32 | Type::Int32 | Type::UInt64 | Type::Int64 => quote!(int),
            Type::Float32 | Type::Float64 => quote!(double),
            Type::String => quote!(String),
            Type::Boolean => quote!(bool),
            Type::Object { name, .. } => quote!($name),
            Type::Optional { inner_type } => quote!($(&self.render_type(inner_type, type_helper))?),
            Type::Sequence { inner_type } => quote!(List<$(&self.render_type(inner_type, type_helper))>),
            Type::Map { key_type, value_type } => quote!(Map<$(&self.render_type(key_type, type_helper)), $(&self.render_type(value_type, type_helper))>),
            Type::Enum { name, .. } => quote!($name),
            Type::Record { name, .. } => quote!($name),
            Type::Duration => quote!(Duration),
            _ => todo!("Type::{:?}", ty),
        };

        if type_helper.include_once_check(&ty.as_codetype().canonical_name(), ty) {
            println!("{} Added", &ty.as_codetype().canonical_name());
        }

        type_name
    }

    fn render_type_helper(&self, type_helper: &dyn TypeHelperRenderer) -> dart::Tokens;
}

pub trait AsRenderable {
    fn as_renderable(&self) -> Box<dyn Renderable>;
}

impl<T: AsType> AsRenderable for T {
    fn as_renderable(&self) -> Box<dyn Renderable> {
        match self.as_type() {
            Type::UInt8 => Box::new(primitives::UInt8CodeType),
            Type::Int8 => Box::new(primitives::Int8CodeType),
            Type::UInt16 => Box::new(primitives::UInt16CodeType),
            Type::Int16 => Box::new(primitives::Int16CodeType),
            Type::UInt32 => Box::new(primitives::UInt32CodeType),
            Type::Int32 => Box::new(primitives::Int32CodeType),
            Type::UInt64 => Box::new(primitives::UInt64CodeType),
            Type::Int64 => Box::new(primitives::Int64CodeType),
            Type::Float32 => Box::new(primitives::Float32CodeType),
            Type::Float64 => Box::new(primitives::Float64CodeType),
            Type::Boolean => Box::new(primitives::BooleanCodeType),
            Type::String => Box::new(primitives::StringCodeType),
            Type::Duration => Box::new(primitives::DurationCodeType),
            Type::Object { name, .. } => Box::new(objects::ObjectCodeType::new(name)),
            Type::Optional { inner_type } => Box::new(compounds::OptionalCodeType::new(
                self.as_type(),
                *inner_type,
            )),
            Type::Sequence { inner_type } => Box::new(compounds::SequenceCodeType::new(
                self.as_type(),
                *inner_type,
            )),
            Type::Enum { name, .. } => Box::new(enums::EnumCodeType::new(name)),
            Type::Record { name, module_path } => {
                Box::new(records::RecordCodeType::new(name, module_path))
            }
            _ => todo!("Renderable for Type::{:?}", self.as_type()),
        }
    }
}

