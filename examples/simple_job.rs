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

    let requester_task = tokio::spawn(async move {
        use std::time::Instant;
        let start = Instant::now();

        let blob = DataUploadRequest::builder()
            .data("input".into())
            .build()
            .unwrap()
            .send(client.clone())
            .await
            .unwrap();

        // dbg!(&blob);

        let lambda = LambdaCreateRequest::builder()
            .data_id(&blob.data_id)
            .runtime("mecrm-rs")
            .build()
            .unwrap()
            .send(client.clone())
            .await
            .unwrap();

        // dbg!(&lambda);

        let job = JobCreateRequest::builder()
            .data_id(&blob.data_id)
            .lambda_id(lambda.lambda_id)
            .build()
            .unwrap()
            .send(client.clone())
            .await
            .unwrap();

        // dbg!(&job);

        let job_info = JobInfoRequest::builder()
            .job_id(&job.job_id)
            .except("Finished")
            .timeout(10)
            .build()
            .unwrap()
            .send(client.clone())
            .await
            .unwrap();

        // dbg!(&job_info);

        let output_blob = DataDownloadRequest::builder()
            .data_id(&job_info.output.unwrap().data_id)
            .build()
            .unwrap()
            .send(client.clone())
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
            .unwrap()
            .send(client.clone())
            .await
            .unwrap();

        // dbg!(&worker);

        let job = WorkerContractRequest::builder()
            .worker_id(&worker.worker_id)
            .timeout(10)
            .build()
            .unwrap()
            .send(client.clone())
            .await
            .unwrap();

        // dbg!(&job);

        let job_info = JobInfoRequest::builder()
            .job_id(&job.job_id.unwrap())
            .build()
            .unwrap()
            .send(client.clone())
            .await
            .unwrap();

        // dbg!(&job_info);

        let input_blob = DataDownloadRequest::builder()
            .data_id(job_info.input.data_id)
            .build()
            .unwrap()
            .send(client.clone())
            .await
            .unwrap();

        // dbg!(&input_blob);

        let output_blob = DataUploadRequest::builder()
            .data("output".into())
            .build()
            .unwrap()
            .send(client.clone())
            .await
            .unwrap();

        // dbg!(&output_blob);

        let job_update = JobUpdateRequest::builder()
            .job_id(job_info.job_id)
            .data_id(output_blob.data_id)
            .status("finished")
            .build()
            .unwrap()
            .send(client.clone())
            .await
            .unwrap();

        dbg!(job_update);
    });

    requester_task.await.unwrap();
    worker_task.await.unwrap();
}
