use super::device_flags::DeviceFlags;
use super::device_link::DeviceLink;

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub struct DeviceReadParameters {
    link_id: DeviceLink,
    bytes_requested: u32,
    io_timeout: u32,
    lock_timeout: u32,
    flags: DeviceFlags,
    termination_char: char,
}

impl DeviceReadParameters {
    pub fn new(link_id: DeviceLink, bytes_requested: u32) -> Self {
        DeviceReadParameters {
            link_id,
            bytes_requested,
            io_timeout: 0,
            lock_timeout: 0,
            flags: DeviceFlags::default(),
            termination_char: '\0',
        }
    }
}