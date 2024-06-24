use crate::req::Request;
use std::io::Result;
use std::mem::size_of;
use std::net::ToSocketAddrs;
use std::net::UdpSocket;

pub const BUF_SIZE: usize = size_of::<Request>();

#[derive(Debug)]
pub struct Client {
    socket: UdpSocket,
    dest_addrs: Vec<String>,
    dest_port: u16,
}

impl Client {
    pub fn new<T: ToSocketAddrs>(addr: T, dest_addrs: Vec<String>, dest_port: u16) -> Result<Self> {
        Ok(Self {
            socket: UdpSocket::bind(addr)?,
            dest_addrs,
            dest_port,
        })
    }

    pub fn recv_req(&mut self) -> Result<Request> {
        let mut buf: [u8; BUF_SIZE] = [0; BUF_SIZE];
        self.socket.recv(&mut buf)?;
        let req: Request = bincode::deserialize(&buf).unwrap();
        Ok(req)
    }

    pub fn send_req(&mut self, req: &Request) -> Result<()> {
        for dest in self.dest_addrs.iter() {
            let buf = bincode::serialize(&req).unwrap();
            self.socket.send_to(&buf, (dest.as_str(), self.dest_port))?;
        }
        Ok(())
    }

    pub fn try_clone(&self) -> Result<Self> {
        let new_socket = self.socket.try_clone()?;
        Ok(Self {
            socket: new_socket,
            dest_addrs: self.dest_addrs.clone(),
            dest_port: self.dest_port,
        })
    }
}
