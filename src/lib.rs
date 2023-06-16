#[cfg(feature = "build")]
mod build;
#[cfg(feature = "bindgen-tests")]
pub mod testing;
#[cfg(feature = "build")]
pub use build::generate_scaffolding;

use anyhow::Result;
use camino::Utf8Path;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use toml::Value;
use uniffi_bindgen::backend::TemplateExpression;
use uniffi_bindgen::{BindingGenerator, BindingGeneratorConfig, ComponentInterface};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Config {
    package_name: Option<String>,
    cdylib_name: Option<String>,
    #[serde(default)]
    custom_types: HashMap<String, CustomTypeConfig>,
    #[serde(default)]
    external_packages: HashMap<String, String>,
}

impl Config {
    pub fn package_name(&self) -> String {
        if let Some(package_name) = &self.package_name {
            package_name.clone()
        } else {
            "uniffi".into()
        }
    }

    pub fn cdylib_name(&self) -> String {
        if let Some(cdylib_name) = &self.cdylib_name {
            cdylib_name.clone()
        } else {
            "uniffi".into()
        }
    }
}

impl BindingGeneratorConfig for Config {
    fn get_entry_from_bindings_table(_bindings: &Value) -> Option<Value> {
        if let Some(table) = _bindings.as_table() {
            table.get("dart").map(|v| v.clone())
        } else {
            None
        }
    }

    fn get_config_defaults(ci: &ComponentInterface) -> Vec<(String, Value)> {
        vec![
            (
                "package_name".to_string(),
                Value::String(ci.namespace().to_string()),
            ),
            (
                "cdylib_name".to_string(),
                Value::String(ci.namespace().to_string()),
            ),
        ]
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct CustomTypeConfig {
    imports: Option<Vec<String>>,
    type_name: Option<String>,
    into_custom: TemplateExpression,
    from_custom: TemplateExpression,
}
pub struct DartBindingGenerator {}

impl BindingGenerator for DartBindingGenerator {
    type Config = Config;

    fn write_bindings(
        &self,
        ci: ComponentInterface,
        config: Self::Config,
        out_dir: &Utf8Path,
    ) -> Result<()> {
        unimplemented!("this is where the fun happens")
    }
}
