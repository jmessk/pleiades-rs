# mecrm-rs

MEC-RM client library for Rust.

## Usage

## Requester

Refer to [simple_requester.rs](./examples/simple_requester.rs).

```rust:simple_requester.rs
let client = pleiades::Client::builder()
    .host("https://pleiades.dolylab.cc/api/v0.5-snapshot/")
    .build();

// create lambda
let lambda = {
    // create lambda code as blob
    let code = client.blob().new("example lambda").await?;

    // create lambda from code blob
    code.into_lambda("mecrm-rs+example").await?
};

// create input
let input = client.blob().new("example input").await?;

let job = lambda
    // run job
    .invoke(input)
    .await?
    // wait job finished
    .wait_finished(10)
    .await?;

// get output
let output = job.output.fetch().await?;
println!("output: {:?}", output);
```
