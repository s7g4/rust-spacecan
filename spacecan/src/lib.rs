#![no_std]

#[cfg(feature = "std")]
extern crate std;

pub mod primitives;
pub mod protocol;
pub mod services;
pub mod transport;

pub type PacketData = heapless::Vec<u8, 1024>;
pub type FrameData = heapless::Vec<u8, 8>;
pub type ParamList = heapless::Vec<u16, 64>;

#[cfg(test)]
pub mod tests;
