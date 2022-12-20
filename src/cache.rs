use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

pub struct Entry {
    value: String,
    ttl: u128
}

pub struct Cache {
    cache_data: HashMap<String, Entry>,
}

impl Cache {
    pub fn new() -> Self {
        return Self {
            cache_data: HashMap::new(),
        };
    }

    pub fn set(&mut self, key: String, value: String, ttl: u128) {
        self.cache_data.insert(key, Entry { value, ttl });
    }

    pub fn get(&mut self, key: String) -> String {
        return match self.cache_data.get(&key) {
            Some(entry) =>{
                if entry.ttl == 0 {
                    return entry.value.clone();
                }

                let start = SystemTime::now();
                let since_the_epoch = start.duration_since(UNIX_EPOCH).unwrap();
                let current: u128 = since_the_epoch.as_millis();
                if entry.ttl <= current{
                    self.cache_data.remove(&key);
                    return String::new();
                }
                return entry.value.clone()
            },
            None => String::new()
        };
    }
}
