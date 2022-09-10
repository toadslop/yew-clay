use lru::LruCache;
use std::{collections::HashMap, future::Future};

pub enum ValueOption<T> {
    Value(T),
    FutureValue(Box<dyn Future<Output = T>>),
}

pub struct DataClient<T> {
    cache: LruCache<&'static str, T>,
    current_futures: HashMap<&'static str, Box<dyn Future<Output = T>>>,
    cursors: HashMap<&'static str, String>,
}

impl<T> DataClient<T> {
    pub fn new(storage_max_size: usize) -> Self {
        Self {
            cache: LruCache::new(storage_max_size),
            current_futures: HashMap::new(),
            cursors: HashMap::new(),
        }
    }

    pub fn read(&mut self, key: &'static str) -> Option<&T> {
        self.cache.get(&key)
    }

    pub fn update(&mut self, key: &'static str, value: ValueOption<T>) {
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
}
