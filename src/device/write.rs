use futures::{Async, Future, IntoFuture, Poll};
use futures::future::{FutureResult, Join, Map};
use onc_rpc;

use super::{Device, OpenLink};
use super::future_cell::FutureCell;
use super::super::rpc::{DeviceWriteResponse, DeviceWriteResult};

type WaitForWriteCompletionFn = fn((Device, DeviceWriteResponse)) -> Device;
type WrapDeviceFn = fn(Device) -> Option<Device>;

pub struct DeviceWrite<'s> {
    linked_device: FutureCell<Map<OpenLink, WrapDeviceFn>>,
    string: &'s str,
}

impl<'s> DeviceWrite<'s> {
    pub fn new(device: Device, string: &'s str) -> Self {
        let linked_device = device
            .open_link()
            .map(Some as WrapDeviceFn)
            .into();

        DeviceWrite { linked_device, string }
    }

    fn wait_for_write_completion(
        (device, _write_response): (Device, DeviceWriteResponse),
    ) -> Device {
        device
    }
}

impl<'s> Future for DeviceWrite<'s> {
    type Item =
        Map<
            Join<FutureResult<Device, Self::Error>, DeviceWriteResult>,
            WaitForWriteCompletionFn,
        >;

    type Error = onc_rpc::Error;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        let mut device_cell = try_ready!(self.linked_device.poll());
        let mut device = device_cell.take()
            .expect("DeviceWrite future polled more than once");

        let device_write = device.raw_write(self.string);

        Ok(
            Async::Ready(
                Ok(device)
                    .into_future()
                    .join(device_write)
                    .map(Self::wait_for_write_completion)
            )
        )
    }
}
