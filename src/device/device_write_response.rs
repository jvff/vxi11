use super::device_error_code::DeviceErrorCode;

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub struct DeviceWriteResponse {
    error: DeviceErrorCode,
    bytes_written: u32,
}
