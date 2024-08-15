use pleiades::api;
use pleiades::Client;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("debug"));

    // create arc client
    let client = Client::builder()
        // .host("https://pleiades.dolylab.cc/api/v0.5-snapshot/")
        // .host("http://192.168.168.127:8332/api/v0.5/")
        // .host("http://172.21.39.32:8332/api/v0.5/")
        .host("http://pleiades.local:8332/api/v0.5/")
        .build()?;

    // blob as lambda code
    let code_blob = {
        let request = api::DataUploadRequest::builder()
            .data(r#"
                console.log("hello world");
            "#)
            .build();

        client.send(&request).await?
    };

    // lambda
    let lambda = {
        let request = api::LambdaCreateRequest::builder()
            .data_id(code_blob.data_id)
            .runtime("mecrm-rs+example")
            .build();

        client.send(&request).await?
    };

    // input blob
    let input = {
        let request = api::DataUploadRequest::builder()
            .data("example input")
            .build();

        client.send(&request).await?
    };

    // create job
    let create_job = {
        let request = api::JobCreateRequest::builder()
            .lambda_id(lambda.lambda_id)
            .data_id(input.data_id)
            .build();

        client.send(&request).await?
    };

    // wait for finish
    let job_info = {
        let request = api::JobInfoRequest::builder()
            .job_id(create_job.job_id)
            .except("Finished")
            .timeout(10)
            .build();

        client.send(&request).await?
    };

    // download output
    let _output = {
        let request = api::DataDownloadRequest::builder()
            .data_id(job_info.output.unwrap().data_id)
            .build();

        client.send(&request).await?
    };

    Ok(())
}
