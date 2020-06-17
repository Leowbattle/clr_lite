use crate::metadata::*;

pub struct Module {
	pub name: StringHandle,
	pub mvid: GuidHandle,
}

impl TableRow for Module {}
