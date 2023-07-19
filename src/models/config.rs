use std::cell::{Cell, RefCell};

use crate::infra::queue::producer::queue_producer::QueueProducer;

#[derive(Clone)]
pub struct Config {
    pub queue_producer: RefCell<Box<dyn QueueProducer>>,
}
