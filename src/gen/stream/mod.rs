pub mod stream;

#[macro_export]
macro_rules! create_stream_ext {
    (
        $struct_name:ident,
        $inner_type:ty,
        $create_fn_name:ident,
        $stream_fn_name:ident
    ) => {
        #[derive(uniffi::Object)]
        pub struct $struct_name {
            stream: std::sync::Arc<std::sync::Mutex<std::pin::Pin<Box<dyn futures::Stream<Item = $inner_type> + Send>>>>,
        }

        #[uniffi::export]
        impl $struct_name {
            #[uniffi::constructor]
            fn new() -> std::sync::Arc<Self> {
                std::sync::Arc::new(Self {
                    stream: std::sync::Arc::new(std::sync::Mutex::new(Box::pin($stream_fn_name())))
                })
            }

            async fn poll_next(&self) -> Option<$inner_type> {
                let result = futures::executor::block_on(self.stream.lock().unwrap().as_mut().next());
                result
            }    
        }

        #[uniffi::export]
        pub fn $create_fn_name() -> std::sync::Arc<$struct_name> {
            $struct_name::new()
        }
    };
}