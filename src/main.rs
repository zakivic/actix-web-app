use futures::executor::block_on;
use futures::join;
use std::{thread, time};

async fn do_something(number: i8) -> i8 {
    println!("number {} is running", number);
    let two_seconds = time::Duration::new(2, 0);
    thread::sleep(two_seconds);
    return 2;
}
fn main() {
    let now = time::Instant::now();
    let future_tree = async {
        let outcome_one = do_something(1);
        let outcome_two = do_something(2);
        let results = join!(outcome_one, outcome_two);
        return results.0 + results.1;
    };

    let outcome = block_on(future_tree);
    println!("time elapsed {:?}", now.elapsed());
    println!("Here is the outcome: {}", outcome);
}
