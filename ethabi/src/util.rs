//! Utils used by different modules.

use Word;

/// Converts u32 to right aligned array of 32 bytes.
pub fn pad_u32(value: u32) -> Word {
	let mut padded = [0u8; 32];
	padded[28] = (value >> 24) as u8;
	padded[29] = (value >> 16) as u8;
	padded[30] = (value >> 8) as u8;
	padded[31] = value as u8;
	padded
}

/// Converts i32 to right aligned array of 32 bytes.
pub fn pad_i32(value: i32) -> Word {
	if value >= 0 {
		return pad_u32(value as u32);
	}

	let mut padded = [0xffu8; 32];
	padded[28] = (value >> 24) as u8;
	padded[29] = (value >> 16) as u8;
	padded[30] = (value >> 8) as u8;
	padded[31] = value as u8;
	padded
}

#[cfg(test)]
mod tests {
	use super::{pad_u32, pad_i32};

	#[test]
	fn test_pad_u32() {
		// this will fail if endianess is not supported
        assert_eq!(pad_u32(0).to_vec(), hex!("0000000000000000000000000000000000000000000000000000000000000000").to_vec());
        assert_eq!(pad_u32(1).to_vec(), hex!("0000000000000000000000000000000000000000000000000000000000000001").to_vec());
        assert_eq!(pad_u32(0x100).to_vec(), hex!("0000000000000000000000000000000000000000000000000000000000000100").to_vec());
        assert_eq!(pad_u32(0xffffffff).to_vec(), hex!("00000000000000000000000000000000000000000000000000000000ffffffff").to_vec());
	}

	#[test]
	fn test_i32() {
		assert_eq!(pad_i32(0).to_vec(), hex!("0000000000000000000000000000000000000000000000000000000000000000").to_vec());
		assert_eq!(pad_i32(-1).to_vec(), hex!("ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff").to_vec());
		assert_eq!(pad_i32(-2).to_vec(), hex!("fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe").to_vec());
		assert_eq!(pad_i32(-256).to_vec(), hex!("ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff00").to_vec());
	}
}
