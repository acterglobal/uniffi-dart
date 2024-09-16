use genco::prelude::*;
use uniffi_bindgen::interface::Object;

use crate::gen::oracle::DartCodeOracle;
use crate::gen::render::TypeHelperRenderer;

pub fn generate_stream(obj: &Object, _type_helper: &dyn TypeHelperRenderer) -> dart::Tokens {
    let obj_name = obj.name();
    let fn_name = DartCodeOracle::fn_name(&obj_name.replace("StreamExt", ""));
    let obj_var_name = &DartCodeOracle::var_name(&fn_name);
    let create_obj_fn_name = format!("createStream{}", &obj_name.replace("StreamExt", ""));

    quote! {
        $fn_name() async* {
            final $obj_var_name = $create_obj_fn_name();
            try {
                while (true) {
                    final value = await $obj_var_name.next();
                    if (value == null) {
                        break;
                    }
                    yield value;
                }
            } catch (e) {
                // Handle exceptions if necessary
                rethrow;
            }
            // No need to call drop(), Finalizer will handle it
        }
    }
}
