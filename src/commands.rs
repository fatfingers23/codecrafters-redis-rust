use crate::{cache::Cache, resp::RESPType};
use std::sync::{Arc, Mutex, MutexGuard};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug)]
pub enum CommandType {
    Ping,
    Echo,
    Get,
    Set,
    None,
}

impl CommandType {
    pub fn get_type(command_text: String) -> CommandType {
        match command_text.to_lowercase().as_str() {
            "ping" => CommandType::Ping,
            "echo" => CommandType::Echo,
            "set" => CommandType::Set,
            "get" => CommandType::Get,
            _ => CommandType::None,
        }
    }

    pub fn handle_command(&self, args: Vec<&RESPType>, cache: &mut Arc<Mutex<Cache>>) -> Vec<u8> {

        let mut unlocked_cache: MutexGuard<Cache> = cache.lock().unwrap();
        println!("Args: {:?}", args);
        match self {
            Self::Ping | Self::Echo => {
                if args.len() == 0 {
                    return RESPType::SimpleString("PONG".to_string()).pack();
                }
                let args = args.get(0).unwrap();
                return args.pack();
            }
            Self::Set => {
                let mut key: String = String::new();
                let mut value: String = String::new();
                let start = SystemTime::now();
                let since_the_epoch = start.duration_since(UNIX_EPOCH).unwrap();
                let mut ttl: u128 = since_the_epoch.as_millis();
                let mut set_expire = false;
                if let RESPType::BulkString(key_text) = args.get(0).unwrap() {
                    key = key_text.to_string();
                };
                if let RESPType::BulkString(value_text) = args.get(1).unwrap() {
                    value = value_text.to_string();
                };

                if args.len() == 4 {
                    if let RESPType::BulkString(arg) = args.get(2).unwrap() {
                        if arg.to_lowercase() == "px"{
                            if let RESPType::BulkString(arg) = args.get(3).unwrap() {
                                match arg.parse::<u128>() {
                                    Ok(n) => {
                                        ttl = ttl + n;
                                        set_expire = true;
                                    },
                                    Err(_e) => {
                                        return RESPType::Error("That's not a number.".to_string()).pack();
                                    }
                                }
                            }
                        }
                    };
                }


                if set_expire {
                    unlocked_cache.set(key, value, ttl);
                }else{
                    unlocked_cache.set(key, value, 0);
                }

                drop(unlocked_cache);
                let result = RESPType::BulkString("OK".to_string());
                return result.pack();
            }
            Self::Get => {
                let mut key: String = String::new();
                if let RESPType::BulkString(key_text) = args.get(0).unwrap() {
                    key = key_text.to_string();
                };
                let saved_value = unlocked_cache.get(key);
                if saved_value == String::new() {
                    return RESPType::Null.pack();
                }
                let result = RESPType::BulkString(saved_value);
                return result.pack();
            }
            _ => {
                return RESPType::Error("That command has not been setup.".to_string()).pack();
            }
        }
    }
}
