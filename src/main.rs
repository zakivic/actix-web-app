use std::thread::JoinHandle;
use std::{thread, time};

fn do_something(number: i8) -> i8 {
    println!("number {} is running", number);
    let two_seconds = time::Duration::new(2, 0);
    thread::sleep(two_seconds);
    return 2;
}

fn main() {
    let now = time::Instant::now();
    let thread_one: JoinHandle<i8> = thread::spawn(|| do_something(1));
    let thread_two: JoinHandle<i8> = thread::spawn(|| do_something(2));
    let thread_three: JoinHandle<i8> = thread::spawn(|| do_something(3));

    let result_one = thread_one.join();
    let result_two = thread_two.join();
    let result_three = thread_three.join();

    let handle_result = |thread_result: Result<i8, Box<dyn std::any::Any + Send>>, name: &str| {
        match thread_result {
            Ok(result) => {
                println!("the result for {} is {}", result, name);
                result
            }
            Err(result) => {
                if let Some(string) = result.downcast_ref::<String>() {
                    println!("the error for {} is: {}", name, string);
                } else {
                    println!("the error for {} does not have a message", name);
                }
                0
            }
        }
    };

    let res_one = handle_result(result_one, "thread_one");
    let res_two = handle_result(result_two, "thread_two");
    let res_three = handle_result(result_three, "thread_three");

    println!("time elapsed {:?}", now.elapsed());
    println!("result {}", res_one + res_two + res_three);
}
