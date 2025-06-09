# pleiades-rs

Pleiades client library for Rust.

## Features

- **pleiades-api** - HTTP client library for [Pleiades Core](https://git.short-circuits.org/pleiades/pleiades-core) API
- **pleiades** - Wrapper library of pleiades-api for easy to use (now alpha)

## Usage

### Requester

Refer to [simple_requester.rs](./pleiades/examples/simple_requester.rs).

```rust
let input = client.blob().new(r#"{"a":3,"b":5}"#).await?;

// create job
let job = lambda.invoke(input, None).await?;

// wait job finished
let job = job.wait_finished(Duration::from_secs(10)).await?;

// get output
let output = job.output.fetch().await?;
println!("finished. output: {:?}", output);
```
