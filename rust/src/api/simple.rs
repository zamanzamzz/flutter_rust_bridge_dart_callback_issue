use flutter_rust_bridge::DartFnFuture;
use std::panic::UnwindSafe;
use std::{thread, time};

async fn random_async() -> anyhow::Result<()> {
    println!("RUST: sleeping");
    thread::sleep(time::Duration::from_millis(400));
    println!("RUST: finished sleeping");
    Ok(())
}

// this function will return but println won't output anything to stdout
#[flutter_rust_bridge::frb(sync)] // Synchronous mode for simplicity of the demo
pub fn greet(name: String) -> String {
    println!("RUST: this won't get printed, sync_greet received name: {name}");
    format!("sync_greet: Hello, {name}!")
}

// this function will never return on Android SDK <=30 and both println and dart logger won't output anything to stdout
pub async fn async_greet_with_callback(name: String, logger: impl Fn(String) -> DartFnFuture<()> + UnwindSafe) -> anyhow::Result<String> {
    println!("RUST: this won't get printed, async_greet_with_callback received name: {name}");
    logger(format!("DART FROM RUST: this won't get printed, async_greet_with_callback received name: {name}").to_owned()).await;
    Ok(format!("async_greet_with_callback: Hello, {name}!"))
}

// this function will never return on Android SDK <=30 and both println and dart logger won't output anything to stdout
pub async fn async_greet(name: String) -> anyhow::Result<String> {
    println!("RUST: this won't get printed, async greet received name: {name}");
    let _ = random_async().await;
    Ok(format!("async_greet: Hello, {name}!"))
}

// this function will return but both println and dart logger won't print anything
pub async fn async_no_await_greet(name: String, logger: impl Fn(String) -> DartFnFuture<()> + UnwindSafe) -> anyhow::Result<String> {
    println!("RUST: this won't get printed, async_no_await_greet received name: {name}");
    logger(format!("DART FROM RUST: this won't get printed, async_no_await_greet received name: {name}").to_owned());
    Ok(format!("async_no_await_greet: Hello, {name}!"))
}
