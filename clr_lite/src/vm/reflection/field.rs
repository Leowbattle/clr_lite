use crate::metadata::tables::FieldHandle;
use crate::metadata::*;
use crate::vm::*;

use std::cell::RefCell;
use std::fmt;
use std::rc::{Rc, Weak};

#[derive(Clone)]
pub struct Field(pub(crate) Rc<FieldInternal>);

impl Field {
	pub(crate) fn load<'a>(
		clr: ClrLite,
		declaring_type: Type,
		i: usize,
		metadata: &'a Metadata<'a>,
	) -> Result<Field, String> {
		let def = &metadata.tables().field[FieldHandle(i)];

		let name = metadata.strings().get(def.name).unwrap().to_string();

		let signature = metadata
			.blob()
			.new_reader(def.signature)
			.map_err(|e| e.to_string())?
			.read_field_signature()
			.map_err(|e| e.to_string())?;
		let field_type = Type::get_type_for_element_type(clr.clone(), metadata, signature.r#type)?;

		Ok(Field(Rc::new(FieldInternal {
			clr: Rc::downgrade(&clr.0),
			name,
			field_type: Rc::downgrade(&field_type.0),
			declaring_type: Rc::downgrade(&declaring_type.0),
			is_static: def.attributes.is_static,
		})))
	}

	pub fn name<'a>(&'a self) -> &'a str {
		&self.0.name
	}

	pub fn field_type(&self) -> Option<Type> {
		Some(Type(self.0.field_type.upgrade()?))
	}

	pub fn declaring_type(&self) -> Option<Type> {
		Some(Type(self.0.declaring_type.upgrade()?))
	}

	pub fn is_static(&self) -> bool {
		self.0.is_static
	}
}

impl fmt::Display for Field {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{} {}", self.field_type().unwrap().name(), self.name())
	}
}

pub struct FieldInternal {
	clr: Weak<RefCell<ClrInternal>>,
	name: String,
	field_type: Weak<TypeInternal>,
	declaring_type: Weak<TypeInternal>,
	is_static: bool,
}
