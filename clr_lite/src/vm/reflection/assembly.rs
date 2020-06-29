use crate::metadata::tables::TypeDefOrRefHandle;
use crate::metadata::*;
use crate::vm::*;

use std::cell::RefCell;
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

		let a = Assembly(Rc::new(AssemblyInternal {
			clr: Rc::downgrade(&clr.0),
			name,
			types: RefCell::new(vec![]),
		}));

		// Load type names
		let type_count = metadata.tables().type_def.rows().len();
		let mut types = Vec::with_capacity(type_count);
		for i in 0..type_count {
			types.push(Type::load(clr.clone(), a.clone(), i, &metadata)?);
		}

		// Resolve types
		for i in 0..type_count {
			types[i].resolve(clr.clone(), i, &metadata)?;
		}

		// Resolve type interface implementations
		for i in metadata.tables().interface_impl.rows() {
			let td = &metadata.tables().type_def[i.class];
			let name = metadata.strings().get(td.name).unwrap().to_string();
			let namespace = metadata.strings().get(td.namespace).unwrap().to_string();

			let full_name = if namespace.is_empty() {
				name.to_string()
			} else {
				format!("{}.{}", namespace, name)
			};
			let t = clr.get_type(&full_name).unwrap();

			let interface = {
				let (n, ns) = match i.interface {
					TypeDefOrRefHandle::TypeDefHandle(t) => (
						metadata
							.strings()
							.get(metadata.tables().type_def[t].name)
							.unwrap(),
						metadata
							.strings()
							.get(metadata.tables().type_def[t].namespace)
							.unwrap(),
					),
					TypeDefOrRefHandle::TypeRefHandle(t) => (
						metadata
							.strings()
							.get(metadata.tables().type_ref[t].name)
							.unwrap(),
						metadata
							.strings()
							.get(metadata.tables().type_ref[t].namespace)
							.unwrap(),
					),
					_ => unimplemented!("Generics not yet supported"),
				};
				clr.get_type(&if ns.is_empty() {
					n.to_string()
				} else {
					format!("{}.{}", ns, n)
				})
				.unwrap()
			};

			t.0.interfaces.borrow_mut().push(interface);
		}

		*a.0.types.borrow_mut() = types;

		clr.0.borrow_mut().assemblies.push(a.clone());

		Ok(a)
	}

	pub fn name<'a>(&'a self) -> &'a str {
		&self.0.name
	}

	pub fn types<'a>(&'a self) -> Types<'a> {
		Types {
			types: self.0.types.borrow(),
		}
	}
}

pub struct Types<'a> {
	types: Ref<'a, Vec<Type>>,
}

impl<'a> Deref for Types<'a> {
	type Target = [Type];

	fn deref(&self) -> &Self::Target {
		&self.types
	}
}

pub(crate) struct AssemblyInternal {
	clr: Weak<RefCell<ClrInternal>>,
	name: String,
	types: RefCell<Vec<Type>>,
}
