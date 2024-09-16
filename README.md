# mecrm-rs

MEC-RM client library for Rust.

## Usage

### Requester

Refer to [simple_requester.rs](./examples/simple_requester.rs).

```rust:simple_requester.rs
    let input = client.blob().new(r#"{"a":3,"b":5}"#).await?;

    // create job
    let job = lambda.invoke(input, &[]).await?;

    // wait job finished
    let job = job.wait_finished(Duration::from_secs(10)).await?;

    // get output
    let output = job.output.fetch().await?;
    println!("finished. output: {:?}", output);
```
