use std::{sync::{RwLock, Arc}, thread};
use std::time::Duration;
use crate::errors::BatchError;

struct Batcher<T> {
    collection: Arc<RwLock<Vec<T>>>,
    config: BatchConfig
}

impl<T> Batcher<T> {
    pub fn new(config: BatchConfig) -> Self {
        Batcher{
            collection: Arc::new(RwLock::new(Vec::new())), 
            config
        }
    }

    pub async fn start_batching<E>(&mut self) -> Result<(), BatchError> {
        let a = Arc::new(self);
        thread::spawn(move ||  {
            loop {
                let duration = Duration::from_secs(a.config.timeout_seconds);
                thread::sleep(duration);
            }
        });
        Ok(())
    }

    fn process_batch<E>(&mut self, func: impl Fn(&Vec<T>) -> Result<(), E>) -> Result<(), BatchError>{
        {
            let collection = self.collection.read().unwrap();

            if let Err(_) = func(&collection) {
                return Err(BatchError {message: "error processing batch".to_string()});
            }
        }
        // clear the batcher
        self.collection.write().unwrap().clear();
        Ok(())
    }

    pub fn add_to_batch(&mut self, element: T){
        self.collection.write().unwrap().push(element);
    }
}

struct BatchConfig {
    max_batch_size: u32,
    timeout_seconds: u64
}
