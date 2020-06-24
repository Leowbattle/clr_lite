use crate::metadata::*;
use crate::vm::*;

use std::rc::{Rc, Weak};

#[derive(Clone)]
pub struct Assembly(Rc<AssemblyInternal>);

impl Assembly {
	pub(crate) fn load(clr: ClrLite, data: &[u8]) -> Result<Assembly, String> {
		let metadata = Metadata::read(data).map_err(|e| e.to_string())?;
		let name = metadata
			.strings()
			.get(metadata.tables().assembly[0.into()].name)
			.unwrap();

		Ok(Assembly(Rc::new(AssemblyInternal {
			clr: Rc::downgrade(&clr.0),
			name: name.to_string(),
		})))
	}

	pub fn name<'a>(&'a self) -> &'a str {
		&self.0.name
	}
}

pub struct AssemblyInternal {
	clr: Weak<RefCell<ClrInternal>>,
	name: String,
}
