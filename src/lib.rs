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
pub use worker::Worker;

pub trait MecrmObject {
    type Builder: ObjectBuilder<Output = Self>;

    fn new() -> Self::Builder;
}

pub trait ObjectBuilder {
    type Output: MecrmObject<Builder = Self>;

    fn new() -> Self;
    fn build(self) -> Result<Self::Output>;
}
