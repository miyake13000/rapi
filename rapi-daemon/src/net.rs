use crate::req::Request;
use std::io::Result;
use std::mem::size_of;
use std::net::ToSocketAddrs;
use std::net::UdpSocket;

pub const BUF_SIZE: usize = size_of::<Request>();

#[derive(Debug)]
pub struct Connection {
    socket: UdpSocket,
}

impl Connection {
    pub fn new<T: ToSocketAddrs, S: AsRef<str>>(
        addr: T,
        dest_addr: S,
        dest_port: u16,
    ) -> Result<Self> {
        let socket = UdpSocket::bind(addr)?;
        socket.connect((dest_addr.as_ref(), dest_port))?;
        Ok(Self { socket })
    }

    pub fn recv_req(&mut self) -> Result<Request> {
        let mut buf: [u8; BUF_SIZE] = [0; BUF_SIZE];
        self.socket.recv_from(&mut buf)?;
        let req: Request = bincode::deserialize(&buf).unwrap();
        Ok(req)
    }

    pub fn send_req(&mut self, req: &Request) -> Result<()> {
        let buf = bincode::serialize(&req).unwrap();
        self.socket.send(&buf)?;
        Ok(())
    }

    pub fn try_clone(&self) -> Result<Self> {
        let new_socket = self.socket.try_clone()?;
        Ok(Self { socket: new_socket })
    }
}
