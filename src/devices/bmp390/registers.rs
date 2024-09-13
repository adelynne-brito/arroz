use crate::devices::bmp390::enums::{IIRFilter, InteruptOutput, PowerMode, OversamplingSetting};
use bondrewd::BitfieldEnum;
use embedded_devices_derive::device_register;
use embedded_registers::register;
use crate::common::enums::LogicLevel;

// msb0
// 000 > 000 > 000


/// This register contains the chip ID
#[device_register(super::BMP390)]
#[register(address = 0x00, mode = "r")]
#[bondrewd(read_from = "msb0", default_endianness = "le", enforce_bytes = 1)]
pub struct ChipID {
	pub chip_id: u8
}

/// This register contains the Chip Revision
#[device_register(super::BMP390)]
#[register(address = 0x01, mode = "r")]
#[bondrewd(read_from = "msb0", default_endianness = "le", enforce_bytes = 1)]
pub struct Revision {
	pub rev_id: u8
}


/// This register contains an error code in case of a failed command
#[device_register(super::BMP390)]
#[register(address = 0x02, mode = "r")]
#[bondrewd(read_from = "msb0", default_endianness = "le", enforce_bytes = 1)]
pub struct Error {
	#[bondrewd(bit_length = 5, reserve)]
	reserve: u8,

	pub conf_err: bool,
	pub cmd_err: bool,
	pub fatal_err: bool,
}

/// This register indicates whether a certain type of data is ready to be read
#[device_register(super::BMP390)]
#[register(address = 0x02, mode = "r")]
#[bondrewd(read_from = "msb0", default_endianness = "le", enforce_bytes = 1)]
pub struct Status {
	#[bondrewd(bit_length = 1, reserve)]
	reserve: u8,

	pub data_ready_temp: bool,
	pub data_ready_pres: bool,
	pub command_ready: bool,
	#[bondrewd(bit_length = 4, reserve)]
	reserve2: u8
}


/// This register contains the pressure reading
#[device_register(super::BMP390)]
#[register(address = 0x04, mode = "r")]
#[bondrewd(read_from = "msb0", default_endianness = "le", enforce_bytes = 3)]
pub struct Pressure {
	#[bondrewd(bit_length = 24)]
	pub pressure: u32
}


/// This register contains the temperature reading
#[device_register(super::BMP390)]
#[register(address = 0x07, mode = "r")]
#[bondrewd(read_from = "msb0", default_endianness = "le", enforce_bytes = 3)]
pub struct Temperature {
	#[bondrewd(bit_length = 24)]
	pub temperature: u32
}


/// Todo! verify that this reads correctly
#[device_register(super::BMP390)]
#[register(address = 0x04, mode = "r")]
#[bondrewd(read_from = "msb0", default_endianness = "le", enforce_bytes = 6)]
pub struct BurstRead {
	#[bondrewd(bit_length = 24)]
	pub pressure: u32,
	#[bondrewd(bit_length = 24)]
	pub temperature: u32
}


/// This register contains the sensor time reading
#[device_register(super::BMP390)]
#[register(address = 0x0C, mode = "r")]
#[bondrewd(read_from = "msb0", default_endianness = "le", enforce_bytes = 3)]
pub struct SensorTime {
	#[bondrewd(bit_length = 24)]
	pub time: u32
}

#[device_register(super::BMP390)]
#[register(address = 0x10, mode = "r")]
#[bondrewd(read_from = "msb0", default_endianness = "le", enforce_bytes = 1)]
pub struct Event {
	#[bondrewd(bit_length = 6, reserve)]
	reserve: u8,
	/// Serial transaction occured during conversion(clear on read)
	pub itf_act_pt: bool,
	/// Power on detected,(clear on read)
	pub por_detected: bool,
}

/// Shows interupt status and is cleared after reading
#[device_register(super::BMP390)]
#[register(address = 0x11, mode = "r")]
#[bondrewd(read_from = "msb0", default_endianness = "le", enforce_bytes = 1)]
pub struct InterruptStatus {
	#[bondrewd(bit_length = 4, reserve)]
	reserve: u8,

	pub data_ready: bool,
	
	#[bondrewd(bit_length = 1, reserve)]
	reserve2: u8,

	pub fifo_full: bool,
	pub fifo_watermark: bool,
}


//todo! This should be 9 bits long but the read and write
#[device_register(super::BMP390)]
#[register(address = 0x12, mode = "r")]
#[bondrewd(read_from = "msb0", default_endianness = "le", enforce_bytes = 2)]
pub struct FIFOLength {
	pub fifo_byte_counter: u16
}

// todo! Figure out FIFO and see if its worth the effort
#[device_register(super::BMP390)]
#[register(address = 0x14, mode = "r")]
#[bondrewd(read_from = "msb0", default_endianness = "le", enforce_bytes = 1)]
pub struct FIFOData {
	pub fifo_data: u8
}


#[device_register(super::BMP390)]
#[register(address = 0x15, mode = "rw")]
#[bondrewd(read_from = "msb0", default_endianness = "le", enforce_bytes = 2)]
pub struct FIFOWatermark {
	#[bondrewd(bit_length = 9)]
	pub watermark: u16,
	#[bondrewd(bit_length = 7, reserve)]
	#[allow(dead_code)]
	pub reserved: u8,
}

#[device_register(super::BMP390)]
#[register(address = 0x17, mode = "rw")]
#[bondrewd(read_from = "msb0", default_endianness = "le", enforce_bytes = 1)]
pub struct FIFOConfig1 {
	#[bondrewd(bit_length = 3, reserve)]
	reserve: u8,

	/// Include temp in FIFO frame
	pub fifo_temp_en: bool,
	/// Include press in FIFO frame
	pub fifo_press_en: bool,
	/// Append time to FIFO frame
	pub fifo_time_en: bool,
	pub fifo_stop_on_full: bool,
	pub fifo_enable: bool,
}

#[device_register(super::BMP390)]
#[register(address = 0x18, mode = "rw")]
#[bondrewd(read_from = "msb0", default_endianness = "le", enforce_bytes = 1)]
pub struct FIFOConfig2 {
	#[bondrewd(bit_length = 3, reserve)]
	reserve: u8,
	/// Select data source for FIFO
	#[bondrewd(bit_length = 2)]
	pub data_select: u8,
	/// FIFO downsampling selection for pressure and temperature data, factor is
	/// 2^fifo_subsampling
	#[bondrewd(bit_length = 3)]
	pub fifo_subsampling: u8,
}

#[device_register(super::BMP390)]
#[register(address = 0x19, mode = "rw")]
#[bondrewd(read_from = "msb0", default_endianness = "le", enforce_bytes = 1)]
pub struct InteruptControl {
	#[bondrewd(bit_length = 1, reserve)]
	reserve: u8,
	/// Enable interupt for when temp and press data is ready
	pub data_ready_int: bool,
	pub int_ds: bool,
	/// Enable interupt when FIFO buffer is full
	pub fifo_full_int: bool,
	/// Enable interrupt when FIFO buffer reaches water mark
	pub fifo_watermark_int: bool,
	/// Latching of interupts for INT pin and INT_STATUS register
	pub int_latch: bool,
	/// Specifies whether the interupt pin is active low or high
	#[bondrewd(bit_length = 1,enum_primitive = "u8")]
	pub int_active_level: LogicLevel,
	/// Specifies whether the interupt pin is open drain or push pull
	#[bondrewd(bit_length = 1,enum_primitive = "u8")]
	pub int_od: InteruptOutput,
}

#[device_register(super::BMP390)]
#[register(address = 0x1A, mode = "rw")]
#[bondrewd(read_from = "msb0", default_endianness = "le", enforce_bytes = 1)]
pub struct IFConf {
	#[bondrewd(bit_length = 5, reserve)]
	reserve: u8,
	i2c_wdt_sel: bool,
	i2c_wdt_en: bool,
	spi3: bool
}

#[device_register(super::BMP390)]
#[register(address = 0x1B, mode = "rw")]
#[bondrewd(read_from = "msb0", default_endianness = "le", enforce_bytes = 1)]
pub struct PowerControl {
	#[bondrewd(bit_length = 2, reserve)]
	reserve: u8,
	#[bondrewd(bit_length = 2,enum_primitive = "u8")]
	pub mode: PowerMode,
	#[bondrewd(bit_length = 2, reserve)]
	reserve2: u8,
	/// Enable Temperature Reading
	pub temp_en: bool,
	/// Enable Pressure Reading
	pub press_en: bool,
}

#[device_register(super::BMP390)]
#[register(address = 0x1C, mode = "rw")]
#[bondrewd(read_from = "msb0", default_endianness = "le", enforce_bytes = 1)]
pub struct Oversampling {
	#[bondrewd(bit_length = 2, reserve)]
	reserve: u8,
	#[bondrewd(bit_length = 3,enum_primitive = "u8")]
	pub oversampling_temp: OversamplingSetting,
	#[bondrewd(bit_length = 3,enum_primitive = "u8")]
	pub over_sampling_press: OversamplingSetting,
}

#[device_register(super::BMP390)]
#[register(address = 0x1D, mode = "rw")]
#[bondrewd(read_from = "msb0", default_endianness = "le", enforce_bytes = 1)]
pub struct OutputDataRate {
	#[bondrewd(bit_length = 3, reserve)]
	reserve: u8,
	/// The subdivision factor = 2^value range: 0-17
	/// See datasheet for more nfo
	#[bondrewd(bit_length = 5)]
	pub subdivision_factor: u8,
}

#[device_register(super::BMP390)]
#[register(address = 0x1F, mode = "rw")]
#[bondrewd(read_from = "msb0", default_endianness = "le", enforce_bytes = 1)]
pub struct IIRFilterConfiguration {
	#[bondrewd(bit_length = 4, reserve)]
	reserve: u8,
	#[bondrewd(bit_length = 3,enum_primitive = "u8")]
	pub iir_filter: IIRFilter,
	short_in: bool
	
}

#[device_register(super::BMP390)]
#[register(address = 0x7E, mode = "w")]
#[bondrewd(read_from = "msb0", default_endianness = "le", enforce_bytes = 1)]
pub struct Command {
	command: u8
}


/// Device-internal trimming coefficients (calibration registers)
#[device_register(super::BMP390)]
#[register(address = 0x31, mode = "r")]
#[bondrewd(read_from = "msb0", default_endianness = "le", enforce_bytes = 21)]
pub struct CalibrationICoefficients {
	pub par_t1: u16,
	pub par_t2: u16,
	pub par_t3: i8,
	pub par_p1: i16,
	pub par_p2: i16,
	pub par_p3: i8,
	pub par_p4: i8,
	pub par_p5: u16,
	pub par_p6: u16,
	pub par_p7: i8,
	pub par_p8: i8,
	pub par_p9: i16,
	pub par_p10: i8,
	pub par_p11: i8,
}