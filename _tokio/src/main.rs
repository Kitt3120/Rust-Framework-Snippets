/*
    Tokio is an implementation of an asynchronous runtime for the Rust programming language.
    It leverages Rust's async/await syntax to provide an easy-to-use asynchronous API.
    Tokio should be used to parallelize tasks that are mostly I/O-bound.
    Those are tasks that are waiting on I/O, like filesystem or network streams.
    For I/O, Tokio provides a set of asynchronous APIs that are similar to the ones provided by the standard library.
    For CPU-bound tasks, you should tell Tokio about the task being CPU-bound so that it can schedule it differently.
    Use the tokio::task::spawn_blocking function to do that.
    But the better approach is to use the Rayon crate for parallelizing CPU-bound tasks.

    To add Tokio to your project, run the following command:
    cargo add tokio --features full

    You can disable features that you don't need later on by editing your Cargo.toml file.
*/

use std::time::Duration;

use tokio::{join, select, time::sleep};

#[tokio::main] // Add the tokio::main attribute to the main function and mark it as async.
async fn main() {
    // Awaiting async functions
    println!("Awaiting function...");
    awaiting_function().await;
    println!("Done!");

    println!();

    // Async functions return a Future
    println!("Creating future");
    let future = awaiting_function();
    println!("Awaiting future...");
    future.await;
    println!("Done!");

    println!();

    // Futures are lazy
    let future = say_hi();
    println!("Sleeping on main function...");
    sleep(Duration::from_secs(1)).await;
    println!("This will still be printed before Hi");
    future.await; // This means that the future will only be executed when awaited

    println!();

    // If you want to execute a future in the background, you can use the spawn function
    let future = say_hi();
    println!("Running future as task in the background");
    let handle = tokio::spawn(future);
    println!("Sleeping on main function...");
    sleep(Duration::from_secs(1)).await;
    println!("This will be printed after Hi");
    println!("Waiting for task to finish...");
    handle.await.unwrap();
    println!("Done!");

    println!();

    // To await multiple futures, you can use the join macro
    let future1 = say_hi();
    let future2 = awaiting_function();
    println!("Awaiting multiple futures...");
    join!(future1, future2);
    println!("Done!");

    println!();

    // To await the first future to finish, you can use the select macro
    let future1 = say_hi();
    let future2 = awaiting_function();
    println!("Awaiting first future to finish...");
    select! {
        _ = future1 => println!("Future 1 finished first"),
        _ = future2 => println!("Future 2 finished first"),
    }
}

// This is the same as doing the following:
fn _manual_main() {
    tokio::runtime::Runtime::new()
        .unwrap()
        .block_on(awaiting_function());
}

async fn awaiting_function() {
    sleep(Duration::from_secs(1)).await;
}

async fn say_hi() {
    println!("Hi");
}
