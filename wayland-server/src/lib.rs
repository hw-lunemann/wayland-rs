#![warn(missing_debug_implementations)]
#![forbid(improper_ctypes, unsafe_op_in_unsafe_fn)]

use wayland_backend::{
    protocol::{Interface, Message},
    server::{ClientId, InvalidId, ObjectId},
};

mod client;
mod dispatch;
mod display;
mod global;
pub mod socket;

pub use client::Client;
pub use dispatch::{DataInit, DelegateDispatch, Dispatch, New, ResourceData};
pub use display::{Display, DisplayHandle};
pub use global::{DelegateGlobalDispatch, GlobalDispatch};

pub mod backend {
    pub use wayland_backend::protocol;
    pub use wayland_backend::server::{
        Backend, ClientData, ClientId, Credentials, DisconnectReason, GlobalHandler, GlobalId,
        Handle, InitError, InvalidId, ObjectData, ObjectId, WeakHandle,
    };
    pub use wayland_backend::smallvec;
}

pub use wayland_backend::protocol::WEnum;

pub mod protocol {
    use self::__interfaces::*;
    use crate as wayland_server;
    pub mod __interfaces {
        wayland_scanner::generate_interfaces!("wayland.xml");
    }
    wayland_scanner::generate_server_code!("wayland.xml");
}

pub trait Resource: Sized {
    type Event;
    type Request;

    fn interface() -> &'static Interface;

    fn id(&self) -> ObjectId;

    fn client_id(&self) -> Option<ClientId> {
        self.handle().upgrade().and_then(|dh| dh.get_client(self.id()).ok())
    }

    fn version(&self) -> u32;

    fn data<U: 'static>(&self) -> Option<&U>;

    fn object_data(&self) -> Option<&std::sync::Arc<dyn std::any::Any + Send + Sync>>;

    fn handle(&self) -> &backend::WeakHandle;

    fn from_id(dh: &DisplayHandle, id: ObjectId) -> Result<Self, InvalidId>;

    fn send_event(&self, evt: Self::Event) -> Result<(), InvalidId>;

    fn parse_request(
        dh: &DisplayHandle,
        msg: Message<ObjectId>,
    ) -> Result<(Self, Self::Request), DispatchError>;

    fn write_event(
        &self,
        dh: &DisplayHandle,
        req: Self::Event,
    ) -> Result<Message<ObjectId>, InvalidId>;

    #[inline]
    fn post_error(&self, code: impl Into<u32>, error: impl Into<String>) {
        if let Some(dh) = self.handle().upgrade().map(DisplayHandle::from) {
            dh.post_error(self, code.into(), error.into());
        }
    }

    #[doc(hidden)]
    fn __set_object_data(
        &mut self,
        odata: std::sync::Arc<dyn std::any::Any + Send + Sync + 'static>,
    );
}

#[derive(thiserror::Error, Debug)]
pub enum DispatchError {
    #[error("Bad message for interface {interface} : {msg:?}")]
    BadMessage { msg: Message<ObjectId>, interface: &'static str },
    #[error("Unexpected interface {interface} for message {msg:?}")]
    NoHandler { msg: Message<ObjectId>, interface: &'static str },
}
