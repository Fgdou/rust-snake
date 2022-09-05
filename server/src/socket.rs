use std::net::{SocketAddr, TcpListener};

fn listen(port: u16) -> std::io::Result<()>{
    let listener = TcpListener::bind(SocketAddr::from(([0, 0, 0, 0], port)))?;

    for stream in listener.incoming() {

    }

    Ok(())

}