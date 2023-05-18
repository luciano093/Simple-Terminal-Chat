use std::net::{TcpListener, Incoming, ToSocketAddrs};

pub struct Deploying;
pub struct Running;

pub struct Server<Status = Deploying> {
    socket: Option<TcpListener>,
    status: std::marker::PhantomData<Status>,
}

impl Default for Server<Deploying> { 
    fn default() -> Self {
        Server { socket: None, status: std::marker::PhantomData }
    }
}

impl Server<Deploying> {
    pub fn new() -> Server<Deploying> {
        Server { socket: None, status: std::marker::PhantomData }
    }

    pub fn run(self, addr: impl ToSocketAddrs) -> Server<Running>  {
        let socket = TcpListener::bind(addr)
        .map_err(|err| panic!("Error: {}", err))
        .unwrap();

        Server { socket: Some(socket), status: std::marker::PhantomData }
    }
}

impl Server<Running> {
    pub fn incoming(&self) -> Incoming<'_> {
        self.socket.as_ref().map(|socket| socket.incoming()).unwrap()
    }

    pub fn port(&self) -> u16 {
        self.socket.as_ref().unwrap().local_addr().unwrap().port()
    }
}