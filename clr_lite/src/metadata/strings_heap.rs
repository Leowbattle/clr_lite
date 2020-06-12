use std::cell::RefCell;
use std::collections::HashMap;

/// ECMA-335 II.24.2.3
pub struct StringsHeap<'data> {
	data: &'data [u8],
	cache: RefCell<HashMap<usize, &'data str>>,
}

def_handle!(StringHandle);

impl<'data> StringsHeap<'data> {
	pub fn new(data: &'data [u8]) -> Self {
		Self {
			data,
			cache: Default::default(),
		}
	}

	pub fn get(&self, index: StringHandle) -> Option<&'data str> {
		let index = index.into();

		if index > self.data.len() {
			None
		} else {
			Some(
				self.cache.borrow_mut().entry(index).or_insert(
					std::str::from_utf8(
						&self.data
							[index..index + self.data[index..].iter().position(|&c| c == b'\0')?],
					)
					.ok()?,
				),
			)
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_valid() {
		let sh = StringsHeap::new(b"hello\0world\0");
		assert_eq!(sh.get(0.into()), Some("hello"));
		assert_eq!(sh.get(0.into()), Some("hello"));
		assert_eq!(sh.get(1.into()), Some("ello"));
		assert_eq!(sh.get(5.into()), Some(""));
		assert_eq!(sh.get(6.into()), Some("world"));
	}

	#[test]
	fn test_out_of_range() {
		let sh = StringsHeap::new(b"he\0");
		assert_eq!(sh.get(10.into()), None);
	}

	#[test]
	fn test_non_ending() {
		let sh = StringsHeap::new(b"hello\0world");
		assert_eq!(sh.get(6.into()), None);
	}

	#[test]
	fn test_invalid_utf8() {
		let sh = StringsHeap::new(&[0xd8, 0x00]);
		assert_eq!(sh.get(0.into()), None);
	}
}
