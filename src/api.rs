mod data;
mod lambda;
mod worker;
mod job;
mod kv;

enum BodyType {
    Json,
    Multipart,
}

trait IntoBody {
    fn into_body(self) -> reqwest::Body;
}
