use std::net::IpAddr;

use futures::future::*;
use onc_rpc;
use tokio_core::reactor::Handle;

use super::rpc::*;

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

type CloseLink =
    OrElse<
        AndThen<GetLinkId, DestroyLink, DestroyLinkFn>,
        FutureResult<Device, onc_rpc::Error>,
        ResultIntoFutureFn,
    >;
type GetLinkId =
    FutureResult<(Device, DeviceLink), Result<Device, onc_rpc::Error>>;
type DestroyLink =
    MapErr<
        Map<
            Join<FutureResult<Device, onc_rpc::Error>, DestroyLinkResult>,
            ReturnSelfAfterLinkIsDestroyedFn,
        >,
        ErrorIntoResultFn,
    >;

type DestroyLinkFn = fn((Device, DeviceLink)) -> DestroyLink;
type ReturnSelfAfterLinkIsDestroyedFn = fn((Device, DeviceErrorCode)) -> Device;
type ErrorIntoResultFn = fn(onc_rpc::Error) -> Result<Device, onc_rpc::Error>;
type ResultIntoFutureFn =
    fn(Result<Device, onc_rpc::Error>) -> FutureResult<Device, onc_rpc::Error>;

impl Device {
    pub fn connect(
        address: IpAddr,
        handle: &Handle,
    ) -> Map<CoreChannelConnect, fn(CoreChannel) -> Device> {
        CoreChannel::connect(address, handle)
            .map(Device::from)

    pub fn close_link(self) -> CloseLink {
        self.get_link_id()
            .and_then(Self::destroy_link as DestroyLinkFn)
            .or_else(Result::into_future)
    }

    fn get_link_id(self) -> GetLinkId {
        if let Some(link_id) = self.link_id {
            Ok((self, link_id)).into_future()
        } else {
            Err(Ok(self)).into_future()
        }
    }

    fn destroy_link((mut this, link_id): (Self, DeviceLink)) -> DestroyLink {
        let destroy_link = this.core_channel.destroy_link(link_id);

        this.link_id = None;

        Ok(this)
            .into_future()
            .join(destroy_link)
            .map(
                Self::return_self_after_link_is_destroyed
                    as ReturnSelfAfterLinkIsDestroyedFn,
            )
            .map_err(Err as ErrorIntoResultFn)
    }

    fn return_self_after_link_is_destroyed(
        (mut this, _error): (Self, DeviceErrorCode),
    ) -> Self {
        this
    }
}
