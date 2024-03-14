mod udmx;
mod artnet;

pub use udmx::UDMXDevice;
pub use artnet::accept_artnet_and_callback;

/// DMX Callee
///
/// Contains the standard callee function for all event-driven services.
/// The vec is the channel values to receive.
pub trait DMXCallee {
    fn dmx_callback(&self, vec: Vec<u8>);
}
