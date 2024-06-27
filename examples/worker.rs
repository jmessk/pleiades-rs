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

    let worker_register = WorkerRegisterRequest::builder()
        .runtimes(vec!["mecrm-rs".into()])
        .build()
        .send(client.client(), client.host())
        .await?;

    // to exit
    let mut count = 0;

    while count < 2 {
        println!("contracting...");
        let contracted = WorkerContractRequest::builder()
            .worker_id(&worker_register.worker_id)
            .timeout(5)
            .build()
            .send(client.client(), client.host())
            .await?;

        if contracted.job_id.is_none() {
            count += 1;
            continue;
        }

        count = 0;

        let job_id = contracted.job_id.unwrap();
        let client = client.clone();
        tokio::spawn(async move {
            worker(client, job_id).await.unwrap();
        });
    }

    Ok(())
}

async fn worker(client: Arc<Client>, job_id: String) -> Result<()> {
    let job_info = JobInfoRequest::builder()
        .job_id(&job_id)
        .build()
        .send(client.client(), client.host())
        .await?;

    // dbg!(&job_info);

    let _ = DataDownloadRequest::builder()
        .data_id(job_info.input.data_id)
        .build()
        .send(client.client(), client.host())
        .await?;

    // dbg!(&input_blob);

    let output_blob = DataUploadRequest::builder()
        .data(b"")
        .build()
        .send(client.client(), client.host())
        .await?;

    // dbg!(&output_blob);

    let _ = JobUpdateRequest::builder()
        .job_id(job_info.job_id)
        .data_id(output_blob.data_id)
        .status("finished")
        .build()
        .send(client.client(), client.host())
        .await?;

    println!("Job {} finished", job_id);

    Ok(())
}
