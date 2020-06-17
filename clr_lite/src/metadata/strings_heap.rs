use std::cell::RefCell;
use std::collections::HashMap;

pub struct StringsHeap<'data> {
	data: &'data [u8],
	cache: RefCell<HashMap<StringHandle, &'data str>>,
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct StringHandle(pub(crate) usize);

impl<'data> StringsHeap<'data> {
	pub(crate) fn new(data: &'data [u8]) -> Self {
		StringsHeap {
			data,
			cache: Default::default(),
		}
	}

	pub fn get(&self, index: StringHandle) -> Option<&'data str> {
		if index.0 > self.data.len() {
			None
		} else {
			Some(
				self.cache.borrow_mut().entry(index).or_insert(
					std::str::from_utf8(
						&self.data[index.0
							..index.0 + self.data[index.0..].iter().position(|&c| c == b'\0')?],
					)
					.ok()?,
				),
			)
		}
	}
}
