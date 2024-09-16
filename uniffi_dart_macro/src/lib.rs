use proc_macro::TokenStream;
use quote::{format_ident, quote};
use stringcase::pascal_case;
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input, ItemFn, Type,
};

// Struct to parse the attribute input
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
            receiver: tokio::sync::Mutex<futures::channel::mpsc::Receiver<#item_type>>,
            // Optional: You can add a sender if you need to control the stream externally
        }

        #[uniffi::export]
        impl #struct_name {
            #[uniffi::constructor]
            pub fn new() -> std::sync::Arc<Self> {
                use futures::SinkExt;
                let (sender, receiver) = futures::channel::mpsc::channel(100);
                let stream = #fn_name();
                let sender = std::sync::Arc::new(tokio::sync::Mutex::new(sender));

                // Spawn a Tokio task to drive the stream and send items through the channel
                tokio::spawn(async move {
                    futures::pin_mut!(stream);
                    while let Some(item) = stream.next().await {
                        // Clone the sender to avoid holding the lock across await points
                        let sender_clone = sender.clone();
                        let mut sender_lock = sender_clone.lock().await;
                        if sender_lock.send(item).await.is_err() {
                            // Receiver has been dropped
                            break;
                        }
                    }
                });

                std::sync::Arc::new(Self {
                    receiver: tokio::sync::Mutex::new(receiver),
                })
            }

            /// Poll the next item from the stream
            pub async fn poll_next(&self) -> Option<#item_type> {
                let mut receiver_lock = self.receiver.lock().await;
                receiver_lock.next().await
            }

        }

        #[uniffi::export]
        #vis fn #create_fn_name() -> std::sync::Arc<#struct_name> {
            #struct_name::new()
        }
    };

    TokenStream::from(expanded)
}
