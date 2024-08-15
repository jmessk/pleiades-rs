# MEC-rs

MEC-RM client library for Rust.

```rust
let client = pleiades::Client::builder()
    .host("https://pleiades.dolylab.cc/api/v0.5-snapshot/")
    .build()?;

// create lambda
let lambda = {
    // create lambda code as blob
    let code = client.new_blob("example lambda").await?;

    // define runtime
    let runtime = pleiades::Runtime::new("mecrm-rs+example");

    // create lambda from code blob
    code.into_lambda(runtime).await?
};
// or
// let lambda = client.lambda_from_id("<lambda_id>").await?;

// create input
let input = client.new_blob("example input").await?;
// or
// let input = client.blob_from_id("<data_id>");

let job = lambda
    // run job
    .invoke(input)
    .await?
    // wait job finished
    .wait_finished(10)
    .await?;

// get output
let output = job.output.fetch().await?;
println!("{:?}", output);
```
