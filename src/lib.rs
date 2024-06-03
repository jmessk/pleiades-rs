mod api;
mod blob;
mod runtime;
mod lambda;
mod job;
mod worker;

pub trait MecrmObject {
    fn new() -> Self;
    fn id(&self) -> &str;
}

pub trait ObjectBuilder {
    fn new() -> Self;
    fn build(self) -> Self;
    fn id(mut self, id: impl Into<String>) -> Self;
}
