use super::{compounds, enums, primitives, records};
use super::{objects, oracle::AsCodeType};
use genco::{lang::dart, quote};
use uniffi_bindgen::interface::{AsType, Enum, Object, Record, Type};
use uniffi_bindgen::ComponentInterface;

/// This trait will be used by any type that generates dart code according to some logic,
pub trait Renderer<T> {
    fn render(&self) -> T;
}

// This trait contains helpful methods for rendering type helpers
#[allow(dead_code)]
pub trait TypeHelperRenderer {
    // Gives context about weather a type's helper code has already been included
    fn include_once_check(&self, name: &str, ty: &Type) -> bool;
    fn check(&self, name: &str) -> bool;
    // Helps type helper functions specify a required imports should be added
    fn add_import(&self, name: &str) -> bool;
    fn add_import_as(&self, name: &str, as_name: &str) -> bool;
    // Helps Renderer Find Specific Types
    fn get_object(&self, name: &str) -> Option<&Object>;
    fn get_enum(&self, name: &str) -> Option<&Enum>;
    fn get_record(&self, name: &str) -> Option<&Record>;
    fn get_ci(&self) -> &ComponentInterface;
}
/// This trait is used by types that should be generated. The idea is to pass any struct that implements
/// this type to another struct that generates much larger portions of according to some internal logic code
/// and implements `Renderer`.
pub trait Renderable {
    /// Renders the code that defines a type
    #[allow(dead_code)]
    fn render(&self) -> dart::Tokens {
        quote!()
    }
    /// Renders the code to label a type
    fn render_type(&self, ty: &Type, type_helper: &dyn TypeHelperRenderer) -> dart::Tokens {
        let type_name = match ty {
            Type::UInt8
            | Type::UInt32
            | Type::Int8
            | Type::Int16
            | Type::Int64
            | Type::UInt16
            | Type::Int32
            | Type::UInt64 => quote!(int),
            Type::Float32 | Type::Float64 => quote!(double),
            Type::String => quote!(String),
            Type::Object { name, .. } => quote!($name),
            Type::Boolean => quote!(bool),
            Type::Duration => quote!(Duration),
            Type::Optional { inner_type } => quote!($(&self.render_type(inner_type, type_helper))?),
            Type::Sequence { inner_type } => {
                quote!(List<$(&self.render_type(inner_type, type_helper))>)
            }
            Type::Enum { name, .. } => quote!($name),
            Type::Record { name, .. } => quote!($name),
            // Type:: { name,..  } => quote!($name),
            _ => todo!("Type::{:?}", ty), // AbiType::Num(ty) => self.generate_wrapped_num_type(*ty),
                                          // AbiType::Isize | AbiType::Usize => quote!(int),
                                          // AbiType::Bool => quote!(bool),
                                          // AbiType::RefStr | AbiType::String => quote!(String),
                                          // AbiType::RefSlice(ty) | AbiType::Vec(ty) => {
                                          //     quote!(List<#(self.generate_wrapped_num_type(*ty))>)
                                          // }
                                          // AbiType::Option(ty) => quote!(#(self.generate_type(ty))?),
                                          // AbiType::Result(ty) => self.generate_type(ty),
                                          // AbiType::Tuple(tuple) => match tuple.len() {
                                          //     0 => quote!(void),
                                          //     1 => self.generate_type(&tuple[0]),
                                          //     _ => quote!(List<dynamic>),
                                          // },
                                          // AbiType::RefObject(ty) | AbiType::Object(ty) => quote!(#ty),
                                          // AbiType::RefIter(ty) | AbiType::Iter(ty) => quote!(Iter<#(self.generate_type(ty))>),
                                          // AbiType::RefFuture(ty) | AbiType::Future(ty) => {
                                          //     quote!(Future<#(self.generate_type(ty))>)
                                          // }
                                          // AbiType::RefStream(ty) | AbiType::Stream(ty) => {
                                          //     quote!(Stream<#(self.generate_type(ty))>)
                                          // }
                                          // AbiType::Buffer(ty) => quote!(#(ffi_buffer_name_for(*ty))),
                                          // AbiType::List(ty) => quote!(#(format!("FfiList{}", ty))),
                                          // AbiType::RefEnum(ty) => quote!(#(ty)),
        };

        if type_helper.include_once_check(&ty.as_codetype().canonical_name(), ty) {
            println!("{} Added", &ty.as_codetype().canonical_name());
        }

        type_name
    }
    /// Renders code that defines a type and other code for type helpers for lifting, lowering, buffer conversion, etc... with access to the type helper
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
            Type::Optional { inner_type, .. } => Box::new(compounds::OptionalCodeType::new(
                self.as_type(),
                *inner_type,
            )),
            Type::Sequence { inner_type, .. } => Box::new(compounds::SequenceCodeType::new(
                self.as_type(),
                *inner_type,
            )),
            Type::Enum { name, .. } => Box::new(enums::EnumCodeType::new(name)),
            Type::Record { name, module_path } => {
                Box::new(records::RecordCodeType::new(name, module_path))
            }

            _ => todo!("Renderable for Type::{:?}", self.as_type()), // Type::Bytes => Box::new(primitives::BytesCodeType),

                                                                     // Type::Timestamp => Box::new(miscellany::TimestampCodeType),

                                                                     // Type::Object { name, .. } => Box::new(object::ObjectCodeType::new(name)),
                                                                     // Type::Record(id) => Box::new(record::RecordCodeType::new(id)),
                                                                     // Type::CallbackInterface(id) => {
                                                                     //     Box::new(callback_interface::CallbackInterfaceCodeType::new(id))
                                                                     // }
                                                                     // Type::ForeignExecutor => Box::new(executor::ForeignExecutorCodeType),
                                                                     // Type::Optional(inner) => Box::new(compounds::OptionalCodeType::new(*inner)),
                                                                     // Type::Sequence(inner) => Box::new(compounds::SequenceCodeType::new(*inner)),
                                                                     // Type::Map(key, value) => Box::new(compounds::MapCodeType::new(*key, *value)),
                                                                     // Type::External { name, .. } => Box::new(external::ExternalCodeType::new(name)),
                                                                     // Type::Custom { name, .. } => Box::new(custom::CustomCodeType::new(name)),
        }
    }
}
