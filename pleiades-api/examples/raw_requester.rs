use pleiades_api::{api, Client};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // client
    let client = Client::default();

    // blob as lambda code
    let code_blob = {
        let request = api::data::upload::Request::builder()
            .data(r#"console.log("hello, world!");"#,
            )
            .build();
        client.call_api(&request).await?
    };

    // lambda
    let lambda = {
        let request = api::lambda::create::Request::builder()
            .data_id(code_blob.data_id)
            .runtime("mecrm-rs+example")
            .build();
        client.call_api(&request).await?
    };

    // input blob
    let input = {
        let request = api::data::upload::Request::builder()
            .data("example input")
            .build();
        client.call_api(&request).await?
    };

    // create job
    let create_job = {
        let request = api::job::create::Request::builder()
            .lambda_id(lambda.lambda_id)
            .data_id(input.data_id)
            .build();
        client.call_api(&request).await?
    };

    // wait for finish
    let job_info = {
        let request = api::job::info::Request::builder()
            .job_id(create_job.job_id)
            .except("Finished")
            .timeout(10)
            .build();
        client.call_api(&request).await?
    };

    // download output
    let output = {
        let request = api::data::download::Request::builder()
            .data_id(job_info.output.unwrap().data_id)
            .build();
        client.call_api(&request).await?
    };

    println!("Job finished: {:?}", output);

    Ok(())
}
