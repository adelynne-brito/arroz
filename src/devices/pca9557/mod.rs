//! This module contains a driver for the [`PCA9557`] I2C IO expander IC.
//!
//! [`PCA9557`]: https://www.ti.com/lit/ds/symlink/pca9557.pdf
//!
mod registers;
mod address;

// This serves to export the enums used by registers
pub mod enums {
	pub use super::registers::{Polarity,IODirection};
	pub use super::address::Address;
}

use crate::devices::pca9557::registers::{Configuration, IODirection};
use embedded_devices_derive::{device, device_impl};
use embedded_registers::RegisterInterface;

#[device]
pub struct PCA9557<I: RegisterInterface> {
	interface: I,
}

#[device_impl]
impl<I: RegisterInterface> PCA9557<I> {

	pub async fn get_input_port(&mut self) -> Result<[LogicLevel;8], I::Error> {
		 Ok(self.read_register::<InputPort>().await?.read_pins())
	}

	pub async fn get_output_port(&mut self) -> Result<[LogicLevel;8], I::Error> {
		Ok(self.read_register::<OutputPort>().await?.read_pins())
	}
	
	pub async fn get_polarity(&mut self) -> Result<[Polarity;8], I::Error> {
		Ok(self.read_register::<PolarityInversion>().await?.read_pins())
	}

	pub async fn get_configuration(&mut self) -> Result<[IODirection;8], I::Error> {
		Ok(self.read_register::<Configuration>().await?.read_pins())
	}

	pub async fn set_output_port(&mut self, mut input: [LogicLevel;8]) -> Result<(), I::Error> {
		input.reverse();
		let mut reg = self.read_register::<OutputPort>().await?;
		reg.write_pins(input);
		self.write_register(reg).await?;
		Ok(())
	}
	
	pub async fn set_polarity(&mut self, mut input: [Polarity;8]) -> Result<(), I::Error> {
		input.reverse(); // reverse the input so that the index used by the user matches the chip
		let mut reg = self.read_register::<PolarityInversion>().await?;
		reg.write_pins(input);
		self.write_register(reg).await?;
		Ok(())
	}
	
	pub async fn set_configuration(&mut self,mut input: [IODirection;8]) -> Result<(), I::Error> {
		input.reverse(); // reverse the input so that the index used by the user matches the chip
		let mut reg = self.read_register::<Configuration>().await?;
		reg.write_pins(input);
		self.write_register(reg).await?;
		Ok(())
	}


}

use embedded_hal_async as hal;
use embedded_registers::i2c::I2cDevice;
use crate::common::enums::LogicLevel;
use crate::devices::pca9557::address::Address;
use crate::devices::pca9557::registers::{InputPort, OutputPort, Polarity, PolarityInversion};

type PCA9557Codec = embedded_registers::i2c::codecs::OneByteRegAddrCodec;

impl<I> PCA9557<I2cDevice<I,hal::i2c::SevenBitAddress,PCA9557Codec>>
where I: hal::i2c::I2c<hal::i2c::SevenBitAddress> + hal::i2c::ErrorType{
	/// Initializes a new device with the given address on the specified bus.
	/// This consumes the I2C bus `I`.
	///
	/// Before using this device, you must call the [`Self::init`] method which
	/// initializes the device and ensures that it is working correctly.
	pub fn new_i2c(interface: I, address: Address) -> Self {
		Self {
			interface: I2cDevice::new(interface,address.into(), PCA9557Codec::default()),
		}
	}


}


#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_inversion_register() {
		let mut test_config = registers::PolarityInversion::default();
		let mut pins = test_config.read_pins();
		pins[0] = registers::Polarity::Original;
		pins[1] = registers::Polarity::Original;
		pins[2] = registers::Polarity::Original;
		pins[3] = registers::Polarity::Original;
		pins.reverse();

		let inversion_check = 0b11110000u8;

		test_config.write_pins(pins);
				assert_eq!(test_config.data[0], inversion_check,
				"We are expecting {:#010b} and actually got {:#010b}",inversion_check,test_config.data[0]
				);
	}
}