use uniffi;

// A close replication of what's in the original test from the uniffi-rs repo
#[uniffi::export]
pub trait ForeignGetters {
    fn get_bool(&self, v: bool, argument_two: bool) -> Result<bool, SimpleError>;
    fn get_string(&self, v: String, arg2: bool) -> Result<String, SimpleError>;
    fn get_option(&self, v: Option<String>, arg2: bool) -> Result<Option<String>, ComplexError>;
    fn get_list(&self, v: Vec<i32>, arg2: bool) -> Result<Vec<i32>, SimpleError>;
    fn get_nothing(&self, v: String) -> Result<(), SimpleError>;
}

// Dart can throw any non-null object as an exception
#[derive(Debug, thiserror::Error)]
pub enum SimpleError {
    #[error("BadArgument")]
    BadArgument,
    #[error("InternalTelephoneError")]
    UnexpectedError,
}


#[derive(Debug, thiserror::Error)]
pub enum ComplexError {
    #[error("ReallyBadArgument")]
    ReallyBadArgument { code: i32 },
    #[error("InternalTelephoneError")]
    UnexpectedErrorWithReason { reason: String },
}

impl From<uniffi::UnexpectedUniFFICallbackError> for SimpleError {
    fn from(e: uniffi::UnexpectedUniFFICallbackError) -> SimpleError {
        SimpleError::UnexpectedError
    }
}


impl From<uniffi::UnexpectedUniFFICallbackError> for ComplexError {
    fn from(e: uniffi::UnexpectedUniFFICallbackError) -> ComplexError {
        ComplexError::UnexpectedErrorWithReason { reason: e.reason }
    }
}


#[derive(Debug, Clone, uniffi::Object)]
pub struct RustGetters;

// TODO: solve the type error when using the export proc_macro to simplify and delete the contents of api.udl
//#[uniffi::export]
impl RustGetters {
   #[uniffi::constructor]
    pub fn new() -> Self {
        RustGetters
    }
    pub fn get_bool(
        &self,
        callback: Box<dyn ForeignGetters>,
        v: bool,
        argument_two: bool,
    ) -> Result<bool, SimpleError> {
        callback.get_bool(v, argument_two)
    }
    pub fn get_string(
        &self,
        callback: Box<dyn ForeignGetters>,
        v: String,
        arg2: bool,
    ) -> Result<String, SimpleError> {
        callback.get_string(v, arg2)
    }
    pub fn get_option(
        &self,
        callback: Box<dyn ForeignGetters>,
        v: Option<String>,
        arg2: bool,
    ) -> Result<Option<String>, ComplexError> {
        callback.get_option(v, arg2)
    }
   pub fn get_list(
        &self,
        callback: Box<dyn ForeignGetters>,
        v: Vec<i32>,
        arg2: bool,
    ) -> Result<Vec<i32>, SimpleError> {
        callback.get_list(v, arg2)
    }

   pub fn get_string_optional_callback(
        &self,
        callback: Option<Box<dyn ForeignGetters>>,
        v: String,
        arg2: bool,
    ) -> Result<Option<String>, SimpleError> {
        callback.map(|c| c.get_string(v, arg2)).transpose()
    }

   pub fn get_nothing(&self, callback: Box<dyn ForeignGetters>, v: String) -> Result<(), SimpleError> {
        callback.get_nothing(v)
    }
}


impl Default for RustGetters {
    fn default() -> Self {
        Self::new()
    }
}

#[allow(clippy::wrong_self_convention)]
#[uniffi::export]
trait StoredForeignStringifier: Send + Sync + std::fmt::Debug {
    fn from_simple_type(&self, value: i32) -> String;
    fn from_complex_type(&self, values: Option<Vec<Option<f64>>>) -> String;
}

#[derive(Debug, uniffi::Object)]
pub struct RustStringifier {
    callback: Box<dyn StoredForeignStringifier>,
}

#[uniffi::export]
impl RustStringifier {
    fn new(callback: Box<dyn StoredForeignStringifier>) -> Self {
        RustStringifier { callback }
    }

    #[allow(clippy::wrong_self_convention)]
    fn from_simple_type(&self, value: i32) -> String {
        self.callback.from_simple_type(value)
    }
}


uniffi::include_scaffolding!("api");