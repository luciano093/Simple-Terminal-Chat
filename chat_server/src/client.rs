use std::collections::HashMap;
use std::io::{LineWriter, Write, self};
use std::net::{TcpStream, ToSocketAddrs, SocketAddr};
use std::sync::{Mutex, LockResult, MutexGuard};

pub struct Client {
    writer: LineWriter<TcpStream>,
    username: String,
}

impl Client {
    pub fn new(addr: impl ToSocketAddrs, username: &str) -> Result<Self, io::Error> {
        let listener = TcpStream::connect(addr)?;
        listener.shutdown(std::net::Shutdown::Read)?;
        let writer = LineWriter::new(listener);

        Ok(Client { writer, username: username.to_string() })
    }

    pub fn send_message(&mut self, message: &str) -> std::io::Result<()> {
        let message = format!("{}\n", message);
        self.writer.write_all(message.as_bytes())
    }

    pub const fn username(&self) -> &String {
        &self.username
    }
}

/// Can be safely mutated between threads
pub struct AtomicClients(Mutex<HashMap<SocketAddr, Client>>);

impl AtomicClients {
    pub fn new() -> Self {
        AtomicClients(Mutex::new(HashMap::new()))
    }

    pub fn lock(&self) -> LockResult<MutexGuard<'_, HashMap<SocketAddr, Client>>> {
        self.0.lock()
    }

    pub fn add_client(&self, address: SocketAddr, client: Client) {
        let mut lock = self.0.lock().unwrap();
    
        lock.insert(address, client);
    }

    pub fn remove_client(&self, address: SocketAddr) {
        let mut lock = self.0.lock().unwrap();
    
        lock.remove(&address);
    }

    pub fn send_message(&self, message: &str) -> std::io::Result<()> {
        let mut lock = self.0.lock().unwrap();

        for (_, client) in lock.iter_mut() {
            client.send_message(message)?;
        }

        Ok(())
    }
}