# rusty-hook
Custom Curl learning from https://dev.to/chaudharypraveen98/build-your-own-curl-rust-5cj6


https://github.com/chaudharypraveen98/ccurl/blob/master/src/main.rs


## From Copilot: Prepare & build (Windows - CLI)

These commands show a minimal sequence to prepare, build and run this Rust project from PowerShell / Command Prompt on Windows.

1. Verify or install Rust (rustup)
- If you already have Rust: check rustup and cargo
  ```powershell
  rustup --version
  cargo --version
  ```
- If not installed, follow the instructions at https://rustup.rs to install the Rust toolchain for Windows.

2. Ensure toolchain components for formatting/linting (optional but recommended)
```powershell
rustup toolchain install stable
rustup default stable
rustup component add rustfmt
rustup component add clippy
```

3. From your project root, change directory to the project
```powershell
cd "c:\dev\rust\curl\rusty-hook"
```

4. Make sure required crates are listed in Cargo.toml
Add these dependencies if they are not present (edit Cargo.toml or use cargo-edit):
```toml
[dependencies]
clap = { version = "4", features = ["derive"] }
lazy_static = "1.4"
```
(If you prefer `cargo add` and have cargo-edit installed:)
```powershell
cargo add clap --features derive
cargo add lazy_static
```

5. Build the project
- debug build:
```powershell
cargo build
```
- release build:
```powershell
cargo build --release
```

6. Run the binary with example arguments
- Basic GET:
```powershell
cargo run -- http://example.com/
```
- Verbose GET:
```powershell
cargo run -- http://example.com/ -v
```
- POST with data and headers:
```powershell
cargo run -- http://httpbin.org/post -X POST -d '{"hello":"world"}' -H "Content-Type: application/json" -v
```
(The `--` after `cargo run` separates cargo args from program args.)

7. Format and lint (optional)
```powershell
cargo fmt
cargo clippy
```

8. Troubleshooting notes
- If `cargo build` fails, read the compiler output; it points to the file and line.
- Known issues in the current source (quick pointers to look at):
  - main.rs: `mod contants;` should be `mod constants;`
  - cli.rs: imports and names (e.g. `ArgMatches`, `ArgAction`) and the final `get_matches()` call must match the clap API
  - Ensure the `constants` module is referenced correctly from main.rs
- After applying fixes, run `cargo build` again.

If you want, I can produce the exact Cargo.toml snippet or apply the small source fixes to make the project compile.// filepath: c:\dev\rust\curl\rusty-hook\README.md