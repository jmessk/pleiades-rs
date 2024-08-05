use anyhow::Result;

use pleiades::{api::*, Client};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    // create arc client
    let client = Client::builder()
        // .host("https://pleiades.dolylab.cc/api/v0.5-snapshot/")
        // .host("http://192.168.168.127:8332/api/v0.5/")
        // .host("http://172.21.39.32:8332/api/v0.5/")
        .host("http://pleiades.local:8332/api/v0.5/")
        .build()
        .unwrap();

    let worker_register = client
        .send(
            WorkerRegisterRequest::builder()
                .runtimes(vec!["test+mecrs".into()])
                .build(),
        )
        .await?;

    // to exit
    let mut count = 0;

    while count < 1 {
        println!("contracting...");
        let contracted = client
            .send(
                WorkerContractRequest::builder()
                    .worker_id(&worker_register.worker_id)
                    .timeout(5)
                    .build(),
            )
            .await?;

        if contracted.job_id.is_none() {
            count += 1;
            continue;
        }

        count = 0;

        let job_id = contracted.job_id.unwrap();
        let client = client.clone();
        tokio::spawn(async move { worker(client, job_id).await.unwrap() });
    }

    Ok(())
}

async fn worker(client: Client, job_id: String) -> Result<()> {
    // job info
    let request = JobInfoRequest::builder().job_id(&job_id).build();
    let job_info = client.send(request).await?;

    // download input
    let request = DataDownloadRequest::builder()
        .data_id(job_info.input.data_id)
        .build();
    let _ = client.send(request).await?;

    // output
    let request = DataUploadRequest::builder().data(b"").build();
    let output_blob = client.send(request).await?;

    // update job
    let request = JobUpdateRequest::builder()
        .job_id(job_info.job_id)
        .data_id(output_blob.data_id)
        .status("finished")
        .build();
    let _ = client.send(request).await?;

    println!("Job {} finished", job_id);

    Ok(())
}
