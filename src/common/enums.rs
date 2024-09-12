use bondrewd::BitfieldEnum;

/// An enum reflecting logic level of a pin, bus etc.
#[derive(BitfieldEnum, Copy, Clone, Default, PartialEq, Eq, Debug, defmt::Format)]
#[bondrewd_enum(u8)]
pub enum LogicLevel {
	High = 1,
	#[default]
	Low = 0
}