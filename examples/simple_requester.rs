use std::env;
use std::sync::Arc;

use anyhow::Result;
use pleiades::api::*;
use pleiades::Client;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("debug"));

    // create arc client
    let client = Arc::new(
        Client::builder()
            // .host("https://pleiades.dolylab.cc/api/v0.5-snapshot/")
            // .host("http://192.168.168.127:8332/api/v0.5/")
            // .host("http://172.21.39.32:8332/api/v0.5/")
            .host("http://pleiades.local:8332/api/v0.5/")
            .build(),
    );

    // job_num is the number of jobs to be created
    // if not provided, default to 10
    // Usage: ./requester <job_num>
    let job_num: usize = {
        let args: Vec<String> = env::args().collect();
        if 3 < args.len() {
            eprintln!("Usage: {}", args[0]);
            eprintln!("Usage: {} <job_num>", args[0]);
            std::process::exit(1);
        }
        if args.len() == 1 {
            1
        } else {
            args[1].parse::<usize>().unwrap()
        }
    };

    let tasks = (0..job_num)
        .map(|_| {
            let client = client.clone();
            tokio::spawn(async move { requester(client).await.unwrap() })
        })
        .collect::<Vec<_>>();

    futures::future::join_all(tasks).await;
    Ok(())
}

async fn requester(client: Arc<Client>) -> Result<()> {
    // lambda blob
    let lambda_blob = client
        .request(DataUploadRequest::builder().data(b"").build())
        .await?;

    // lambda
    let lambda = client
        .request(
            LambdaCreateRequest::builder()
                .data_id(lambda_blob.data_id)
                .runtime("test+mecrs")
                .build(),
        )
        .await?;

    // input blob
    let input_blob = client
        .request(DataUploadRequest::builder().data(b"").build())
        .await?;

    // create job
    let job_create = client
        .request(
            JobCreateRequest::builder()
                .lambda_id(lambda.lambda_id)
                .data_id(input_blob.data_id)
                .build(),
        )
        .await?;

    // wait for finish
    let job_info = client
        .request(
            JobInfoRequest::builder()
                .job_id(job_create.job_id)
                .except("Finished")
                .timeout(10)
                .build(),
        )
        .await?;

    // download output
    let _ = client
        .request(
            DataDownloadRequest::builder()
                .data_id(job_info.output.unwrap().data_id)
                .build(),
        )
        .await?;

    Ok(())
}
