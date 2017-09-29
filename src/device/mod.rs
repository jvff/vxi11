mod device_error_code;
mod device_link;
mod create_link_parameters;
mod create_link_response;

pub use self::create_link_parameters::CreateLinkParameters;

use self::create_link_response::CreateLinkResponse;

onc_rpc! {
    program(core::CoreChannel) {
        id = 395_183;
        version = 1;
        connect = CoreChannelConnect;

        procedures {
            10 => create_link(link_parameters: CreateLinkParameters)
                -> CreateLinkResponse => CreateLinkResult,
        }
    }
}
