use cell::UnsafeCell;
use fs::File;
use io::{Error, ErrorKind, Result, Read, Write};
use net::SocketAddr;
use time::Duration;

use super::{path_to_peer_addr, path_to_local_addr};

#[derive(Debug)]
pub struct UdpSocket(UnsafeCell<File>, UnsafeCell<Option<SocketAddr>>);

impl UdpSocket {
    pub fn bind(addr: &SocketAddr) -> Result<UdpSocket> {
        let path = format!("udp:/{}", addr);
        Ok(UdpSocket(UnsafeCell::new(File::open(path)?), UnsafeCell::new(None)))
    }

    fn get_bind(&self) -> &mut File {
        unsafe { &mut *(self.0.get()) }
    }

    fn get_conn(&self) -> &mut Option<SocketAddr> {
        unsafe { &mut *(self.1.get()) }
    }

    pub fn connect(&self, addr: &SocketAddr) -> Result<()> {
        unsafe { *self.1.get() = Some(*addr) };
        Ok(())
    }

    pub fn duplicate(&self) -> Result<UdpSocket> {
        let new_bind = self.get_bind().dup(&[])?;
        let new_conn = *self.get_conn();
        Ok(UdpSocket(UnsafeCell::new(new_bind), UnsafeCell::new(new_conn)))
    }

    pub fn recv_from(&self, buf: &mut [u8]) -> Result<(usize, SocketAddr)> {
        let mut from = self.get_bind().dup(b"listen")?;
        let path = from.path()?;
        let peer_addr = path_to_peer_addr(path.to_str().unwrap_or(""));
        let count = from.read(buf)?;
        Ok((count, peer_addr))
    }

    pub fn recv(&self, buf: &mut [u8]) -> Result<usize> {
        if let Some(addr) = *self.get_conn() {
            let mut from = self.get_bind().dup(format!("{}", addr).as_bytes())?;
            from.read(buf)
        } else {
            Err(Error::new(ErrorKind::Other, "UdpSocket::recv not connected"))
        }
    }

    pub fn send_to(&self, buf: &[u8], addr: &SocketAddr) -> Result<usize> {
        let mut to = self.get_bind().dup(format!("{}", addr).as_bytes())?;
        to.write(buf)
    }

    pub fn send(&self, buf: &[u8]) -> Result<usize> {
        if let Some(addr) = *self.get_conn() {
            self.send_to(buf, &addr)
        } else {
            Err(Error::new(ErrorKind::Other, "UdpSocket::send not connected"))
        }
    }

    pub fn take_error(&self) -> Result<Option<Error>> {
        Ok(None)
    }

    pub fn socket_addr(&self) -> Result<SocketAddr> {
        let path = self.get_bind().path()?;
        Ok(path_to_local_addr(path.to_str().unwrap_or("")))
    }

    pub fn broadcast(&self) -> Result<bool> {
        Err(Error::new(ErrorKind::Other, "UdpSocket::broadcast not implemented"))
    }

    pub fn nonblocking(&self) -> Result<bool> {
        Err(Error::new(ErrorKind::Other, "UdpSocket::nonblocking not implemented"))
    }

    pub fn only_v6(&self) -> Result<bool> {
        Err(Error::new(ErrorKind::Other, "UdpSocket::only_v6 not implemented"))
    }

    pub fn ttl(&self) -> Result<u32> {
        Err(Error::new(ErrorKind::Other, "UdpSocket::ttl not implemented"))
    }

    pub fn read_timeout(&self) -> Result<Option<Duration>> {
        Err(Error::new(ErrorKind::Other, "UdpSocket::read_timeout not implemented"))
    }

    pub fn write_timeout(&self) -> Result<Option<Duration>> {
        Err(Error::new(ErrorKind::Other, "UdpSocket::write_timeout not implemented"))
    }

    pub fn set_broadcast(&self, _broadcast: bool) -> Result<()> {
        Err(Error::new(ErrorKind::Other, "UdpSocket::set_broadcast not implemented"))
    }

    pub fn set_nonblocking(&self, _nonblocking: bool) -> Result<()> {
        Err(Error::new(ErrorKind::Other, "UdpSocket::set_nonblocking not implemented"))
    }

    pub fn set_only_v6(&self, _only_v6: bool) -> Result<()> {
        Err(Error::new(ErrorKind::Other, "UdpSocket::set_only_v6 not implemented"))
    }

    pub fn set_ttl(&self, _ttl: u32) -> Result<()> {
        Err(Error::new(ErrorKind::Other, "UdpSocket::set_ttl not implemented"))
    }

    pub fn set_read_timeout(&self, _dur: Option<Duration>) -> Result<()> {
        Err(Error::new(ErrorKind::Other, "UdpSocket::set_read_timeout not implemented"))
    }

    pub fn set_write_timeout(&self, _dur: Option<Duration>) -> Result<()> {
        Err(Error::new(ErrorKind::Other, "UdpSocket::set_write_timeout not implemented"))
    }
}
