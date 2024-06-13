use std::env;
use std::sync::Arc;
use std::time::Instant;

use mecrm_rs::api::*;
use mecrm_rs::Client;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // create arc client
    let client = Arc::new(
        Client::builder()
            // .host("https://mecrm.dolylab.cc/api/v0.5-snapshot/")
            .host("http://192.168.168.127:8332/api/v0.5/")
            .build()?,
    );

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
                requester(client).await;
            })
        })
        .collect::<Vec<_>>();

    futures::future::join_all(tasks).await;
    Ok(())
}

async fn requester(client: Arc<Client>) {
    // let start = Instant::now();

    let lambda_blob = DataUploadRequest::builder()
        .data("".into())
        .build()
        .unwrap()
        .send(client.clone())
        .await
        .unwrap();

    let lambda = LambdaCreateRequest::builder()
        .data_id(&lambda_blob.data_id)
        // .runtime("mecrm-rs")
        .runtime("bench+pymec")
        .build()
        .unwrap()
        .send(client.clone())
        .await
        .unwrap();

    let input_blob = DataUploadRequest::builder()
        .data("".into())
        .build()
        .unwrap()
        .send(client.clone())
        .await
        .unwrap();

    let job = JobCreateRequest::builder()
        .lambda_id(lambda.lambda_id)
        .data_id(&input_blob.data_id)
        .build()
        .unwrap()
        .send(client.clone())
        .await
        .unwrap();

    let job_info = JobInfoRequest::builder()
        .job_id(&job.job_id)
        .except("Finished")
        .timeout(10)
        .build()
        .unwrap()
        .send(client.clone())
        .await
        .unwrap();

    let start = Instant::now();
    let _ = DataDownloadRequest::builder()
        .data_id(&job_info.output.unwrap().data_id)
        .build()
        .unwrap()
        .send(client.clone())
        .await
        .unwrap();

    println!("elapsed: {:?}", start.elapsed().as_secs_f64() * 1000.0);

    // println!("elapsed: {:?}", start.elapsed());
}
