use pleiades::api;
use pleiades::Client;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::fmt()
        .with_max_level(tracing::Level::ERROR)
        .init();

    let client = Client::builder()
        .host("http://pleiades.local/api/v0.5/")
        .build();

    // blob as lambda code
    let code_blob = {
        let request = api::DataUploadRequest::builder()
            .data(
                r#"
                console.tracing("hello world");
            "#,
            )
            .build();

        client.call_api(&request).await?
    };

    // lambda
    let lambda = {
        let request = api::LambdaCreateRequest::builder()
            .data_id(code_blob.data_id)
            .runtime("mecrm-rs+example")
            .build();

        client.call_api(&request).await?
    };

    // input blob
    let input = {
        let request = api::DataUploadRequest::builder()
            .data("example input")
            .build();

        client.call_api(&request).await?
    };

    // create job
    let create_job = {
        let request = api::JobCreateRequest::builder()
            .lambda_id(lambda.lambda_id)
            .data_id(input.data_id)
            .build();

        client.call_api(&request).await?
    };

    // wait for finish
    let job_info = {
        let request = api::JobInfoRequest::builder()
            .job_id(create_job.job_id)
            .except("Finished")
            .timeout(10)
            .build();

        client.call_api(&request).await?
    };

    // download output
    let _output = {
        let request = api::DataDownloadRequest::builder()
            .data_id(job_info.output.unwrap().data_id)
            .build();

        client.call_api(&request).await?
    };

    Ok(())
}
