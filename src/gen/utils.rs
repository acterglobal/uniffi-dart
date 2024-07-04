use heck::{ToLowerCamelCase, ToUpperCamelCase};

pub fn sanitize_identifier(id: &str) -> String {
    if RESERVED_IDENTIFIERS.contains(&id) {
        format!("{}_", id)
    } else {
        id.to_string()
    }
}

/// Get the idiomatic Dart rendering of a class name (for enums, records, errors, etc).
pub fn class_name(nm: &str) -> String {
    sanitize_identifier(&nm.to_upper_camel_case())
}

/// Get the idiomatic Dart rendering of a function name (for methods, etc).
pub fn fn_name(nm: &str) -> String {
    sanitize_identifier(&nm.to_lower_camel_case())
}

/// Get the idiomatic Dart rendering of a variable name.
pub fn var_name(nm: &str) -> String {
    sanitize_identifier(&nm.to_lower_camel_case())
}

/// Get the idiomatic Dart rendering of an individual enum variant.
pub fn enum_variant_name(nm: &str) -> String {
    sanitize_identifier(&nm.to_lower_camel_case())
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

