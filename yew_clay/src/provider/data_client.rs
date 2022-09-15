use lru::LruCache;
use std::{any::Any, collections::HashMap, fmt::Debug, future::Future, num::NonZeroUsize};

pub enum ValueOption {
    Value(Box<dyn Any>),
    FutureValue(Box<dyn Future<Output = Box<dyn Any>>>),
}

pub struct DataClient {
    cache: LruCache<&'static str, Box<dyn Any>>,
    current_futures: HashMap<&'static str, Box<dyn Future<Output = Box<dyn Any>>>>,
    cursors: HashMap<&'static str, String>,
}

impl Debug for DataClient {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DataClient")
            .field("cache", &self.cache)
            .field("current_futures", &self.current_futures.keys())
            .field("cursors", &self.cursors)
            .finish()
    }
}

impl DataClient {
    pub fn new(storage_max_size: NonZeroUsize) -> Self {
        Self {
            cache: LruCache::new(storage_max_size),
            current_futures: HashMap::new(),
            cursors: HashMap::new(),
        }
    }

    pub fn read(&mut self, key: &'static str) -> Option<&Box<dyn Any>> {
        self.cache.get(&key)
    }

    pub fn update(&mut self, key: &'static str, value: ValueOption) {
        match value {
            ValueOption::Value(value) => {
                self.cache.push(key, value);
            }
            ValueOption::FutureValue(future) => {
                self.current_futures.insert(key, future);
            }
        };
    }

    pub fn is_fetching(&self, key: &'static str) -> bool {
        self.current_futures.contains_key(key)
    }

    pub fn get_cursor(&mut self, key: &'static str) -> Option<&String> {
        self.cursors.get(key)
    }

    pub fn set_cursor(&mut self, key: &'static str, value: String) -> Option<String> {
        self.cursors.insert(key, value)
    }

    pub fn resize(&mut self, new_max_size: NonZeroUsize) {
        self.cache.resize(new_max_size)
    }

    pub fn max_size(&self) -> NonZeroUsize {
        self.cache.cap()
    }
}
