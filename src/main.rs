use async_std;
use futures::executor::block_on;
use futures::future::join_all;
use std::vec::Vec;
use std::{thread, time};

async fn do_something(number: i8) -> i8 {
    println!("number {} is running", number);
    let two_seconds = time::Duration::new(2, 0);
    thread::sleep(two_seconds);
    return 2;
}
fn main() {
    let async_outcome = async {
        // 1.
        let mut futures_vec = Vec::new();
        let future_four = do_something(4);
        let future_five = do_something(5);
        // 2.
        futures_vec.push(future_four);
        futures_vec.push(future_five);
        // 3.
        let handles = futures_vec
            .into_iter()
            .map(async_std::task::spawn)
            .collect::<Vec<_>>();
        // 4.
        let results = join_all(handles).await;
        return results.into_iter().sum::<i8>();
    };
    let now = time::Instant::now();
    let result = block_on(async_outcome);
    println!("time elapsed for join vec {:?}", now.elapsed());
    println!("Here is the result: {:?}", result);
}
