use flutter_rust_bridge::DartFnFuture;
use std::panic::UnwindSafe;

// this function will return but println won't output anything to stdout
#[flutter_rust_bridge::frb(sync)] // Synchronous mode for simplicity of the demo
pub fn greet(name: String) -> String {
    println!("RUST: this won't get printed, sync greet received name: {name}");
    format!("sync greet: Hello, {name}!")
}

// this function will never return and both println and dart logger won't output anything to stdout
pub async fn async_greet(name: String, logger: impl Fn(String) -> DartFnFuture<()> + UnwindSafe) -> anyhow::Result<String> {
    println!("RUST: this won't get printed, async greet received name: {name}");
    logger("DART FROM RUST: this won't get printed, async greet received name: {name}".to_owned()).await;
    Ok(format!("async_greet: Hello, {name}!"))
}

// this function will return but both println and dart logger won't print anything
pub async fn async_no_await_greet(name: String, logger: impl Fn(String) -> DartFnFuture<()> + UnwindSafe) -> anyhow::Result<String> {
    println!("RUST: this won't get printed, async greet received name: {name}");
    logger("DART FROM RUST: this won't get printed, async greet received name: {name}".to_owned());
    Ok(format!("async_no_await_greet: Hello, {name}!"))
}
