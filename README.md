# mecrm-rs

MEC-RM client library for Rust.

```rust
let client = pleiades::Client::builder()
    .host("https://pleiades.dolylab.cc/api/v0.5-snapshot/")
    .build()?;

// create lambda
let lambda = {
    let code = client.new_blob("example lambda").await?;
    let runtime = pleiades::Runtime::new("mecrm-rs+example");

    code.into_lambda(runtime).await?
};

// create input
let input = client.new_blob("example input").await?;

let job = lambda
    .invoke(input)
    .await?
    .wait_finished(10)
    .await?;

// get output
let output = job.output.fetch().await?;
println!("{:?}", output);
```
