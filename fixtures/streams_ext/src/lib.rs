use uniffi_dart::*;
use futures::stream::{self, Stream, StreamExt};
use async_stream::stream;
use tokio::time::{interval, Duration};
use std::pin::Pin;

#[uniffi_dart::export_stream(i32)]
pub fn simple_stream() -> impl Stream<Item = i32> {
    stream::iter(0..5)
}


#[uniffi_dart::export_stream(i32)]
pub fn count_stream() -> Pin<Box<dyn Stream<Item = i32> + Send>> {
    Box::pin(stream::iter(0..5))
}

#[uniffi_dart::export_stream(u64)]
pub fn fibonacci_stream() -> Pin<Box<dyn Stream<Item = u64> + Send>> {
    Box::pin(stream! {
        let mut a = 0;
        let mut b = 1;
        loop {
            yield a;
            let next = a + b;
            a = b;
            b = next;
        }
    })
}

// pub fn alphabet_stream() -> Pin<Box<dyn Stream<Item = String> + Send>> {
//     Box::pin(stream::iter('A'..='Z'))
// }

#[uniffi_dart::export_stream(u64)]
pub fn async_timer_stream() -> Pin<Box<dyn Stream<Item = u64> + Send>> {
    Box::pin(stream! {
        let mut interval = interval(Duration::from_secs(1));
        let mut count = 0;
        loop {
            interval.tick().await;
            count += 1;
            yield count;
        }
    })
}

#[uniffi_dart::export_stream(String)]
pub fn combined_streams() -> Pin<Box<dyn Stream<Item = String> + Send>> {
    let stream1 = count_stream().map(|n| format!("Count: {}", n));
   // let stream2 = alphabet_stream().map(|c| format!("Letter: {}", c));
    let stream3 = fibonacci_stream().take(5).map(|n| format!("Fibonacci: {}", n));

    Box::pin(stream::select(stream1, stream3))
}


// create_stream_ext!(
//     SimpleStreamExt,
//     i32,
//     create_simple_stream,
//     simple_stream
// );

// // CountStreamExt
// create_stream_ext!(
//     CountStreamExt,
//     i32,
//     create_count_stream,
//     count_stream
// );

// // AlphabetStreamExt
// // create_stream_ext!(
// //     AlphabetStreamExt,
// //     String,
// //     create_alphabet_stream,
// //     alphabet_stream
// // );

// // FibonacciStreamExt
// create_stream_ext!(
//     FibonacciStreamExt,
//     u64,
//     create_fibonacci_stream,
//     fibonacci_stream
// );

// // AsyncTimerStreamExt
// create_stream_ext!(
//     AsyncTimerStreamExt,
//     u64,
//     create_async_timer_stream,
//     async_timer_stream
// );

// // CombinedStreamsExt
// create_stream_ext!(
//     CombinedStreamsExt,
//     String,
//     create_combined_streams,
//     combined_streams
// );

#[cfg(test)]
mod tests {
    use super::*;
    use futures::stream::StreamExt;
    use tokio::time::timeout;
    use std::time::Duration;
    use tokio::runtime::Runtime;

    #[tokio::test]
    async fn test_simple_stream() {
        let result: Vec<i32> = simple_stream().collect().await;
        assert_eq!(result, vec![0, 1, 2, 3, 4]);
    }

    #[tokio::test]
    async fn test_count_stream() {
        let result: Vec<i32> = count_stream().collect().await;
        assert_eq!(result, vec![0, 1, 2, 3, 4]);
    }

    // #[tokio::test]
    // async fn test_alphabet_stream() {
    //     let result: String = alphabet_stream().collect().await;
    //     assert_eq!(result, "ABCDEFGHIJKLMNOPQRSTUVWXYZ");
    // }

    #[tokio::test]
    async fn test_fibonacci_stream() {
        let result: Vec<u64> = fibonacci_stream().take(10).collect().await;
        assert_eq!(result, vec![0, 1, 1, 2, 3, 5, 8, 13, 21, 34]);
    }

    #[tokio::test]
    async fn test_async_timer_stream() {
        let mut stream = async_timer_stream();
        let result = timeout(Duration::from_secs(3), async {
            let mut values = Vec::new();
            for _ in 0..3 {
                if let Some(value) = stream.next().await {
                    values.push(value);
                }
            }
            values
        }).await.expect("Timeout occurred");

        assert_eq!(result, vec![1, 2, 3]);
    }

    #[tokio::test]
    async fn test_combined_streams() {
        let result: Vec<String> = combined_streams().take(15).collect().await;
        
        // Check if we have the correct number of items
        assert_eq!(result.len(), 10);

        // Check if we have items from all three streams
        assert!(result.iter().any(|s| s.starts_with("Count:")));
        assert!(result.iter().any(|s| s.starts_with("Fibonacci:")));

        // Check specific items
        assert!(result.contains(&"Count: 0".to_string()));
        assert!(result.contains(&"Fibonacci: 3".to_string()));
    }

    #[test]
    fn test_poll_next() {
        let rt = Runtime::new().unwrap();
        let instance = create_stream_count_stream();

        rt.block_on(async {
            let mut results = Vec::new();
            for _ in 0..5 {
                if let Some(value) = instance.poll_next().await {
                    results.push(value);
                }
            }

            assert_eq!(results, vec![0, 1, 2, 3, 4]);
        });
    }

    #[test]
    fn test_multiple_streams() {
        let rt = Runtime::new().unwrap();
        let instance1 = create_stream_count_stream();
        let instance2 = create_stream_count_stream();

        rt.block_on(async {
            let result1 = instance1.poll_next().await;
            let result2 = instance2.poll_next().await;

            // Both instances should return the first item (0)
            assert_eq!(result1, Some(0));
            assert_eq!(result2, Some(0));

            // The next call should return the second item (1) for both instances
            let result3 = instance1.poll_next().await;
            let result4 = instance2.poll_next().await;

            assert_eq!(result3, Some(1));
            assert_eq!(result4, Some(1));
        });
    }

    #[test]
    fn test_stream_exhaustion() {
        let rt = Runtime::new().unwrap();
        let instance = create_stream_count_stream();

        rt.block_on(async {
            // Consume all items
            for _ in 0..5 {
                print!("{:?}", instance.poll_next().await);
            }

            // The next call should return None
            assert_eq!(instance.poll_next().await, None);
        });
    }
}

uniffi::include_scaffolding!("api");