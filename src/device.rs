use std::net::IpAddr;

use futures::future::{Future, Map};
use tokio_core::reactor::Handle;

use super::rpc::{CoreChannel, CoreChannelConnect, DeviceLink};

pub struct Device {
    link_id: Option<DeviceLink>,
    core_channel: CoreChannel,
}

impl From<CoreChannel> for Device {
    fn from(core_channel: CoreChannel) -> Self {
        Device {
            core_channel,
            link_id: None,
        }
    }
}

impl Device {
    pub fn connect(
        address: IpAddr,
        handle: &Handle,
    ) -> Map<CoreChannelConnect, fn(CoreChannel) -> Device> {
        CoreChannel::connect(address, handle)
            .map(Device::from)
    }
}
