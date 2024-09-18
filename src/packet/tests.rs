
	use super::*;
	use assert_hex::assert_eq_hex;

	#[test]
	
	fn test_serialize() {
		let test_packet = TestPacket{
			test: 0x080f
		};
		let packet = test_packet.serialize();
		
		let test_valid = [0x1f,0x2f,0xFF,0x2,0x0f,0x08,0x3f,0x4f];
		assert_eq_hex!(packet.as_slice(), test_valid);
		
	}