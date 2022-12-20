use anyhow::Result;
use std::io::{Read, Write};
// Uncomment this block to pass the first stage
use std::net::{TcpListener, TcpStream};
use std::{str, thread};
use crate::{resp::RESPType, commands::CommandType, cache::Cache};
use std::sync::{Arc, Mutex};
use std::sync::mpsc::channel;

const MESSAGE_SIZE: usize = 512;

pub struct Server {
    listener: TcpListener

}

impl Server {
    pub fn new() -> Result<Self> {
        return Self::with_port("6379");
    }

    pub fn with_port(port: &str) -> Result<Self> {
        let listener = match std::net::TcpListener::bind(format!("0.0.0.0:{port}")) {
            Ok(listener) => listener,
            Err(e) => {
                return Err(e.into());
            }
        };

        println!("Server listening on port {port}");

        Ok(Self { listener })
    }

    pub fn run(&mut self) {
        let arc_cache = Arc::new(Mutex::new(Cache::new()));
        let (tx, _) = channel::<Cache>();

        for stream in self.listener.incoming() {
            match stream {
                Ok(_stream) => {
                    println!("accepted new connection");
                    let (cache, _) = (Arc::clone(&arc_cache), tx.clone());
                    thread::spawn(move || {
                        Self::handle_client(_stream, cache);
                    });
                }
                Err(e) => {
                    println!("error: {}", e);
                }
            }
        }
    }

    fn handle_client(mut stream: TcpStream, mut cache: Arc<Mutex<Cache>> ) {
        let mut data = [0 as u8; MESSAGE_SIZE];
        println!("Starting handler for connection");
        loop {
            match stream.read(&mut data) {
                Ok(_size) => {
                    let (command_array, _) = RESPType::unpack(&data);
                    if let RESPType::Array(array_items) = command_array {
                        if let RESPType::BulkString(command_text) = array_items.get(0).unwrap() {
                            let acutal_command = CommandType::get_type(command_text.to_string());
                            let args: Vec<&RESPType> = array_items.iter().skip(1).collect();

                            let reply = acutal_command.handle_command(args, &mut cache);
                            match stream.write(&reply){
                                Ok(_) => continue,
                                Err(e) => {
                                    eprintln!("error writing to the stream: {}", e);
                                    stream.shutdown(std::net::Shutdown::Both).unwrap();
                                    break;
                                }
                            };
                        }
                    }
                }
                Err(error) => {
                    eprintln!("{error}");
                    break;
                }
            }
        }
    }
}