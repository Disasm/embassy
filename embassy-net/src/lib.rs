#![cfg_attr(not(feature = "std"), no_std)]
#![feature(const_fn)]
#![feature(const_in_array_repeat_expressions)]
#![feature(const_generics)]
#![feature(const_evaluatable_checked)]
#![allow(incomplete_features)]

// This mod MUST go first, so that the others see its macros.
pub(crate) mod fmt;

mod pool; // TODO extract to embassy, or to own crate

mod config;
mod device;
mod packet_pool;
mod stack;
mod tcp_socket;

pub use config::{Config, Configurator, DhcpConfigurator, StaticConfigurator, UpConfig};
pub use device::{Device, LinkState};
pub use packet_pool::{Packet, PacketBox, PacketBuf};
pub use stack::{init, is_init, run};
pub use tcp_socket::TcpSocket;

#[cfg(feature = "std")]
mod tuntap;
#[cfg(feature = "std")]
pub use tuntap::{TunTap, TunTapDevice};

// smoltcp reexports
pub use smoltcp::phy::{DeviceCapabilities, Medium};
pub use smoltcp::time::Duration as SmolDuration;
pub use smoltcp::time::Instant as SmolInstant;
pub use smoltcp::wire::{IpAddress, IpCidr, Ipv4Address, Ipv4Cidr};
pub type Interface = smoltcp::iface::Interface<'static, device::DeviceAdapter>;
pub type SocketSet = smoltcp::socket::SocketSet<'static>;
