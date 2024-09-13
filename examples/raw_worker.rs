use pleiades::{api, Client};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::fmt()
        .with_max_level(tracing::Level::ERROR)
        .init();

    let client = Client::builder()
        .host("http://pleiades.local/api/v0.5/")
        .build();

    let register = {
        let request = api::WorkerRegisterRequest::builder()
            .runtimes(&["mecrm-rs+example"])
            .build();

        client.call_api(&request).await?
    };

    let contract = api::WorkerContractRequest::builder()
        .worker_id(register.worker_id)
        .timeout(10)
        .build();

    while let Some(job_id) = client.call_api(&contract).await?.job_id {
        let client = client.clone();
        tokio::spawn(async move { worker(client.clone(), job_id).await.unwrap() });
    }

    println!("No more job");

    Ok(())
}

async fn worker(client: Client, job_id: String) -> anyhow::Result<()> {
    let job_info = {
        let request = api::JobInfoRequest::builder().job_id(&job_id).build();
        client.call_api(&request).await?
    };

    let _input = {
        let request = api::DataDownloadRequest::builder()
            .data_id(job_info.input.data_id)
            .build();

        client.call_api(&request).await?
    };

    let output = {
        let request = api::DataUploadRequest::builder()
            .data("example output")
            .build();

        client.call_api(&request).await?
    };

    let _update = {
        let request = api::JobUpdateRequest::builder()
            .job_id(&job_id)
            .data_id(output.data_id)
            .status("finished")
            .build();

        client.call_api(&request).await?
    };

    println!("Job {} finished", job_id);

    Ok(())
}
