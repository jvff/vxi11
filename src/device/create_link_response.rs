use super::device_error_code::DeviceErrorCode;
use super::device_link::DeviceLink;

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub struct CreateLinkResponse {
    error: DeviceErrorCode,
    link_id: DeviceLink,
    abort_port: u16,
    max_receive_size: u32,
}
