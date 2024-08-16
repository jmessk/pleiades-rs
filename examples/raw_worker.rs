use pleiades::{api, Client};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::fmt()
        .with_max_level(tracing::Level::ERROR)
        .init();

    // create arc client
    let client = Client::builder()
        // .host("https://pleiades.dolylab.cc/api/v0.5-snapshot/")
        // .host("http://192.168.168.127:8332/api/v0.5/")
        // .host("http://172.21.39.32:8332/api/v0.5/")
        // .host("http://pleiades.local:8332/api/v0.5/")
        .host("http://master.local/api/v0.5/")
        .build();

    let register = {
        let request = api::WorkerRegisterRequest::builder()
            .runtimes(&["mecrm-rs+example"])
            .build();

        client.send(&request).await?
    };

    let contract = api::WorkerContractRequest::builder()
        .worker_id(register.worker_id)
        .timeout(10)
        .build();

    while let Some(job_id) = client.send(&contract).await?.job_id {
        let client = client.clone();
        tokio::spawn(async move { worker(client.clone(), job_id).await.unwrap() });
    }

    println!("No more job");

    Ok(())
}

async fn worker(client: Client, job_id: String) -> anyhow::Result<()> {
    let job_info = {
        let request = api::JobInfoRequest::builder().job_id(&job_id).build();
        client.send(&request).await?
    };

    let _input = {
        let request = api::DataDownloadRequest::builder()
            .data_id(job_info.input.data_id)
            .build();

        client.send(&request).await?
    };

    let output = {
        let request = api::DataUploadRequest::builder()
            .data("example output")
            .build();

        client.send(&request).await?
    };

    let _update = {
        let request = api::JobUpdateRequest::builder()
            .job_id(&job_id)
            .data_id(output.data_id)
            .status("finished")
            .build();

        client.send(&request).await?
    };

    println!("Job {} finished", job_id);

    Ok(())
}
