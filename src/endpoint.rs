use super::*;

/// Represent a USB endpoint
#[derive(Clone, Copy, Debug, Default)]
pub struct UsbEndpoint {
    /// bEndpointAddress
    pub address: u8,
    /// bmAttributes
    pub attributes: u8,
    /// wMaxPacketSize
    pub max_packet_size: u16,
    /// bInterval
    pub interval: u8,
}

impl UsbEndpoint {
    pub fn direction(&self) -> Direction {
        if self.address & 0x80 != 0 {
            Direction::In
        } else {
            Direction::Out
        }
    }

    pub fn is_ep0(&self) -> bool {
        self.address & 0x7F == 0
    }
}
