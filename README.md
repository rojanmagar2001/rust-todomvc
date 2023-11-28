On _Windows_ for the "link" file issue when running the cargo test and run in parallel. The solution is to move `tests/quick_dev.rs` to the `examples` folder, rename the function to `#[tokio::main]`, and it should allow you to do the following:

- In Terminal 1: `cargo watch -q -c -w src/ -x run`
- In Terminal 2: `cargo watch -q -c -w examples/ -x 'run --example quick_dev'`
