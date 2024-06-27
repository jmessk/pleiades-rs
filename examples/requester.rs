use std::env;
use std::sync::Arc;

use anyhow::Result;
use mecrs::api::*;
use mecrs::Client;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // create arc client
    let client = Arc::new(
        Client::builder()
            .host("https://mecrm.dolylab.cc/api/v0.5-snapshot/")
            // .host("http://192.168.168.127:8332/api/v0.5/")
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
            10
        } else {
            args[1].parse::<usize>().unwrap()
        }
    };

    let tasks = (0..job_num)
        .map(|_| {
            let client = client.clone();
            tokio::spawn(async move {
                requester(client).await.unwrap();
            })
        })
        .collect::<Vec<_>>();

    futures::future::join_all(tasks).await;
    Ok(())
}

async fn requester(client: Arc<Client>) -> Result<()> {
    // let start_job = Instant::now();

    let lambda_blob = DataUploadRequest::builder()
        .data(b"")
        .build()
        .send(client.client(), client.host())
        .await?;

    let lambda = LambdaCreateRequest::builder()
        .data_id(lambda_blob.data_id)
        .runtime("mecrs")
        .build()
        .send(client.client(), client.host())
        .await?;

    let input_blob = DataUploadRequest::builder()
        .data(b"")
        .build()
        .send(client.client(), client.host())
        .await?;

    let job_create = JobCreateRequest::builder()
        .lambda_id(lambda.lambda_id)
        .data_id(input_blob.data_id)
        .build()
        .send(client.client(), client.host())
        .await?;

    let job_info = JobInfoRequest::builder()
        .job_id(job_create.job_id)
        .except("Finished")
        .timeout(10)
        .build()
        .send(client.client(), client.host())
        .await?;

    let _ = DataDownloadRequest::builder()
        .data_id(job_info.output.unwrap().data_id)
        .build()
        .send(client.client(), client.host())
        .await?;

    Ok(())
}
