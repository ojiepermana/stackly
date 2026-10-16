use std::io;
use std::net::{Ipv4Addr, TcpListener};

/// Asks the OS for a TCP port on the loopback interface that is free right
/// now.
///
/// Each test allocates its own ports instead of using a provider's default
/// (5432, 4222, and so on), so tests never collide with each other or with
/// services already running on the Mac. Nothing holds the port after this
/// returns, so hand it to the process under test straight away.
///
/// # Errors
///
/// Returns the I/O error when no loopback port can be bound.
pub fn allocate_port() -> io::Result<u16> {
    let listener = TcpListener::bind((Ipv4Addr::LOCALHOST, 0))?;
    Ok(listener.local_addr()?.port())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn allocated_port_can_be_bound_on_loopback() {
        let port = allocate_port().unwrap();

        assert_ne!(port, 0);
        TcpListener::bind((Ipv4Addr::LOCALHOST, port)).unwrap();
    }
}
