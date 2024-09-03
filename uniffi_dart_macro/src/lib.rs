use proc_macro::TokenStream;
use quote::{quote, format_ident};
use stringcase::pascal_case;

use syn::{parse::{Parse, ParseStream}, parse_macro_input, ItemFn, Type};


struct StreamAttr {
    item_type: Type,
}

impl Parse for StreamAttr {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let item_type: Type = input.parse()?;
        Ok(StreamAttr { item_type })
    }
}

#[proc_macro_attribute]
pub fn export_stream(attr: TokenStream, item: TokenStream) -> TokenStream {
    let attr = parse_macro_input!(attr as StreamAttr);
    let input = parse_macro_input!(item as ItemFn);
    
    let fn_name = &input.sig.ident;
    let vis = &input.vis;
    let struct_name = format_ident!("{}StreamExt", pascal_case(&fn_name.to_string()));
    let create_fn_name = format_ident!("create_stream_{}", fn_name);
    let item_type = &attr.item_type;

    let expanded = quote! {
        #input

        #[derive(uniffi::Object)]
        #vis struct #struct_name {
            stream: std::sync::Arc<std::sync::Mutex<std::pin::Pin<Box<dyn futures::Stream<Item = #item_type> + Send>>>>,
        }

        #[uniffi::export]
        impl #struct_name {
            #[uniffi::constructor]
            fn new() -> std::sync::Arc<Self> {
                std::sync::Arc::new(Self {
                    stream: std::sync::Arc::new(std::sync::Mutex::new(Box::pin(#fn_name())))
                })
            }

            async fn poll_next(&self) -> Option<#item_type> {
                use futures::stream::StreamExt;
                let mut stream = self.stream.lock().unwrap();
                futures::executor::block_on(async {
                    stream.as_mut().next().await
                })
            }    
        }

        #[uniffi::export]
        #vis fn #create_fn_name() -> std::sync::Arc<#struct_name> {
            #struct_name::new()
        }
    };

    TokenStream::from(expanded)
}