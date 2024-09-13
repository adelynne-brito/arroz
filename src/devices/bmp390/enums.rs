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
	x1 = 0b000,
	x2 = 0b001,
	x4 = 0b010,
	x8 = 0b011,
	x16 = 0b100,
	x32 = 0b101,
}


#[derive(BitfieldEnum, Copy, Clone, Default, PartialEq, Eq, Debug, defmt::Format)]
#[bondrewd_enum(u8)]
pub enum IIRFilter {
	#[default]
	Coeff_0= 0b000,
	Coeff_1= 0b001,
	Coeff_3= 0b010,
	Coeff_7= 0b011,
	Coeff_15= 0b100,
	Coeff_31= 0b101,
	Coeff_63= 0b110,
	Coeff_127= 0b111,

}