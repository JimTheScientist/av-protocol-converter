use rusb;

use std::time::Duration;
use rusb::{Context, DeviceHandle, UsbContext};
use std::net::{UdpSocket, ToSocketAddrs};
use artnet_protocol::*;

const UDMX_VENDOR_ID: u16 = 0x16C0; // vendor ID for the UDMX device
const UDMX_PRODUCT_ID: u16 = 0x05DC; // product ID for the UDMX device
/// DMX Callee
///
/// Contains the standard callee function for all event-driven services.
/// The vec is the channel values to receive.
pub trait DMXCallee {
    fn dmx_callback(&self, vec: Vec<u8>);
}
/// Set UDMX Channel
///
/// Takes in a starting channel and values. Channel starts at 1, max 512.
///
/// Device handle represents the Libusb device handle for the UDMX device.
/// You can get this with setup_udmx().
pub extern fn set_udmx_channel(channel: u16, values: Vec<u8>, device_handle: &DeviceHandle<Context>) {
    let bm_request_type = rusb::request_type(rusb::Direction::Out, rusb::RequestType::Vendor, rusb::Recipient::Device);
    let _ = device_handle.write_control(bm_request_type, 2, values.len() as u16, channel - 1, values.as_slice(), Duration::from_millis(10));
}
/// Set up UDMX
///
/// Returns the DeviceHandle needed
/// for the set_udmx_channel() function.
pub extern fn setup_udmx() -> DeviceHandle<Context> {
    let context: Context = Context::new().unwrap();
    let device: DeviceHandle<Context> = context.open_device_with_vid_pid(UDMX_VENDOR_ID, UDMX_PRODUCT_ID).unwrap();
    return device;
}
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