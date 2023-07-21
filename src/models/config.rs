use std::{sync::Arc};

use crate::infra::queue::producer::queue_producer::QueueProducer;

type DynQueueProducer = Arc<dyn QueueProducer + Send + Sync>;
#[derive(Clone)]
pub struct Config {
    pub queue_producer: DynQueueProducer,
}
