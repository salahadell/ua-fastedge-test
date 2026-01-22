# FastEdge UA Parser Benchmark

Minimal FastEdge apps to benchmark device detection performance in real WASM environment.

## Structure

```
ua-fastedge-test/
├── rust-device-detector-test/    # Uses rust_device_detector crate
│   ├── Cargo.toml
│   ├── lib.rs
│   └── .cargo/config.toml
└── agent-parser-ro-test/         # Uses agent_parser_ro crate
    ├── Cargo.toml
    ├── lib.rs
    └── .cargo/config.toml
```

## Build

```bash
# Build rust_device_detector version
cd rust-device-detector-test
cargo build --release
# Output: target/wasm32-wasip1/release/rust_device_detector_test.wasm

# Build agent_parser_ro version
cd ../agent-parser-ro-test
cargo build --release
# Output: target/wasm32-wasip1/release/agent_parser_ro_test.wasm
```

