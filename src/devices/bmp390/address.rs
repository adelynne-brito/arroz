use defmt::Format;

const PRIMARY_ADDRESS: u8 = 0x76;
const SECONDARY_ADDRESS: u8 = 0x77;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Format)]
pub enum Address {
	/// Primary device address 0x77 (SDO pulled low)
	Primary,
	/// Secondary device address 0x78 (SDO pulled high)
	Secondary,
	Custom(u8),
}

impl From<Address> for u8 {
	fn from(address: Address) -> Self {
		match address {
			Address::Primary => PRIMARY_ADDRESS,
			Address::Secondary => SECONDARY_ADDRESS,
			Address::Custom(x) => x,
		}
	}
}