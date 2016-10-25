use cell::UnsafeCell;
use fs::File;
use io::{Error, ErrorKind, Result, Read, Write};
use net::{SocketAddr, Shutdown};
use time::Duration;
use vec::Vec;

use super::{path_to_peer_addr, path_to_local_addr};

#[derive(Debug)]
pub struct TcpStream(UnsafeCell<File>);

impl TcpStream {
    pub fn connect(addr: &SocketAddr) -> Result<TcpStream> {
        let path = format!("tcp:{}", addr);
        Ok(TcpStream(UnsafeCell::new(try!(File::open(path)))))
    }

    pub fn duplicate(&self) -> Result<TcpStream> {
        unsafe { (*self.0.get()).dup(&[]).map(|file| TcpStream(UnsafeCell::new(file))) }
    }

    pub fn read(&self, buf: &mut [u8]) -> Result<usize> {
        unsafe { (*self.0.get()).read(buf) }
    }

    pub fn read_to_end(&self, buf: &mut Vec<u8>) -> Result<usize> {
        unsafe { (*self.0.get()).read_to_end(buf) }
    }

    pub fn write(&self, buf: &[u8]) -> Result<usize> {
        unsafe { (*self.0.get()).write(buf) }
    }

    pub fn take_error(&self) -> Result<Option<Error>> {
        Ok(None)
    }

    pub fn peer_addr(&self) -> Result<SocketAddr> {
        let path = unsafe { (*self.0.get()).path() }?;
        Ok(path_to_peer_addr(path.to_str().unwrap_or("")))
    }

    pub fn socket_addr(&self) -> Result<SocketAddr> {
        let path = unsafe { (*self.0.get()).path() }?;
        Ok(path_to_local_addr(path.to_str().unwrap_or("")))
    }

    pub fn shutdown(&self, _how: Shutdown) -> Result<()> {
        Err(Error::new(ErrorKind::Other, "TcpStream::shutdown not implemented"))
    }

    pub fn nodelay(&self) -> Result<bool> {
        Err(Error::new(ErrorKind::Other, "TcpStream::nodelay not implemented"))
    }

    pub fn nonblocking(&self) -> Result<bool> {
        Err(Error::new(ErrorKind::Other, "TcpStream::nonblocking not implemented"))
    }

    pub fn only_v6(&self) -> Result<bool> {
        Err(Error::new(ErrorKind::Other, "TcpStream::only_v6 not implemented"))
    }

    pub fn ttl(&self) -> Result<u32> {
        Err(Error::new(ErrorKind::Other, "TcpStream::ttl not implemented"))
    }

    pub fn read_timeout(&self) -> Result<Option<Duration>> {
        Err(Error::new(ErrorKind::Other, "TcpStream::read_timeout not implemented"))
    }

    pub fn write_timeout(&self) -> Result<Option<Duration>> {
        Err(Error::new(ErrorKind::Other, "TcpStream::write_timeout not implemented"))
    }

    pub fn set_nodelay(&self, _nodelay: bool) -> Result<()> {
        Err(Error::new(ErrorKind::Other, "TcpStream::set_nodelay not implemented"))
    }

    pub fn set_nonblocking(&self, _nonblocking: bool) -> Result<()> {
        Err(Error::new(ErrorKind::Other, "TcpStream::set_nonblocking not implemented"))
    }

    pub fn set_only_v6(&self, _only_v6: bool) -> Result<()> {
        Err(Error::new(ErrorKind::Other, "TcpStream::set_only_v6 not implemented"))
    }

    pub fn set_ttl(&self, _ttl: u32) -> Result<()> {
        Err(Error::new(ErrorKind::Other, "TcpStream::set_ttl not implemented"))
    }

    pub fn set_read_timeout(&self, _dur: Option<Duration>) -> Result<()> {
        Err(Error::new(ErrorKind::Other, "TcpStream::set_read_timeout not implemented"))
    }

    pub fn set_write_timeout(&self, _dur: Option<Duration>) -> Result<()> {
        Err(Error::new(ErrorKind::Other, "TcpStream::set_write_timeout not implemented"))
    }
}

#[derive(Debug)]
pub struct TcpListener(SocketAddr);

impl TcpListener {
    pub fn bind(addr: &SocketAddr) -> Result<TcpListener> {
        Ok(TcpListener(*addr))
    }

    pub fn accept(&self) -> Result<(TcpStream, SocketAddr)> {
        let file = File::open(&format!("tcp:/{}", self.0.port()))?;
        let path = file.path()?;
        Ok((TcpStream(UnsafeCell::new(file)), path_to_peer_addr(path.to_str().unwrap_or(""))))
    }

    pub fn duplicate(&self) -> Result<TcpListener> {
        Ok(TcpListener(self.0))
    }

    pub fn take_error(&self) -> Result<Option<Error>> {
        Ok(None)
    }

    pub fn socket_addr(&self) -> Result<SocketAddr> {
        Ok(self.0)
    }

    pub fn nonblocking(&self) -> Result<bool> {
        Err(Error::new(ErrorKind::Other, "TcpListener::nonblocking not implemented"))
    }

    pub fn only_v6(&self) -> Result<bool> {
        Err(Error::new(ErrorKind::Other, "TcpListener::only_v6 not implemented"))
    }

    pub fn ttl(&self) -> Result<u32> {
        Err(Error::new(ErrorKind::Other, "TcpListener::ttl not implemented"))
    }

    pub fn set_nonblocking(&self, _nonblocking: bool) -> Result<()> {
        Err(Error::new(ErrorKind::Other, "TcpListener::set_nonblocking not implemented"))
    }

    pub fn set_only_v6(&self, _only_v6: bool) -> Result<()> {
        Err(Error::new(ErrorKind::Other, "TcpListener::set_only_v6 not implemented"))
    }

    pub fn set_ttl(&self, _ttl: u32) -> Result<()> {
        Err(Error::new(ErrorKind::Other, "TcpListener::set_ttl not implemented"))
    }
}
