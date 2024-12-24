# Backend: Initial Design

---
## Framework

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

```
✗ make qa
rustfmt -l src/*.rs
error[E0670]: `async fn` is not permitted in Rust 2015
  --> ~/go/src/github.com/seafoodfry/kamekai/backend/src/main.rs:52:1
   |
52 | async fn main() -> Result<(), BedrockConverseError> {
   | ^^^^^ to use `async fn`, switch to Rust 2018 or later
   |
   = help: pass `--edition 2021` to `rustc`
   = note: for more on editions, read https://doc.rust-lang.org/edition-guide

make: *** [fmt] Error 1
```

Solution was to create a `rustfmt.toml` with the following config:
```toml
edition = "2021"
```

Sample code over at
[Invoke Anthropic Claude on Amazon Bedrock using Bedrock's Converse API](https://docs.aws.amazon.com/bedrock/latest/userguide/bedrock-runtime_example_bedrock-runtime_Converse_AnthropicClaude_section.html)
is but the bare bones.
It needs many things, one of them is error handling.


---
## Error Handling

[crate thiserror](https://docs.rs/thiserror/latest/thiserror/)
first saw it in [Firecracker's Jailer](https://github.com/firecracker-microvm/firecracker/blob/main/src/jailer/src/main.rs)

[Rust: Structuring and handling errors in 202X](https://nick.groenen.me/posts/rust-error-handling/)

```rs
ServiceError(
    ServiceError {
        source: ValidationException(
            ValidationException {
                message: Some(
                    "Invocation of model ID anthropic.claude-3-5-sonnet-20241022-v2:0 with on-demand throughput isn’t supported. Retry your request with the ID or ARN of an inference profile that contains this model.",
                ),
                meta: ErrorMetadata {
                    code: Some(
                        "ValidationException",
                    ),
                    ...
```

There are inference profiles already available.
We had to go into the console and found them in the section about cross-region inference.

The message builder came about because we wanted to pass a system prompt:
```rs
Error: AWS Bedrock error: ServiceError(
    ServiceError {
        source: ValidationException(
            ValidationException {
                message: Some(
                    "A conversation must start with a user message. Try again with a conversation that starts with a user message.",
                ),
               ...
```