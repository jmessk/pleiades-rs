use std::sync::Arc;
use pleiades_api::{api, Client};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = Arc::new(Client::default());

    let register = {
        let request = api::worker::register::Request::builder()
            .runtimes(&["mecrm-rs+example"])
            .build();
        client.call_api(&request).await?
    };

    let contract = api::worker::contract::Request::builder()
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

async fn worker(client: Arc<Client>, job_id: String) -> anyhow::Result<()> {
    let job_info = {
        let request = api::job::info::Request::builder().job_id(&job_id).build();
        client.call_api(&request).await?
    };

    let _input = {
        let request = api::data::download::Request::builder()
            .data_id(job_info.input.data_id)
            .build();
        client.call_api(&request).await?
    };

    let output = {
        let request = api::data::upload::Request::builder()
            .data("hello, world!")
            .build();
        client.call_api(&request).await?
    };

    let _update = {
        let request = api::job::update::Request::builder()
            .job_id(&job_id)
            .data_id(output.data_id)
            .status("finished")
            .build();
        client.call_api(&request).await?
    };

    println!("Job {} finished", job_id);

    Ok(())
}
