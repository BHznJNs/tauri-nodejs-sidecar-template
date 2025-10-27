use std::net::TcpListener;

pub fn get_available_tcp_port() -> std::io::Result<u16> {
  let listener = TcpListener::bind("0.0.0.0:0")?;
  let addr = listener.local_addr()?;
  Ok(addr.port())
}
