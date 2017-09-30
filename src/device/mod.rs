use std::net::IpAddr;

use futures::future::*;
use onc_rpc;
use tokio_core::reactor::Handle;

use self::write::DeviceWrite;
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

type OpenLink =
    OrElse<
        Map<
            AndThen<CheckIfLinkIsClosed, CreateLink, CreateLinkFn>,
            SetLinkIdFn,
        >,
        FutureResult<Device, onc_rpc::Error>,
        ResultIntoFutureFn,
    >;
type CheckIfLinkIsClosed = FutureResult<Device, Result<Device, onc_rpc::Error>>;
type CreateLink =
    MapErr<
        Join<FutureResult<Device, onc_rpc::Error>, CreateLinkResult>,
        ErrorIntoResultFn,
    >;
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
type Read =
    Join<
        FutureResult<Device, onc_rpc::Error>,
        Map<DeviceReadResult, GetReadDataFn>,
    >;

type CreateLinkFn = fn(Device) -> CreateLink;
type SetLinkIdFn = fn((Device, CreateLinkResponse)) -> Device;
type DestroyLinkFn = fn((Device, DeviceLink)) -> DestroyLink;
type ReturnSelfAfterLinkIsDestroyedFn = fn((Device, DeviceErrorCode)) -> Device;
type ErrorIntoResultFn = fn(onc_rpc::Error) -> Result<Device, onc_rpc::Error>;
type ResultIntoFutureFn =
    fn(Result<Device, onc_rpc::Error>) -> FutureResult<Device, onc_rpc::Error>;
type GetReadDataFn = fn(DeviceReadResponse) -> Vec<u8>;

impl Device {
    pub fn connect(
        address: IpAddr,
        handle: &Handle,
    ) -> Map<CoreChannelConnect, fn(CoreChannel) -> Device> {
        CoreChannel::connect(address, handle)
            .map(Device::from)
    }

    pub fn open_link(self) -> OpenLink {
        self.check_if_link_is_closed()
            .and_then(Self::create_link as CreateLinkFn)
            .map(Self::set_link_id as SetLinkIdFn)
            .or_else(Result::into_future)
    }

    fn check_if_link_is_closed(self) -> CheckIfLinkIsClosed {
        if self.link_id.is_some() {
            Err(Ok(self)).into_future()
        } else {
            Ok(self).into_future()
        }
    }

    fn create_link(this: Self) -> CreateLink {
        let create_link =
            this.core_channel.create_link(CreateLinkParameters::new());

        Ok(this)
            .into_future()
            .join(create_link)
            .map_err(Err)
    }

    fn set_link_id(
        (mut this, create_link_result): (Self, CreateLinkResponse),
    ) -> Self {
        this.link_id = Some(create_link_result.link_id());

        this
    }

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

    pub fn write<'s>(self, string: &'s str) -> Flatten<DeviceWrite<'s>> {
        DeviceWrite::new(self, string).flatten()
    }

    fn raw_write(&mut self, string: &str) -> DeviceWriteResult {
        let link_id = self.link_id
            .expect("link to device should have been opened");

        let mut parameters = DeviceWriteParameters::new(link_id);

        parameters.set_data(string);
        parameters.mark_end();

        self.core_channel.device_write(parameters)
    }

    pub fn read(self) -> Read {
        let link_id = self.link_id
            .expect("link to device should have been opened");

        let mut parameters = DeviceReadParameters::new(link_id, 100);

        parameters.set_io_timeout(1000);
        parameters.set_lock_timeout(1000);

        let device_read = self.core_channel
            .device_read(parameters)
            .map(Self::get_read_data as GetReadDataFn);

        Ok(self)
            .into_future()
            .join(device_read)
    }

    fn get_read_data(read_response: DeviceReadResponse) -> Vec<u8> {
        read_response.into_data().into()
    }
}

mod future_cell;
mod write;
