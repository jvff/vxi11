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
