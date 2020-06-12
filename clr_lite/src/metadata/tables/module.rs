use super::super::{GuidHandle, StringHandle};

pub struct Module {
	name: StringHandle,
	mvid: GuidHandle,
}

crate::def_table!(Module, ModuleHandle);
