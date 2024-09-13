use crate::devices::bmp390::registers::{IIRFilterConfiguration, InteruptControl, OutputDataRate, Oversampling};
use embedded_devices_derive::{device, device_impl};
use embedded_registers::i2c::I2cDevice;
use embedded_registers::RegisterInterface;
use address::Address;

pub mod address;
pub mod registers;
pub mod enums;

#[device]
pub struct BMP390<I: RegisterInterface> {
	interface: I,
}

#[device_impl]
impl<I: RegisterInterface>BMP390<I> {



	pub async fn set_power_mode(&mut self, mode: PowerMode, enable_press:bool,enable_temp:bool) -> Result<(), I::Error> {
		let mut reg = self.read_register::<PowerControl>().await?;
		reg.write_mode(mode);
		reg.write_press_en(enable_press);
		reg.write_temp_en(enable_temp);
		self.write_register(reg).await?;
		Ok(())
	}

	pub async fn set_oversample(&mut self, temp_oversample: OversamplingSetting, press_oversample: OversamplingSetting)  -> Result<(), I::Error> {
		let mut reg = self.read_register::<Oversampling>().await?;
		reg.write_oversampling_temp(temp_oversample);
		reg.write_over_sampling_press(press_oversample);
		self.write_register(reg).await?;
		Ok(())

	}

	pub async fn set_output_data_rate(&mut self, output_data_rate: u8)-> Result<(), I::Error> {
		let mut reg = self.read_register::<OutputDataRate>().await?;
		reg.write_subdivision_factor(output_data_rate);
		self.write_register(reg).await?;
		Ok(())

	}

	pub async fn set_iir_filter(&mut self, coef: IIRFilter)-> Result<(), I::Error> {
		let mut reg = self.read_register::<IIRFilterConfiguration>().await?;
		reg.write_iir_filter(coef);
		reg.write_short_in(false);
		self.write_register(reg).await?;
		Ok(())
	}
	
	pub async fn set_interrupt(&mut self, data_ready: bool, fifo_full:bool, fifo_watermark:bool,int_latch:bool,int_active_level:LogicLevel,int_od: InteruptOutput)
	-> Result<(), I::Error> {
		let mut reg = InteruptControl::default();
		reg.write_data_ready_int(data_ready);
		reg.write_fifo_full_int(fifo_full);
		reg.write_fifo_watermark_int(fifo_watermark);
		reg.write_int_latch(int_latch);
		reg.write_int_active_level(int_active_level);
		reg.write_int_od(int_od);
		self.write_register(reg).await?;
		Ok(())
	}



}

type BMP390Codec = embedded_registers::i2c::codecs::OneByteRegAddrCodec;


use embedded_hal_async as hal;
use crate::common::enums::LogicLevel;
use crate::devices::bmp390::enums::{IIRFilter, InteruptOutput, OversamplingSetting, PowerMode};
use crate::devices::bmp390::registers::PowerControl;

impl<I> BMP390<I2cDevice<I,hal::i2c::SevenBitAddress,BMP390Codec>>
where I: hal::i2c::I2c<hal::i2c::SevenBitAddress> + hal::i2c::ErrorType{
	/// Initializes a new device with the given address on the specified bus.
	/// This consumes the I2C bus `I`.
	///
	/// Before using this device, you must call the [`Self::init`] method which
	/// initializes the device and ensures that it is working correctly.
	pub fn new_i2c(interface: I, address: Address) -> Self {
		Self {
			interface: I2cDevice::new(interface,address.into(), BMP390Codec::default())
		}
	}


}