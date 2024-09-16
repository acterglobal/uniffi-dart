use proc_macro::TokenStream;
use quote::{format_ident, quote};
use stringcase::pascal_case;
use syn::{parse::Parse, parse_macro_input, ItemFn, Type};

struct StreamAttr {
    item_type: Type,
}

impl Parse for StreamAttr {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
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
    let struct_name = format_ident!("{}Stream", pascal_case(&fn_name.to_string()));
    let create_fn_name = format_ident!("create_stream_{}", fn_name);
    let item_type = &attr.item_type;

    let expanded = quote! {
        #input

        #[derive(uniffi::Object)]
        #vis struct #struct_name {
            stream: tokio::sync::Mutex<std::pin::Pin<Box<dyn futures::Stream<Item = #item_type> + Send>>>,
        }

        #[uniffi::export(async_runtime = "tokio")]
        impl #struct_name {
            #[uniffi::constructor]
            pub fn new() -> std::sync::Arc<Self> {
                std::sync::Arc::new(Self {
                    stream: tokio::sync::Mutex::new(Box::pin(#fn_name())),
                })
            }

            pub async fn next(&self) -> Option<#item_type> {
                let mut stream = self.stream.lock().await;
                stream.as_mut().next().await
            }

        }

        #[uniffi::export]
        #vis fn #create_fn_name() -> std::sync::Arc<#struct_name> {
            #struct_name::new()
        }
    };

    TokenStream::from(expanded)
}
