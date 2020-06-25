use crate::metadata::*;
use crate::vm::*;

use std::rc::{Rc, Weak};

#[derive(Clone)]
pub struct Assembly(pub(crate) Rc<AssemblyInternal>);

impl Assembly {
	pub(crate) fn load(mut clr: ClrLite, data: &[u8]) -> Result<Assembly, String> {
		let metadata = Metadata::read(data).map_err(|e| e.to_string())?;
		let name = metadata
			.strings()
			.get(metadata.tables().assembly[0.into()].name)
			.unwrap()
			.to_string();

		// Load all unloaded assemblies referenced by this assembly
		for r in metadata.tables().assembly_ref.rows().iter() {
			let ref_name = metadata
				.strings()
				.get(r.name)
				.ok_or_else(|| format!("{} contains invalid assembly references", name))?;

			// If this reference is not already loaded, load it.
			if !clr.assemblies().iter().any(|a| a.name() == ref_name) {
				clr.load_assembly(ref_name)?;
			}
		}

		// Load type names
		let type_count = metadata.tables().type_def.rows().len();
		let mut types = Vec::with_capacity(type_count);
		for i in 0..type_count {
			types.push(Type::load(clr.clone(), i, &metadata)?);
		}

		// Resolve types
		for i in 0..type_count {
			types[i].resolve(clr.clone(), i, &metadata)?;
		}

		let a = Assembly(Rc::new(AssemblyInternal {
			clr: Rc::downgrade(&clr.0),
			name,
			types,
		}));

		clr.0.borrow_mut().assemblies.push(a.clone());

		Ok(a)
	}

	pub fn name<'a>(&'a self) -> &'a str {
		&self.0.name
	}

	pub fn types<'a>(&'a self) -> &'a [Type] {
		&self.0.types
	}
}

pub(crate) struct AssemblyInternal {
	clr: Weak<RefCell<ClrInternal>>,
	name: String,
	types: Vec<Type>,
}
