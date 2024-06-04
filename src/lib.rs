mod api;
mod blob;
mod job;
mod lambda;
mod runtime;
mod worker;

use anyhow::Result;

pub use blob::Blob;
pub use job::Job;
pub use lambda::Lambda;
pub use runtime::Runtime;
// pub use worker::Worker;

pub trait MecrmObject {
    fn new() -> impl ObjectBuilder;
    // fn id() -> Option<&'a str>;
}

pub trait ObjectBuilder {
    fn new() -> Self;
    fn build(self) -> Result<impl MecrmObject>;
}
