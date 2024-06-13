use std::sync::Arc;

use mecrm_rs::api::*;
use mecrm_rs::Client;

#[tokio::main]
async fn main() {
    let client_arc = Arc::new(
        Client::builder()
            // .host("https://mecrm.dolylab.cc/api/v0.5-snapshot/")
            .host("http://192.168.168.127:8332/api/v0.5/")
            .build()
            .unwrap(),
    );

    let client = client_arc.clone();

    let task = tokio::spawn(async move {
        use std::time::Instant;
        let start = Instant::now();

        let blob = DataUploadRequest::builder(client.clone())
            .data("input".into())
            .build()
            .unwrap()
            .send()
            .await
            .unwrap();

        dbg!(&blob);

        let lambda = LambdaCreateRequest::builder(client.clone())
            .data_id(&blob.data_id)
            .runtime("mecrm-rs")
            .build()
            .unwrap()
            .send()
            .await
            .unwrap();

        dbg!(&lambda);

        let job = JobCreateRequest::builder(client.clone())
            .data_id(&blob.data_id)
            .lambda_id(lambda.lambda_id)
            .build()
            .unwrap()
            .send()
            .await
            .unwrap();

        dbg!(&job);

        let job_info = JobInfoRequest::builder(client.clone())
            .job_id(&job.job_id)
            .except("Finished")
            .timeout(10)
            .build()
            .unwrap()
            .send()
            .await
            .unwrap();

        dbg!(&job_info);

        let output_blob = DataDownloadRequest::builder(client.clone())
            .data_id(&job_info.output.unwrap().data_id)
            .build()
            .unwrap()
            .send()
            .await
            .unwrap();

        dbg!(output_blob);

        println!("Elapsed: {:?}", start.elapsed());
    });

    let client = client_arc.clone();

    tokio::spawn(async move {
        let worker = WorkerRegisterRequest::builder(client.clone())
            .runtimes(vec!["mecrm-rs".into()])
            .build()
            .unwrap()
            .send()
            .await
            .unwrap();

        dbg!(&worker);

        let job = WorkerContractRequest::builder(client.clone())
            .worker_id(&worker.worker_id)
            .timeout(10)
            .build()
            .unwrap()
            .send()
            .await
            .unwrap();

        dbg!(&job);

        let job_info = JobInfoRequest::builder(client.clone())
            .job_id(&job.job_id.unwrap())
            .build()
            .unwrap()
            .send()
            .await
            .unwrap();

        dbg!(&job_info);

        let input_blob = DataDownloadRequest::builder(client.clone())
            .data_id(job_info.input.data_id)
            .build()
            .unwrap()
            .send()
            .await
            .unwrap();

        dbg!(&input_blob);

        let output_blob = DataUploadRequest::builder(client.clone())
            .data("output".into())
            .build()
            .unwrap()
            .send()
            .await
            .unwrap();

        dbg!(&output_blob);

        let _ = JobUpdateRequest::builder(client.clone())
            .job_id(job_info.job_id)
            .data_id(output_blob.data_id)
            .status("finished")
            .job_status("finished")
            .build()
            .unwrap()
            .send()
            .await
            .unwrap();
    })
    .await
    .unwrap();

    task.await.unwrap();
}
