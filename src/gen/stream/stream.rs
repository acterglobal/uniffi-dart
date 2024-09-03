use genco::prelude::*;
use uniffi_bindgen::interface::Object;

use crate::gen::oracle::DartCodeOracle;
use crate::gen::render::TypeHelperRenderer;


pub fn generate_stream(obj: &Object, _type_helper: &dyn TypeHelperRenderer) -> dart::Tokens {
    let obj_name = obj.name();
    let obj_var_name = &DartCodeOracle::var_name(obj_name);
    let fn_name = DartCodeOracle::fn_name(&obj_name.replace("StreamExt", ""));
    let create_obj_fn_name = format!("createStream{}", &obj_name.replace("StreamExt", ""));

    quote! {
        final $obj_var_name = $create_obj_fn_name();

        $fn_name() async* {
            try {
                while (true) {
                final value = await $obj_var_name.pollNext();
                print(value);
                if (value == null) {
                    break;
                }
                yield value;
                }
            } finally {
                $obj_var_name.drop();
            }
        }
    }
}