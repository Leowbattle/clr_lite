///! ECMA-335 II.23.2

pub fn compress_u32(x: u32) -> u32 {
	match x {
		0..=0x7f => x,
		0x80..=0x3fff => x | 0x8000,
		_ => x | 0xc000_0000,
	}
}

pub fn decompress_u32(x: u32) -> u32 {
	if x & 0xc000_0000 != 0 {
		x & !0xc000_0000
	} else if x & 0x8000 != 0 {
		x & !0x8000
	} else {
		x
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	const VALS: &[(u32, u32)] = &[
		(0x3, 0x3),
		(0x7f, 0x7f),
		(0x80, 0x8080),
		(0x2e57, 0xae57),
		(0x3fff, 0xbfff),
		(0x4000, 0xc000_4000),
		(0x1fff_ffff, 0xdfff_ffff),
	];

	#[test]
	fn test_encode() {
		VALS.iter()
			.for_each(|&(v, e)| assert_eq!(compress_u32(v), e));
	}

	#[test]
	fn test_decode() {
		VALS.iter()
			.for_each(|&(v, e)| assert_eq!(decompress_u32(e), v));
	}
}
