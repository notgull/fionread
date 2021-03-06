//               Copyright John Nunley, 2022.
// Distributed under the Boost Software License, Version 1.0.
//       (See accompanying file LICENSE or copy at
//         https://www.boost.org/LICENSE_1_0.txt)

//! This crate provides a single function: [`fionread`]. See that function for more
//! information.

use __internal::Sealed;
use std::io::Result;

#[cfg(unix)]
use std::os::unix::io::{AsRawFd, RawFd};

#[cfg(windows)]
use std::os::windows::io::{AsRawSocket, RawSocket};

// raw file descriptor or socket
#[cfg(unix)]
type Raw = RawFd;
#[cfg(windows)]
type Raw = RawSocket;
#[cfg(not(any(unix, windows)))]
type Raw = std::convert::Infallible;

/// An object that the [`fionread`] function can be called on.
///
/// Implemented for all `AsRawFd` on Unix and `AsRawSocket` on Windows.
pub trait AsRaw: Sealed {
    /// Returns the raw file descriptor or socket.
    fn as_raw(&self) -> Raw;
}

#[cfg(unix)]
impl<T: AsRawFd + ?Sized> AsRaw for T {
    fn as_raw(&self) -> Raw {
        self.as_raw_fd()
    }
}

#[cfg(windows)]
impl<T: AsRawSocket + ?Sized> AsRaw for T {
    fn as_raw(&self) -> Raw {
        self.as_raw_socket()
    }
}

#[cfg(unix)]
mod unix_impl {
    use std::{
        io::Result,
        mem::MaybeUninit,
        os::{raw::c_int, unix::io::RawFd},
    };

    #[inline]
    pub fn fionread_impl(sock: RawFd) -> Result<usize> {
        use nix::libc::FIONREAD;

        nix::ioctl_read_bad!(fionread, FIONREAD, c_int);

        let mut len = MaybeUninit::uninit();
        unsafe { fionread(sock, len.as_mut_ptr()) }?;
        Ok(unsafe { len.assume_init() } as usize)
    }
}

#[cfg(windows)]
mod windows_impl {
    use std::{
        io::{self, Result},
        mem::MaybeUninit,
        os::windows::io::RawSocket,
    };
    use windows_sys::Win32::Networking::WinSock::{ioctlsocket, FIONREAD, SOCKET};

    #[inline]
    pub fn fionread_impl(sock: RawSocket) -> Result<usize> {
        let mut len = MaybeUninit::uninit();
        let res = unsafe { ioctlsocket(sock as SOCKET, FIONREAD, len.as_mut_ptr()) };
        if res == 0 {
            Ok(unsafe { len.assume_init() } as usize)
        } else {
            Err(io::Error::last_os_error())
        }
    }
}

#[cfg(not(any(windows, unix)))]
mod placeholder_impl {
    use std::{convert::Infallible, io};

    #[inline]
    pub fn fionread_impl(_sock: Infallible) -> Result<usize> {
        Err(io::Error::new(io::ErrorKind::Other, "not implemented"))
    }
}

#[cfg(not(any(windows, unix)))]
use placeholder_impl::fionread_impl;
#[cfg(unix)]
use unix_impl::fionread_impl;
#[cfg(windows)]
use windows_impl::fionread_impl;

/// Read the number of bytes available from a socket.
///
/// This function calls the `FIONREAD` ioctl on Unix and the
/// ioctlsocket equivalent on Windows.
///
/// # Example
///
/// ```no_run
/// # use fionread::fionread;
/// # use std::net::TcpStream;
/// # fn main() -> std::io::Result<()> {
/// let mut stream = TcpStream::connect(("google.com", 80))?;
/// let n = fionread(&stream)?;
/// println!("There are {} bytes available", n);
/// # Ok(())
/// # }
/// ```
#[inline]
pub fn fionread<T: AsRaw + ?Sized>(sock: &T) -> Result<usize> {
    fionread_impl(sock.as_raw())
}

#[doc(hidden)]
mod __internal {
    #[cfg(unix)]
    use std::os::unix::io::AsRawFd;

    #[cfg(windows)]
    use std::os::windows::io::AsRawSocket;

    #[doc(hidden)]
    pub trait Sealed {
        #[doc(hidden)]
        fn __sealed_trait_marker__() {}
    }

    #[cfg(unix)]
    impl<T: AsRawFd + ?Sized> Sealed for T {}
    #[cfg(windows)]
    impl<T: AsRawSocket + ?Sized> Sealed for T {}
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{
        io::Write,
        net::{TcpListener, TcpStream},
        sync::mpsc,
        thread,
    };

    #[test]
    fn test_fionread() {
        let port = 41234u16;
        let listener = TcpListener::bind(("::1", port)).unwrap();

        let (tx1, rx1) = mpsc::channel();
        let (tx2, rx2) = mpsc::channel();
        let _ = thread::spawn(move || {
            let (mut sock, _) = listener.accept().unwrap();
            rx1.recv().unwrap();
            sock.write_all(b"hello").unwrap();
            tx2.send(()).unwrap();
        });

        let sock = TcpStream::connect(("::1", port)).unwrap();

        // at the start, we should not receive any data at all
        assert_eq!(fionread(&sock).unwrap(), 0);

        tx1.send(()).unwrap();

        // after the write, we should receive some data
        rx2.recv().unwrap();

        assert_eq!(fionread(&sock).unwrap(), 5);
    }
}
