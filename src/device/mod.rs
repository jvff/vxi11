mod device_error_code;
mod device_link;
mod create_link_parameters;
mod create_link_response;

pub use self::create_link_parameters::CreateLinkParameters;
pub use self::device_link::DeviceLink;

use self::create_link_response::CreateLinkResponse;
use self::device_error_code::DeviceErrorCode;

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
        }
    }
}
