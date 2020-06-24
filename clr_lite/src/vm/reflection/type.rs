use crate::metadata::tables::*;
use crate::metadata::*;
use crate::vm::reflection::Assembly;
use crate::vm::*;

use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Clone)]
pub struct Type(pub(crate) Rc<TypeInternal>);

impl Type {
	pub(crate) fn load<'a>(
		clr: ClrLite,
		def: &TypeDef,
		metadata: &'a Metadata<'a>,
	) -> Result<Type, String> {
		let name = metadata.strings().get(def.name).unwrap().to_string();
		let namespace = metadata.strings().get(def.namespace).unwrap().to_string();

		let full_name = if namespace.is_empty() {
			name.to_string()
		} else {
			format!("{}.{}", namespace, name)
		};

		let t = Type(Rc::new(TypeInternal {
			clr: Rc::downgrade(&clr.0),
			name,
			namespace,
			full_name,
			base: RefCell::new(None),
		}));

		clr.0.borrow_mut().types.push(t.clone());

		Ok(t)
	}

	/// Types can have circular dependencies, so first we load just the names,
	/// then resolve the rest of the type.
	pub(crate) fn resolve<'a>(
		&mut self,
		clr: ClrLite,
		def: &TypeDef,
		metadata: &'a Metadata<'a>,
	) -> Result<(), String> {
		self.resolve_base(clr.clone(), def, metadata)?;
		Ok(())
	}

	fn resolve_base<'a>(
		&mut self,
		clr: ClrLite,
		def: &TypeDef,
		metadata: &'a Metadata<'a>,
	) -> Result<(), String> {
		// System.Object and <Module> have no base class.
		if let TypeDefOrRefHandle::TypeDefHandle(TypeDefHandle(0)) = def.extends {
			return Ok(());
		}

		let (name_handle, namespace_handle) = match def.extends {
			TypeDefOrRefHandle::TypeDefHandle(t) => (
				metadata.tables().type_def[t].name,
				metadata.tables().type_def[t].namespace,
			),
			TypeDefOrRefHandle::TypeRefHandle(t) => (
				metadata.tables().type_ref[t].name,
				metadata.tables().type_ref[t].namespace,
			),
			TypeDefOrRefHandle::TypeSpecHandle(t) => {
				unimplemented!("Inheriting from generic types not yet supported")
			}
		};

		let name = metadata
			.strings()
			.get(name_handle)
			.ok_or_else(|| format!("Could not find base class for {}", self.full_name()))?;
		let namespace = metadata
			.strings()
			.get(namespace_handle)
			.ok_or_else(|| format!("Could not find base class for {}", self.full_name()))?;

		let base_full_name = if namespace.is_empty() {
			name.to_string()
		} else {
			format!("{}.{}", namespace, name)
		};

		let clr_internal = clr.0.borrow();
		let base = clr_internal
			.types
			.iter()
			.find(|t| t.full_name() == base_full_name)
			.ok_or_else(|| format!("Could not find base class for {}", self.full_name()))?
			.clone();

		*self.0.base.borrow_mut() = Some(base);

		Ok(())
	}

	pub fn name<'a>(&'a self) -> &'a str {
		&self.0.name
	}

	pub fn namespace<'a>(&'a self) -> &'a str {
		&self.0.namespace
	}

	pub fn full_name<'a>(&'a self) -> &'a str {
		&self.0.full_name
	}

	pub fn base(&self) -> Option<Type> {
		self.0.base.borrow().clone()
	}
}

pub(crate) struct TypeInternal {
	clr: Weak<RefCell<ClrInternal>>,
	name: String,
	namespace: String,
	full_name: String,
	base: RefCell<Option<Type>>,
}
