use std::cell::RefCell;
use std::collections::HashMap;

pub struct StringsHeap<'data> {
	data: &'data [u8],
	cache: RefCell<HashMap<StringHandle, &'data str>>,
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct StringHandle(usize);

impl StringHandle {
	fn index(self) -> usize {
		self.0
	}
}

impl From<usize> for StringHandle {
	fn from(x: usize) -> StringHandle {
		StringHandle(x)
	}
}

impl<'data> StringsHeap<'data> {
	pub(crate) fn new(data: &'data [u8]) -> Self {
		StringsHeap {
			data,
			cache: Default::default(),
		}
	}

	pub fn get(&self, index: StringHandle) -> Option<&'data str> {
		if index.index() > self.data.len() {
			None
		} else {
			Some(
				self.cache.borrow_mut().entry(index).or_insert(
					std::str::from_utf8(
						&self.data[index.index()
							..index.index()
								+ self.data[index.index()..]
									.iter()
									.position(|&c| c == b'\0')?],
					)
					.ok()?,
				),
			)
		}
	}
}
