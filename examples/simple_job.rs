use std::sync::Arc;

use mecrs::api::*;
use mecrs::Client;

#[tokio::main]
async fn main() {
    let client_arc = Arc::new(
        Client::builder()
            // .host("https://mecrm.dolylab.cc/api/v0.5-snapshot/")
            // .host("http://192.168.168.127:8332/api/v0.5/")
            .host("http://172.21.39.32:8332/api/v0.5/")
            .build(),
    );

    let client = client_arc.clone();

    let requester_task = tokio::spawn(async move {
        let start = std::time::Instant::now();

        let blob = DataUploadRequest::builder()
            .data(b"input")
            .build()
            .send(&client)
            .await
            .unwrap();

        dbg!(&blob);

        let lambda = LambdaCreateRequest::builder()
            .data_id(&blob.data_id)
            .runtime("mecrm-rs")
            .build()
            .send(&client)
            .await
            .unwrap();

        dbg!(&lambda);

        let job = JobCreateRequest::builder()
            .data_id(&blob.data_id)
            .lambda_id(lambda.lambda_id)
            .build()
            .send(&client)
            .await
            .unwrap();

        dbg!(&job);

        let job_info = JobInfoRequest::builder()
            .job_id(&job.job_id)
            .except("Finished")
            .timeout(10)
            .build()
            .send(&client)
            .await
            .unwrap();

        dbg!(&job_info);

        let output_blob = DataDownloadRequest::builder()
            .data_id(&job_info.output.unwrap().data_id)
            .build()
            .send(&client)
            .await
            .unwrap();

        dbg!(output_blob);

        println!("Elapsed: {:?}", start.elapsed());
    });

    let client = client_arc.clone();

    let worker_task = tokio::spawn(async move {
        let worker = WorkerRegisterRequest::builder()
            .runtimes(vec!["mecrm-rs".into()])
            .build()
            .send(&client)
            .await
            .unwrap();

        dbg!(&worker);

        let job = WorkerContractRequest::builder()
            .worker_id(&worker.worker_id)
            .timeout(10)
            .build()
            .send(&client)
            .await
            .unwrap();

        dbg!(&job);

        let job_info = JobInfoRequest::builder()
            .job_id(&job.job_id.unwrap())
            .build()
            .send(&client)
            .await
            .unwrap();

        dbg!(&job_info);

        let input_blob = DataDownloadRequest::builder()
            .data_id(job_info.input.data_id)
            .build()
            .send(&client)
            .await
            .unwrap();

        dbg!(&input_blob);

        let output_blob = DataUploadRequest::builder()
            .data(b"output")
            .build()
            .send(&client)
            .await
            .unwrap();

        dbg!(&output_blob);

        let job_update = JobUpdateRequest::builder()
            .job_id(job_info.job_id)
            .data_id(output_blob.data_id)
            .status("finished")
            .build()
            .send(&client)
            .await
            .unwrap();

        dbg!(job_update);
    });

    requester_task.await.unwrap();
    worker_task.await.unwrap();
}
