mod create_link_parameters;
mod create_link_response;
mod device_flags;
mod device_error_code;
mod device_link;
mod device_read_parameters;
mod device_read_response;
mod device_write_parameters;
mod device_write_response;

pub use self::create_link_parameters::CreateLinkParameters;
pub use self::device_error_code::DeviceErrorCode;
pub use self::device_link::DeviceLink;
pub use self::device_read_parameters::DeviceReadParameters;
pub use self::device_write_parameters::DeviceWriteParameters;

use self::create_link_response::CreateLinkResponse;
use self::device_read_response::DeviceReadResponse;
use self::device_write_response::DeviceWriteResponse;

onc_rpc! {
    program(core::CoreChannel) {
        id = 395_183;
        version = 1;
        connect = CoreChannelConnect;

        procedures {
            10 => create_link(link_parameters: CreateLinkParameters)
                -> CreateLinkResponse => CreateLinkResult,
            23 => destroy_link(link_id: DeviceLink) -> DeviceErrorCode
                => DestroyLinkResult,

            11 => device_write(write_parameters: DeviceWriteParameters)
                -> DeviceWriteResponse => DeviceWriteResult,
            12 => device_read(read_parameters: DeviceReadParameters)
                -> DeviceReadResponse => DeviceReadResult,
        }
    }
}
