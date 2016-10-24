use cell::UnsafeCell;
use fs::File;
use io::{Error, ErrorKind, Result, Read, Write};
use net::SocketAddr;
use time::Duration;

#[derive(Debug)]
pub struct UdpSocket(UnsafeCell<File>, bool);

impl UdpSocket {
    pub fn bind(addr: &SocketAddr) -> Result<UdpSocket> {
        let path = format!("udp:{}", addr);
        Ok(UdpSocket(UnsafeCell::new(try!(File::open(path))), false))
    }

    pub fn connect(&self, addr: &SocketAddr) -> Result<()> {
        Err(Error::new(ErrorKind::Other, "UdpSocket::connect not implemented"))
    }

    pub fn duplicate(&self) -> Result<UdpSocket> {
        Err(Error::new(ErrorKind::Other, "UdpSocket::duplicate not implemented"))
    }

    pub fn recv_from(&self, buf: &mut [u8]) -> Result<(usize, SocketAddr)> {
        Err(Error::new(ErrorKind::Other, "UdpSocket::recv_from not implemented"))
    }

    pub fn recv(&self, buf: &mut [u8]) -> Result<usize> {
        if self.1 {
            unsafe { (*self.0.get()).read(buf) }
        } else {
            Err(Error::new(ErrorKind::Other, "UdpSocket::recv not connected"))
        }
    }

    pub fn send_to(&self, buf: &[u8], addr: &SocketAddr) -> Result<usize> {
        Err(Error::new(ErrorKind::Other, "UdpSocket::send_to not implemented"))
    }

    pub fn send(&self, buf: &[u8]) -> Result<usize> {
        if self.1 {
            unsafe { (*self.0.get()).write(buf) }
        } else {
            Err(Error::new(ErrorKind::Other, "UdpSocket::send not connected"))
        }
    }

    pub fn take_error(&self) -> Result<Option<Error>> {
        Ok(None)
    }

    pub fn socket_addr(&self) -> Result<SocketAddr> {
        Err(Error::new(ErrorKind::Other, "UdpSocket::socket_addr not implemented"))
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
