# Backend: Initial Design

```
cargo new backend
```

[AWS SDK for Rust](https://awslabs.github.io/aws-sdk-rust/)

[aws-sdk-bedrockagentruntime](https://crates.io/crates/aws-sdk-bedrockagentruntime)

[tokyo](https://crates.io/crates/tokio)

```toml
[dependencies]
aws-config = { version = "1.1.7", features = ["behavior-version-latest"] }
aws-sdk-bedrockagentruntime = "1.67.0"
tokio = { version = "1", features = ["full"] }
```

```
cargo install cargo-audit
```

```
./run-cmd-in-shell.sh make debug
```