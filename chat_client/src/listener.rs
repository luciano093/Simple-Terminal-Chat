use std::net::TcpListener;

#[derive(Debug)]
pub enum BindError {
    TcpError(std::io::Error),
    OutOfPortsError
}

pub fn get_tcp_socket() -> Result<TcpListener, BindError> {
    // max u16 is equal to max port
    let mut port = std::num::Wrapping(4096u16);

    // tries to use next port if last one is being used by another program
    while port.0 != 0 {
        let addr = format!("127.0.0.1:{}", port);

        match TcpListener::bind(addr) {
            Ok(socket) => return Ok(socket),
            Err(err) => {
                if err.kind() != std::io::ErrorKind::AddrInUse &&
                    err.kind() != std::io::ErrorKind::AddrNotAvailable {
                    return Err(BindError::TcpError(err));
                }

                port += 1;
            }
        }
    }

    Err(BindError::OutOfPortsError)
}