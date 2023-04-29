use quiche::{Config, ConnectionError, Error};

pub fn get_config() -> Result<Config, Error> {
    let mut config = Config::new(quiche::PROTOCOL_VERSION)?;

    match set_defaults_on_config(config) {
        Ok(con) => {
            Ok(con)
        }
        Err(e) => Err(e)
    }
}

fn set_defaults_on_config(mut config: Config) -> Result<Config, Error> {
    // Set the protocol e.g. http2, http1 - includes h3 drafts
    config.set_application_protos(quiche::h3::APPLICATION_PROTOCOL)?;

    // Bidirectional stream limits and buffer limits
    config.set_initial_max_streams_bidi(32);
    config.set_initial_max_stream_data_bidi_local(4096);
    config.set_initial_max_stream_data_bidi_remote(4096);

    // Buffer per connection
    config.set_initial_max_data(16384);

    // Unidirectional streams
    config.set_initial_max_streams_uni(64);
    config.set_initial_max_stream_data_uni(4096);

    // Idle timeout
    config.set_max_idle_timeout(3000);
    Ok(config)
}

fn set_tls_on_config(mut config: Config) -> Config {
    //TODO - Boring SSL or nah?
    config
}

pub fn set_udp_socket() {
    // Setup the event loop.
    // let mut poll = mio::Poll::new().unwrap();
    // let mut events = mio::Events::with_capacity(1024);
    //
    // // Create the UDP listening socket, and register it with the event loop.
    // let mut socket =
    //     mio::net::UdpSocket::bind("127.0.0.1:4433".parse().unwrap()).unwrap();
    // poll.registry()
    //     .register(&mut socket, mio::Token(0), mio::Interest::READABLE)
    //     .unwrap();
}
