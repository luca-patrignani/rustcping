use std::net::{SocketAddr, TcpStream};

pub trait Pinger {
    fn ping(&self) -> Option<std::io::Error>;
}

pub struct PingTimeout {
    pub socket: SocketAddr,
    pub conn_timeout: std::time::Duration,
}

impl Pinger for PingTimeout {
    fn ping(&self) -> Option<std::io::Error> {
        TcpStream::connect_timeout(&self.socket, self.conn_timeout).err()
    }
}

pub struct PingWithoutTimeout {
    pub socket: SocketAddr,
}

impl Pinger for PingWithoutTimeout {
    fn ping(&self) -> Option<std::io::Error> {
        TcpStream::connect(self.socket).err()
    }
}
