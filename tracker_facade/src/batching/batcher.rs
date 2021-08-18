use std::{sync::{RwLock, Arc, Mutex}, thread};
use std::time::Duration;
use crate::errors::BatchError;

/// MY OWN KAFKA BATCHING STUFF, MADE IT JUST FOR FUN!!!!! 
pub struct Batcher<T, E> {
    collection: Arc<RwLock<Vec<T>>>,
    config: BatchConfig,
    batch_fn: Arc<Mutex<dyn Fn(&Vec<T>) -> Result<(), E>>>
}

impl<T, E> Batcher<T, E> {
    pub fn new(config: BatchConfig, func: Arc<Mutex<dyn Fn(&Vec<T>) -> Result<(), E>>>) -> Self {
        Batcher{
            collection: Arc::new(RwLock::new(Vec::with_capacity(config.max_batch_size))), 
            config,
            batch_fn: func,
        }
    }

    /// Processing batch operations, must be launched in self task/thread
    pub fn start_batching(&mut self) -> Result<(), BatchError> {
        loop {
            let duration = Duration::from_secs(self.config.timeout_seconds);
            thread::sleep(duration);
            
            println!("start batch proc");
            if let Err(_) = self.process_batch() {
                println!("error processing batch");
            }
        }
    }

    fn process_batch(&mut self) -> Result<(), BatchError>{
        {
            let collection = self.collection.read().unwrap();

            // If no data - break the processing
            if collection.len() == 0{
                return Ok(())
            }

            // start processing
            let func = &self.batch_fn.lock().unwrap();
            if let Err(_) = func(&collection) {
                return Err(BatchError {message: "error processing batch".to_string()});
            }
        }
        // clear the batcher
        self.collection.write().unwrap().clear();
        Ok(())
    }

    /// Adds element to batch, if batch is full - process it
    pub fn add_to_batch(&mut self, element: T){
        let len;

        {
            let mut collection = self.collection.write().unwrap();
            collection.push(element);
            len = collection.len();
        }

        if len >= self.config.max_batch_size {
            let _ = self.process_batch();
        }
    }
}

pub struct BatchConfig {
    max_batch_size: usize,
    timeout_seconds: u64
}

impl BatchConfig {
    pub fn new(max_batch_size: usize, timeout_seconds: u64) -> Self {
        BatchConfig{max_batch_size, timeout_seconds}
    }
}
