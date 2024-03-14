use std::net::{ToSocketAddrs, UdpSocket};
use artnet_protocol::{ArtCommand, Poll};
use crate::DMXCallee;

/// Accept ArtNet And Callback
///
/// Sets up an ArtNet node to receive ArtNet packets.
///
/// Provide a DMXCallee implementation to call
/// whenever an ArtNet Output packet is received.
pub extern fn accept_artnet_and_callback(address: &str, port: u16, art_net_callback: impl DMXCallee) { // setup code taken from the library itself. It's in the first comment of the library
    let socket = UdpSocket::bind((address, port)).unwrap(); // bind to the requested address, this may fail
    let broadcast_addr = ("255.255.255.255", port).to_socket_addrs().unwrap().next().unwrap();
    socket.set_broadcast(true).unwrap();
    let buff = ArtCommand::Poll(Poll::default()).write_to_buffer().unwrap();
    socket.send_to(&buff, &broadcast_addr).unwrap(); // Poll other artnet devices
    loop { // waits for artnet commands.
        let mut buffer = [0u8; 1024];
        let (length, addr) = socket.recv_from(&mut buffer).unwrap();
        let command = ArtCommand::from_buffer(&buffer[..length]).unwrap();
        match command {
            ArtCommand::Output(output) => { // When receiving the Output packet (as per the artnet specification), call the DMXCallee and pass it the output data
                art_net_callback.dmx_callback(output.data.as_ref().clone());
            }
            _ => {}
        }
    }
}