use crate::Id;
use crate::Runtime;

pub struct Worker {
    pub id: Id,
    pub runtimes: Vec<Runtime>,
}
