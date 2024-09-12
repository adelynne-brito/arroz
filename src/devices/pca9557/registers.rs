use bondrewd::BitfieldEnum;
use embedded_devices_derive::device_register;
use embedded_registers::register;

use crate::common::enums::LogicLevel;

/// This register reflects the incoming logic level of pins, regardless of configuration
#[device_register(super::PCA9557)]
#[register(address = 0x00, mode = "r")]
#[bondrewd(read_from = "msb0", default_endianness = "be", enforce_bytes = 1)]
pub struct InputPort {
	#[bondrewd(enum_primitive = "u8", element_bit_length = 1)]
	pub pins: [LogicLevel;8]

}

/// This register shows the outgoing logic levels of the pins defined as outputs
#[device_register(super::PCA9557)]
#[register(address = 0x01, mode = "rw")]
#[bondrewd(read_from = "msb0", default_endianness = "be", enforce_bytes = 1)]
pub struct OutputPort {
	#[bondrewd(enum_primitive = "u8", element_bit_length = 1)]
	pub pins: [LogicLevel;8]
}

#[derive(BitfieldEnum, Copy, Clone, Default, PartialEq, Eq, Debug, defmt::Format)]
#[bondrewd_enum(u8)]
pub enum Polarity {
	#[default]
	Inverted = 1,
	Original = 0
}

#[device_register(super::PCA9557)]
#[register(address = 0x02, mode = "rw")]
#[bondrewd(read_from = "msb0", default_endianness = "le", enforce_bytes = 1)]
pub struct PolarityInversion {
	#[bondrewd(enum_primitive = "u8", element_bit_length = 1)]
	pub pins: [Polarity;8]
}

#[derive(BitfieldEnum, Copy, Clone, Default, PartialEq, Eq, Debug, defmt::Format)]
#[bondrewd_enum(u8)]
pub enum IODirection {
	#[default]
	Input = 1,
	Output = 0
}

#[device_register(super::PCA9557)]
#[register(address = 0x03, mode = "rw")]
#[bondrewd(read_from = "msb0", default_endianness = "be", enforce_bytes = 1)]
pub struct Configuration {
	#[bondrewd(enum_primitive = "u8", element_bit_length = 1)]
	pub pins: [IODirection;8]
}