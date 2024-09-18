
	use super::*;
	use assert_hex::assert_eq_hex;

	#[test]
	
	fn test_serialize() {
		let test_packet = TestPacket{
			test: 0x080f // crc in le 0xC89A8602
		};
		let packet = test_packet.serialize();
		
		let test_valid = [0xb0,0x0b,0x02,0x86,0x9a,0xc8,0xFF,0x2,0x0f,0x08,0xa0,0x0a];
		assert_eq_hex!(packet.as_slice(), test_valid);
		
	}