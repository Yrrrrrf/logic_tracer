# [Cargo](https://doc.rust-lang.org/cargo/index.html)


This file contains some of the most useful commands for cargo.
```bash	
cargo new <project_name> # Create a new project
cargo build # Build the project
cargo run # Build and run the project
cargo check # Check if the project compiles

cargo build --release # Build the project in release mode
cargo run --release # Build and run the project in release mode

cargo update # Update the dependencies
cargo doc --open # Generate the documentation and open it in the browser
# documentatio without dependencies
cargo doc --no-deps --open

cargo clean # Remove the target directory
cargo clean -p <project_name> # Remove the target directory of a specific project
```

Run code after every successful build
```bash
cargo watch -x 'run --bin <project_name>'
```

## Cargo watch
This command is used to run code after every successful build.
```bash
cargo install cargo-watch

cargo watch -x test  # Run tests only
cargo watch -x check -x test  # Run check then tests
cargo watch -x 'run -- --some-arg'  # Run run with arguments
cargo watch -- echo Hello world  # Run an arbitrary command
cargo watch --features "foo,bar"  # Run with features passed to cargo

cargo watch -x 'run --bin <project_name>'  # Run the project after every successful build

```
