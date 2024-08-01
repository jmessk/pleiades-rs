use crate::id::Id;
use crate::runtime::Runtime;

pub struct Worker {
    pub id: Id,
    pub runtimes: Vec<Runtime>,
}
