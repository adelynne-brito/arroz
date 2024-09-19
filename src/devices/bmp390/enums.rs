use bondrewd::BitfieldEnum;

/// This specifies whether the respective pin is pushpull or open drain
#[derive(BitfieldEnum, Copy, Clone, Default, PartialEq, Eq, Debug, defmt::Format)]
#[bondrewd_enum(u8)]
pub enum InteruptOutput {
	OpenDrain = 1,
	#[default]
	PushPull = 0
}




// An enum reflecting logic level of a pin, bus etc.
#[derive(BitfieldEnum, Copy, Clone, Default, PartialEq, Eq, Debug, defmt::Format)]
#[bondrewd_enum(u8)]
pub enum PowerMode {
	Sleep = 0b00,
	Forced = 0b01,
	#[default]
	Normal = 0b11
}


#[derive(BitfieldEnum, Copy, Clone, Default, PartialEq, Eq, Debug, defmt::Format)]
#[bondrewd_enum(u8)]
pub enum OversamplingSetting {
	#[default]
	X1 = 0b000,
	X2 = 0b001,
	X4 = 0b010,
	X8 = 0b011,
	X16 = 0b100,
	X32 = 0b101,
}


#[derive(BitfieldEnum, Copy, Clone, Default, PartialEq, Eq, Debug, defmt::Format)]
#[bondrewd_enum(u8)]
pub enum IIRFilter {
	#[default]
	Coeff0= 0b000,
	Coeff1 = 0b001,
	Coeff3= 0b010,
	Coeff7= 0b011,
	Coeff15= 0b100,
	Coeff31= 0b101,
	Coeff63= 0b110,
	Coeff127= 0b111,

}