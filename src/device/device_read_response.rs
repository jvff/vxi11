use serde_xdr::OpaqueData;

use super::device_error_code::DeviceErrorCode;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DeviceReadResponse {
    error: DeviceErrorCode,
    reason: u32,
    data: OpaqueData,
}
