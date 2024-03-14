use std::time::Duration;
use rusb::{Context, DeviceHandle, UsbContext};
use crate::DMXCallee;

/// The UDMX Device represents a UDMX device on your system.
///
/// It can be initialized with UDMXDevice::new().
///
/// You may then send values to it using set_udmx_channel()

const UDMX_VENDOR_ID: u16 = 0x16C0; // vendor ID for the UDMX device
const UDMX_PRODUCT_ID: u16 = 0x05DC; // product ID for the UDMX device

pub struct UDMXDevice {
    device_handle: DeviceHandle<Context>,
}

impl UDMXDevice {
    pub fn new() -> Self { // set up the libusb device
        let context: Context = Context::new().unwrap();
        let device: DeviceHandle<Context> = context.open_device_with_vid_pid(UDMX_VENDOR_ID, UDMX_PRODUCT_ID).unwrap();
        Self {device_handle: device}
    }
    pub fn set_udmx_channel(&self, channel: u16, values: Vec<u8>) { // send the dmx values to the udmx device
        let bm_request_type = rusb::request_type(rusb::Direction::Out, rusb::RequestType::Vendor, rusb::Recipient::Device);
        let _ = self.device_handle.write_control(bm_request_type, 2, values.len() as u16, channel - 1, values.as_slice(), Duration::from_millis(10));
    }
}

/// The DMX callee for the UDMX device.
///
/// This makes it possible to register the UDMX device as
/// a callee for event-based callbacks.

impl DMXCallee for UDMXDevice {
    fn dmx_callback(&self, vec: Vec<u8>) {
        let values = vec.clone();
        let _ = self.set_udmx_channel(1, values);
    }
}