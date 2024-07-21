Build Run (Run will also build)
---
```bash
cargo build
cargo run
```

Clean and Refresh Dependencies (if you start changing versions of things in Cargo.toml)
---
```bash
cargo clean
cargo fetch
cargo build
```

WASM
---
```bash
rustup target add wasm32-unknown-unknown
cargo build --target wasm32-unknown-unknown
```

Running a Local Server for Testing
---
```bash
cargo install basic-http-server
basic-http-server .
```

This has info on building for Ipad / Iphone
---
https://github.com/not-fl3/macroquad
