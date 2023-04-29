use quiche::{Config, ConnectionId, SendInfo, PROTOCOL_VERSION};
use quiche_server_test::get_config;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};


fn main() {
// Server application
    let SERVER: SocketAddr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080);

// Client application
    let CLIENT: SocketAddr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080);
    let v = vec![1];
    let connection = quiche::accept(
        &ConnectionId::from_vec(v),
        None,
        SERVER,
        CLIENT,
        &mut get_config().unwrap(), //FIXME
    );

    let packet: &mut[u8] = &mut [123];
    let mut conn = connection.unwrap();

    loop {
        let (bytes_for_output_buffer, send_info): (usize, SendInfo) = match conn.send(packet) {
            Ok(v) => {
                println!("bytes for output: {}", v.0);
                dbg!("send info: {}", v.1);
                v
            },
            Err(quiche::Error::Done) => {
                println!("Job's done!");
                break;
            }
            Err(_e) => {
                println!("Something unexpected happened!Oh noes!");
                break;
            }
        };
        // Use UDP socket to send out data
        // TODO



    }
}
