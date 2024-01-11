use heck::{ToLowerCamelCase, ToUpperCamelCase};

pub struct DartCodeOracle;

impl DartCodeOracle {
    /// Sanitize a Dart identifier, appending an underscore if it's a reserved keyword.
    pub fn sanitize_identifier(&self, id: &str) -> String {
        if self.is_reserved_identifier(id) {
            format!("{}_", id)
        } else {
            id.to_string()
        }
    }

    /// Check if the given identifier is a reserved keyword in Dart.
    pub fn is_reserved_identifier(&self, id: &str) -> bool {
        RESERVED_IDENTIFIERS.contains(&id)
    }

    /// Get the idiomatic Dart rendering of a class name (for enums, records, errors, etc).
    pub fn class_name(&self, nm: &str) -> String {
        self.sanitize_identifier(&nm.to_upper_camel_case())
    }

    /// Get the idiomatic Dart rendering of a function name.
    pub fn fn_name(&self, nm: &str) -> String {
        self.sanitize_identifier(&nm.to_lower_camel_case())
    }

    /// Get the idiomatic Dart rendering of a variable name.
    pub fn var_name(&self, nm: &str) -> String {
        self.sanitize_identifier(&nm.to_lower_camel_case())
    }

    /// Get the idiomatic Dart rendering of an individual enum variant.
    pub fn enum_variant_name(&self, nm: &str) -> String {
        self.sanitize_identifier(&nm.to_lower_camel_case())
    }

    /// Get the idiomatic Dart rendering of an exception name.
    pub fn error_name(&self, nm: &str) -> String {
        self.class_name(nm)
    }
}

// https://dart.dev/guides/language/language-tour#keywords
pub static RESERVED_IDENTIFIERS: [&str; 63] = [
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
use heck::{ToLowerCamelCase, ToUpperCamelCase};

pub struct DartCodeOracle;

impl DartCodeOracle {
    pub fn sanitize_identifier(&self, id: &str) -> String {
        if self.is_reserved_identifier(id) {
            format!("{}_", id)
        } else {
            id.to_string()
        }
    }

    pub fn is_reserved_identifier(&self, id: &str) -> bool {
        RESERVED_IDENTIFIERS.contains(&id)
    }

    pub fn class_name(&self, nm: &str) -> String {
        self.sanitize_identifier(&nm.to_upper_camel_case())
    }

    pub fn fn_name(&self, nm: &str) -> String {
        self.sanitize_identifier(&nm.to_lower_camel_case())
    }

    pub fn var_name(&self, nm: &str) -> String {
        self.sanitize_identifier(&nm.to_lower_camel_case())
    }

    pub fn enum_variant_name(&self, nm: &str) -> String {
        self.sanitize_identifier(&nm.to_lower_camel_case())
    }

    pub fn error_name(&self, nm: &str) -> String {
        self.class_name(nm)
    }
}

pub static RESERVED_IDENTIFIERS: [&str; 63] = [
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
