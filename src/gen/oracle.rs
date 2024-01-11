use heck::{ToLowerCamelCase, ToUpperCamelCase};

use uniffi_bindgen::backend::{CodeType};
use uniffi_bindgen::interface::{Type, Literal};


pub struct DartCodeOracle;

impl DartCodeOracle {
    fn find(&self, type_: &Type) -> Box<dyn CodeType> {
        type_.clone().as_type().as_codetype()
    }

    fn find_as_error(&self, type_: &Type) -> Box<dyn CodeType> {
        match type_ {
            Type::Enum(id) => Box::new(error::ErrorCodeType::new(id.clone())),
            _ => panic!("unsupported type for error: {type_:?}"),
        }
    }

    /// Sanitize a Dart identifier, appending an underscore if it's a reserved keyword.
    pub fn sanitize_identifier(&self, id: &str) -> String {
        if self.is_reserved_identifier(id) {
            format!("{}_", id)
        } else {
            id.to_string()
        }
    }

    /// Get the idiomatic Dart rendering of a class name (for enums, records, errors, etc).
    fn class_name(&self, nm: &str) -> String {
        self.sanitize_identifier(&nm.to_upper_camel_case())
    }

    /// Get the idiomatic Dart rendering of a function name.
    fn fn_name(&self, nm: &str) -> String {
        self.sanitize_identifier(&nm.to_lower_camel_case())
    }

    /// Get the idiomatic Dart rendering of a variable name.
    fn var_name(&self, nm: &str) -> String {
        self.sanitize_identifier(&nm.to_lower_camel_case())
    }

    /// Get the idiomatic Dart rendering of an individual enum variant.
    fn enum_variant_name(&self, nm: &str) -> String {
        nm.to_string().to_shouty_snake_case()
    }

    /// Get the idiomatic Dart rendering of an exception name
    ///
    /// This replaces "Error" at the end of the name with "Exception".  Rust code typically uses
    /// "Error" for any type of error but in the Java world, "Error" means a non-recoverable error
    /// and is distinguished from an "Exception".
    fn error_name(&self, nm: &str) -> String {
        // errors are a class in Dart.
        let name = self.class_name(nm);
        match name.strip_suffix("Error") {
            None => name,
            Some(stripped) => format!("{stripped}Exception"),
        }
    }
}

    /// Check if the given identifier is a reserved keyword in Dart.
    pub fn is_reserved_identifier(&self, id: &str) -> bool {
        RESERVED_IDENTIFIERS.contains(&id)
    }

    /// Get the idiomatic Dart rendering of a class name (for enums, records, errors, etc).
    #[allow(dead_code)]
    pub fn class_name(&self, nm: &str) -> String {
        self.sanitize_identifier(&nm.to_upper_camel_case())
    }

    /// Get the idiomatic Dart rendering of a function name.
    pub fn fn_name(&self, nm: &str) -> String {
        self.sanitize_identifier(&nm.to_lower_camel_case())
    }

    /// Get the idiomatic Dart rendering of a variable name.
    #[allow(dead_code)]
    pub fn var_name(&self, nm: &str) -> String {
        self.sanitize_identifier(&nm.to_lower_camel_case())
    }

    /// Get the idiomatic Dart rendering of an individual enum variant.
    #[allow(dead_code)]
    pub fn enum_variant_name(&self, nm: &str) -> String {
        self.sanitize_identifier(&nm.to_lower_camel_case())
    }

    /// Get the idiomatic Dart rendering of an exception name.
    #[allow(dead_code)]
    pub fn error_name(&self, nm: &str) -> String {
        self.class_name(nm)
    }

    
}

// https://dart.dev/guides/language/language-tour#keywords
pub static RESERVED_IDENTIFIERS: [&str; 63] = [
    // This list may need to be updated as the Dart language evolves.
    "abstract",
    "as",
    "assert",
    "async",
    "await",
    "break",
    "case",
    "catch",
    "class",
    "const",
    "continue",
    "covariant",
    "default",
    "deferred",
    "do",
    "dynamic",
    "else",
    "enum",
    "export",
    "extends",
    "extension",
    "external",
    "factory",
    "false",
    "final",
    "finally",
    "for",
    "Function",
    "get",
    "hide",
    "if",
    "implements",
    "import",
    "in",
    "interface",
    "is",
    "late",
    "library",
    "mixin",
    "new",
    "null",
    "on",
    "operator",
    "part",
    "required",
    "rethrow",
    "return",
    "set",
    "show",
    "static",
    "super",
    "switch",
    "sync",
    "this",
    "throw",
    "true",
    "try",
    "typedef",
    "var",
    "void",
    "while",
    "with",
    "yield",
];
