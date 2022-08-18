#![warn(missing_docs)]

//! Bluest is a cross-platform Bluetooth Low Energy (BLE) crate. It currently supports Windows (version 10 and later)
//! and MacOS/iOS. Linux and Android support are planned.
//!
//! The crate currently supports the GAP Central and GATT Client roles. Peripheral and Server roles are not supported.
//!
//! ## Usage
//!
//! ```rust,no_run
//! ```

pub mod btuuid;
pub mod error;

#[cfg(any(target_os = "macos", target_os = "ios"))]
mod corebluetooth;
#[cfg(target_os = "windows")]
mod windows;

#[cfg(any(target_os = "macos", target_os = "ios"))]
use crate::corebluetooth as sys;
#[cfg(target_os = "windows")]
use crate::windows as sys;

use enumflags2::bitflags;
// Dependency re-exports
pub use smallvec;
pub use uuid;

pub use error::Error;

/// Convenience alias for a result with [Error]
pub type Result<T, E = Error> = core::result::Result<T, E>;

pub use btuuid::BluetoothUuidExt;

pub use sys::adapter::Adapter;
pub use sys::characteristic::Characteristic;
pub use sys::descriptor::Descriptor;
pub use sys::device::{Device, DeviceId};
pub use sys::service::Service;

use smallvec::SmallVec;
use std::collections::HashMap;
use uuid::Uuid;

/// Events generated by [crate::Adapter]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AdapterEvent {
    /// The adapter has become available (powered on and ready to use)
    Available,
    /// The adapter has become unavailable (powered off or otherwise disabled)
    Unavailable,
}

/// Represents a device discovered during a scan operation
pub struct AdvertisingDevice {
    /// The source of the advertisement
    pub device: crate::Device,
    /// The advertisment data
    pub adv_data: AdvertisementData,
    /// The signal strength in dBm of the received advertisement packet
    pub rssi: Option<i16>,
}

/// Data included in a Bluetooth advertisement or scan reponse.
#[derive(Debug, Clone)]
pub struct AdvertisementData {
    /// The (possibly shortened) local name of the device (CSS §A.1.2)
    pub local_name: Option<String>,
    /// Manufacturer specific data (CSS §A.1.4)
    pub manufacturer_data: Option<ManufacturerData>,
    /// Advertised GATT service UUIDs (CSS §A.1.1)
    pub services: SmallVec<[Uuid; 1]>,
    /// Solicited GATT service UUIDs (CSS §A.1.10)
    pub solicited_services: SmallVec<[Uuid; 1]>,
    /// Service associated data (CSS §A.1.11)
    pub service_data: HashMap<Uuid, SmallVec<[u8; 16]>>,
    /// Transmitted power level (CSS §A.1.5)
    pub tx_power_level: Option<i16>,
    /// Set to true for connectable advertising packets
    pub is_connectable: bool,
}

/// Manufacturer specific data included in Bluetooth advertisements. See the Bluetooth Core Specification Supplement
/// §A.1.4 for details.
#[derive(Debug, Clone)]
pub struct ManufacturerData {
    /// Company identifier (defined [here](https://www.bluetooth.com/specifications/assigned-numbers/company-identifiers/))
    pub company_id: u16,
    /// Manufacturer specific data
    pub data: SmallVec<[u8; 16]>,
}

/// GATT characteristic properties as defined in the Bluetooth Core Specification, Vol 3, Part G, §3.3.1.1.
/// Extended properties are also included in the upper bits as defined in §3.3.3.1.
#[allow(missing_docs)]
#[bitflags]
#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum CharacteristicProperty {
    Broadcast = 0x01,
    Read = 0x02,
    WriteWithoutResponse = 0x04,
    Write = 0x08,
    Notify = 0x10,
    Indicate = 0x20,
    AuthenticatedSignedWrites = 0x40,
    ExtendedProperties = 0x80,
    ReliableWrite = 0x0100,
    WritableAuxiliaries = 0x0200,
}
