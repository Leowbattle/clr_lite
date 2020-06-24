use crate::metadata::tables::FieldHandle;
use crate::metadata::*;
use crate::vm::*;

use std::cell::RefCell;
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
		println!("{} {}", field_type.full_name(), name);

		Ok(Field(Rc::new(FieldInternal {
			clr: Rc::downgrade(&clr.0),
			name,
			field_type: Rc::downgrade(&field_type.0),
			declaring_type: Rc::downgrade(&declaring_type.0),
		})))
	}

	pub fn name<'a>(&'a self) -> &'a str {
		&self.0.name
	}
}

pub struct FieldInternal {
	clr: Weak<RefCell<ClrInternal>>,
	name: String,
	field_type: Weak<TypeInternal>,
	declaring_type: Weak<TypeInternal>,
}
