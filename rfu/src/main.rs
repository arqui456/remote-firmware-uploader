use env_logger;
use log::*;
use std::net::*;
use std::time::Duration;
use usbip;
use text_io::read;

#[tokio::main]
async fn main() {
    env_logger::init();
    let server = usbip::UsbIpServer::new_from_host();
    let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), 3240);
    tokio::spawn(usbip::server(addr, server));
    let line: String = read!("{}\n");

    loop {
        // sleep 1s
        tokio::time::sleep(Duration::new(1, 0)).await;
        println!("Read in: {}", line);
    }
}