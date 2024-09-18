#[macro_use]

#[cfg(test)]
mod tests;

use alloc::vec::Vec;

macro_rules! impl_packet {
    ($T:ident,$p:path) => {
		impl DataPacket for $T {
			fn get_type(&self) -> PacketType  {$p}
			fn serialize(&self) -> Vec<u8> {
				let data = self.into_bytes();
				let mut out: Vec<u8> = Vec::with_capacity(data.len()+7);
				out.extend(BEGIN_PACKET);
				out.push(self.get_type() as u8);
				out.push(data.len() as u8);
				out.extend(data);
				out.extend(END_PACKET);
				out
			}

		}
    }
}



pub trait DataPacket{
	fn get_type(&self) -> PacketType;
	fn serialize(&self)->Vec<u8>;
}

#[repr(u8)]
pub enum PacketType{
	Accelerometer = 0x01,
	Test = 0xFF,
}

const BEGIN_PACKET: [u8;2] = [0x1F, 0x2F];
const END_PACKET: [u8;2] = [0x3F, 0x4F];


use bondrewd::Bitfields;

#[derive(Bitfields, Copy, Clone)]
#[bondrewd(default_endianness = "le")]
pub struct SixAxisIMUPacket {
	pub acc_x:u16,
	pub acc_y:u16,
	pub acc_z:u16,
	pub gyr_x:u16,
	pub gyr_y:u16,
	pub gyr_z:u16,
}
impl_packet!(SixAxisIMUPacket,PacketType::Accelerometer);

#[derive(Bitfields, Copy, Clone)]
#[bondrewd(default_endianness = "le")]
pub struct TestPacket {
	pub test:u16,
}

impl_packet!(TestPacket,PacketType::Test);


impl From<PacketType> for u8 {
	fn from(packet_type: PacketType) -> Self {
		match packet_type {
			PacketType::Accelerometer => 0x00,
			PacketType::Test => 0xFF,
		}
	}
}

