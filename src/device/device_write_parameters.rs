use serde_xdr::OpaqueData;

use super::device_flags::DeviceFlags;
use super::device_link::DeviceLink;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DeviceWriteParameters {
    link_id: DeviceLink,
    io_timeout: u32,
    lock_timeout: u32,
    flags: DeviceFlags,
    data: OpaqueData,
}

impl DeviceWriteParameters {
    pub fn new(link_id: DeviceLink) -> Self {
        DeviceWriteParameters {
            link_id,
            io_timeout: 0,
            lock_timeout: 0,
            flags: DeviceFlags::default(),
            data: OpaqueData::new(),
        }
    }

    pub fn set_data(&mut self, string: &str) {
        self.data.truncate(0);
        self.data.extend(string.as_bytes().iter());
    }
}
