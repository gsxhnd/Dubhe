//! TCP transport layer.
//!
//! Handles raw TCP connection to the broker. TLS and WebSocket support
//! will be added in the future.

use std::io;

use tokio::net::TcpStream;

/// Establishes a TCP connection to the given address.
pub(crate) async fn connect_tcp(addr: &str) -> io::Result<TcpStream> {
    TcpStream::connect(addr).await
}
