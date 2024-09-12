use defmt::Format;

const PRIMARY_ADDRESS: u8 = 0x18;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Format)]
pub enum Address {
	/// Primary device address 0x24 (A0, A1, A2 connected to GND).
	Primary,
	Custom(u8),
}

impl From<Address> for u8 {
	fn from(address: Address) -> Self {
		match address {
			Address::Primary => PRIMARY_ADDRESS,
			Address::Custom(x) => x,
		}
	}
}