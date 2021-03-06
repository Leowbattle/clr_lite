use crate::metadata::blob::BlobReader;
use crate::metadata::tables::{MemberRefParentHandle, TableType, TypeDefOrRefHandle};
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

		// Get methods referenced by this assembly.
		let mut method_refs = Vec::with_capacity(metadata.tables().member_ref.rows().len());
		for mr in metadata.tables().member_ref.rows() {
			match mr.parent {
				MemberRefParentHandle::TypeRefHandle(h) => {
					let type_ref = &metadata.tables().type_ref[h];
					let type_name = metadata.strings().get(type_ref.name).unwrap().to_string();
					let type_namespace = metadata
						.strings()
						.get(type_ref.namespace)
						.unwrap()
						.to_string();

					let type_full_name = if type_namespace.is_empty() {
						type_name.to_string()
					} else {
						format!("{}.{}", type_namespace, type_name)
					};

					let t = clr.get_type(&type_full_name).ok_or_else(|| {
						format!("Cannot find type {} referenced by {}", type_full_name, name)
					})?;

					let method_name = metadata
						.strings()
						.get(mr.name)
						.ok_or_else(|| format!("Unable to load methods referenced by {}", name))?;

					let method = t.get_method(method_name).ok_or_else(|| {
						format!(
							"Unable to find {}.{} referenced by {}",
							t, method_name, name
						)
					})?;
					method_refs.push(method);
				}
				_ => continue,
			}
		}

		// Get types references by this assembly
		let mut type_refs = Vec::with_capacity(metadata.tables().type_ref.rows().len());
		for tr in metadata.tables().type_ref.rows() {
			let type_name = metadata.strings().get(tr.name).unwrap().to_string();
			let type_namespace = metadata.strings().get(tr.namespace).unwrap().to_string();

			let type_full_name = if type_namespace.is_empty() {
				type_name.to_string()
			} else {
				format!("{}.{}", type_namespace, type_name)
			};

			let t = clr.get_type(&type_full_name).ok_or_else(|| {
				format!("Cannot find type {} referenced by {}", type_full_name, name)
			})?;
			type_refs.push(t);
		}

		let a = Assembly(Rc::new(AssemblyInternal {
			clr: Rc::downgrade(&clr.0),
			name,
			types: RefCell::new(vec![]),
			entry_point: RefCell::new(None),

			user_string_data: metadata.user_string_data().to_vec().into_boxed_slice(),

			method_defs: RefCell::new(vec![]),
			method_refs: method_refs,
			fields: RefCell::new(vec![]),
			type_refs,
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

		// Get method defs
		let mut method_defs = Vec::with_capacity(metadata.tables().method_def.rows().len());
		for t in a.types().iter() {
			for m in t.methods().iter() {
				method_defs.push(m.clone());
			}
		}
		*a.0.method_defs.borrow_mut() = method_defs;

		// Get entry point if one exists
		if let Some(ep) = metadata.entry_point() {
			*a.0.entry_point.borrow_mut() = Some(
				a.resolve_method(MetadataToken::new(ep.0, TableType::MethodDef))
					.ok_or_else(|| format!("Missing entry point for {}", a.name()))?,
			);
		}

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

	pub fn entry_point(&self) -> Option<Method> {
		self.0.entry_point.borrow().clone()
	}

	pub fn resolve_method(&self, token: MetadataToken) -> Option<Method> {
		const METHOD_DEF: usize = TableType::MethodDef as usize;
		const MEMBER_REF: usize = TableType::MemberRef as usize;
		match token.table() {
			METHOD_DEF => Some(self.0.method_defs.borrow().get(token.index() - 1)?.clone()),
			MEMBER_REF => Some(self.0.method_refs.get(token.index() - 1)?.clone()),
			_ => None,
		}
	}

	pub fn resolve_field(&self, token: MetadataToken) -> Option<Field> {
		if token.table() == TableType::Field as usize {
			Some(self.0.fields.borrow().get(token.index() - 1)?.clone())
		} else {
			None
		}
	}

	pub fn resolve_type(&self, token: MetadataToken) -> Option<Type> {
		const TYPE_DEF: usize = TableType::TypeDef as usize;
		const TYPE_REF: usize = TableType::TypeRef as usize;
		match token.table() {
			TYPE_DEF => Some(self.0.types.borrow().get(token.index() - 1)?.clone()),
			TYPE_REF => Some(self.0.type_refs.get(token.index() - 1)?.clone()),
			_ => None,
		}
	}

	pub fn get_string<'a>(&'a self, token: MetadataToken) -> Option<&'a [u16]> {
		if token.table() == 0x70 {
			BlobReader::new(&self.0.user_string_data[token.index()..])
				.ok()?
				.read_utf16_str()
				.ok()
		} else {
			None
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
	#[allow(dead_code)]
	clr: Weak<RefCell<ClrInternal>>,
	name: String,
	types: RefCell<Vec<Type>>,
	entry_point: RefCell<Option<Method>>,

	user_string_data: Box<[u8]>,

	method_defs: RefCell<Vec<Method>>,
	method_refs: Vec<Method>,
	pub(crate) fields: RefCell<Vec<Field>>,
	type_refs: Vec<Type>,
}
