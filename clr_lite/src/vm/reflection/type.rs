use crate::metadata::blob::ElementType;
use crate::metadata::tables::{TypeDefHandle, TypeDefOrRefHandle, TypeSemantics};
use crate::metadata::*;
use crate::vm::*;

use std::cell::RefCell;
use std::fmt;
use std::rc::{Rc, Weak};

#[derive(Clone)]
pub struct Type(pub(crate) Rc<TypeInternal>);

impl Type {
	pub(crate) fn load<'a>(
		clr: ClrLite,
		i: usize,
		metadata: &'a Metadata<'a>,
	) -> Result<Type, String> {
		let def = &metadata.tables().type_def[i.into()];

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
			kind: RefCell::new(None),
			is_abstract: def.attributes.is_abstract,
			base: RefCell::new(None),
			fields: RefCell::new(vec![]),
		}));

		clr.0.borrow_mut().add_type(t.clone());

		Ok(t)
	}

	/// Types can have circular dependencies, so first we load just the names,
	/// then resolve the rest of the type.
	pub(crate) fn resolve<'a>(
		&mut self,
		clr: ClrLite,
		i: usize,
		metadata: &'a Metadata<'a>,
	) -> Result<(), String> {
		self.resolve_base(clr.clone(), i, metadata)?;
		self.resolve_kind(clr.clone(), i, metadata)?;
		self.resolve_fields(clr.clone(), i, metadata)?;

		Ok(())
	}

	fn resolve_base<'a>(
		&mut self,
		clr: ClrLite,
		i: usize,
		metadata: &'a Metadata<'a>,
	) -> Result<(), String> {
		let def = &metadata.tables().type_def[i.into()];

		// System.Object and <Module> have no base class.
		if let TypeDefOrRefHandle::TypeDefHandle(TypeDefHandle(0)) = def.extends {
			return Ok(());
		}

		let base_name = Type::type_def_or_ref_name(clr.clone(), metadata, def.extends);
		let base = base_name
			.and_then(|n| clr.get_type(&n))
			.ok_or_else(|| format!("Unable to find base type for {}", self.full_name()))?;

		*self.0.base.borrow_mut() = Some(base);

		Ok(())
	}

	fn resolve_kind<'a>(
		&mut self,
		clr: ClrLite,
		i: usize,
		metadata: &'a Metadata<'a>,
	) -> Result<(), String> {
		let def = &metadata.tables().type_def[i.into()];

		let kind = if let TypeSemantics::Interface = def.attributes.semantics {
			TypeKind::Interface
		} else if let None = self.base() {
			TypeKind::Class
		} else {
			match self.base().unwrap().full_name() {
				"System.ValueType" => TypeKind::Struct,
				"System.Array" => TypeKind::Array,
				"System.Enum" => TypeKind::Enum,
				_ => TypeKind::Class,
			}
		};

		*self.0.kind.borrow_mut() = Some(kind);

		Ok(())
	}

	fn resolve_fields<'a>(
		&mut self,
		clr: ClrLite,
		i: usize,
		metadata: &'a Metadata<'a>,
	) -> Result<(), String> {
		let mut fields = self.0.fields.borrow_mut();

		// ECMA-335 II.22.37:
		// The field list is the start of a contiguous run of fields owned by this type.
		// The run continues until the smaller of:
		//     The last row of the field table
		//     The start of the next run of fields owned by the next type

		let def = &metadata.tables().type_def[i.into()];
		let field_count = if i == metadata.tables().type_def.rows().len() - 1 {
			metadata.tables().field.rows().len() - (def.field_list.0 - 1)
		} else {
			metadata.tables().type_def[(i + 1).into()].field_list.0 - def.field_list.0
		};

		fields.reserve(field_count);

		let field_start = def.field_list.0;
		let field_end = field_start + field_count;

		for i in field_start..field_end {
			fields.push(Field::load(clr.clone(), self.clone(), i, metadata)?);
		}

		Ok(())
	}

	pub(crate) fn type_def_or_ref_name<'a>(
		clr: ClrLite,
		metadata: &'a Metadata<'a>,
		def: TypeDefOrRefHandle,
	) -> Option<String> {
		let (name_handle, namespace_handle) = match def {
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

		let name = metadata.strings().get(name_handle)?;
		let namespace = metadata.strings().get(namespace_handle)?;

		Some(if namespace.is_empty() {
			name.to_string()
		} else {
			format!("{}.{}", namespace, name)
		})
	}

	pub(crate) fn get_type_for_element_type<'a>(
		clr: ClrLite,
		metadata: &'a Metadata<'a>,
		e: ElementType,
	) -> Result<Type, String> {
		Ok(match e {
			ElementType::Void => clr.get_type("System.Void").unwrap(),
			ElementType::Bool => clr.get_type("System.Boolean").unwrap(),
			ElementType::Char => clr.get_type("System.Char").unwrap(),
			ElementType::SByte => clr.get_type("System.SByte").unwrap(),
			ElementType::Byte => clr.get_type("System.Byte").unwrap(),
			ElementType::Short => clr.get_type("System.Int16").unwrap(),
			ElementType::UShort => clr.get_type("System.UInt16").unwrap(),
			ElementType::Int => clr.get_type("System.Int32").unwrap(),
			ElementType::UInt => clr.get_type("System.UInt32").unwrap(),
			ElementType::Long => clr.get_type("System.Int64").unwrap(),
			ElementType::ULong => clr.get_type("System.UInt64").unwrap(),
			ElementType::Float => clr.get_type("System.Single").unwrap(),
			ElementType::Double => clr.get_type("System.Double").unwrap(),
			ElementType::String => clr.get_type("System.String").unwrap(),
			ElementType::Pointer(t) => unimplemented!("Pointers not yet supported"),
			ElementType::Reference(t) => unimplemented!("References not yet supported"),
			ElementType::ValueType(t) | ElementType::Class(t) => {
				let name = Type::type_def_or_ref_name(clr.clone(), metadata, t)
					.ok_or_else(|| "Unable to locate type".to_string())?;

				clr.get_type(&name)
					.ok_or_else(|| format!("Unable to locate type {:?}", name))?
			}
			ElementType::TypeGenericParam(i) => unimplemented!("Generics not yet supported"),
			ElementType::Array { .. } => unimplemented!("Non-vector arrays not yet supported"),
			ElementType::Generic { .. } => unimplemented!("Generics not yet supported"),
			ElementType::TypedReference => unimplemented!("Typed references not yet supported"),
			ElementType::IntPtr => clr.get_type("System.IntPtr").unwrap(),
			ElementType::UIntPtr => clr.get_type("System.UIntPtr").unwrap(),
			ElementType::FnPtr => unimplemented!("Function pointers not yet supported"),
			ElementType::Object => clr.get_type("System.Object").unwrap(),
			ElementType::SzArray(et) => Type::get_or_create_array_type(
				clr.clone(),
				Type::get_type_for_element_type(clr, metadata, *et)?,
			),
			ElementType::MethodGenericParam(i) => unimplemented!("Generics not yet supported"),
			_ => return Err(format!("Invalid element type {:?}", e)),
		})
	}

	fn get_or_create_array_type(clr: ClrLite, element: Type) -> Type {
		let full_name = format!("{}[]", element.full_name());
		if let Some(t) = clr.get_type(&full_name) {
			return t;
		}

		let t = Type(Rc::new(TypeInternal {
			clr: Rc::downgrade(&clr.0),
			name: format!("{}[]", element.name()),
			namespace: element.namespace().to_string(),
			full_name,
			kind: RefCell::new(Some(TypeKind::Array)),
			is_abstract: false,
			base: RefCell::new(clr.get_type("System.Array")),
			fields: RefCell::new(vec![]),
		}));

		clr.0.borrow_mut().add_type(t.clone());

		t
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

	pub fn fields(&self) -> impl Iterator<Item = Field> {
		Fields {
			t: self.clone(),
			current: 0,
		}
	}

	pub fn is_abstract(&self) -> bool {
		self.0.is_abstract
	}

	pub fn kind(&self) -> TypeKind {
		self.0.kind.borrow().unwrap()
	}
}

impl fmt::Display for Type {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}", self.full_name())
	}
}

struct Fields {
	t: Type,
	current: usize,
}

impl Iterator for Fields {
	type Item = Field;

	fn next(&mut self) -> Option<Self::Item> {
		let next = self.t.0.fields.borrow().get(self.current)?.clone();
		self.current += 1;
		Some(next)
	}
}

#[derive(Copy, Clone, Debug)]
pub enum TypeKind {
	Class,
	Interface,
	Struct,
	Enum,
	Array,
}

impl fmt::Display for TypeKind {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{:?}", self)
	}
}

// The reason that some fields are wrapped in RefCell and not the whole struct, as
// pub struct Field(Rc<RefCell<FieldInternal>>) is because the name, namespace, etc
// methods return references to the fields of this struct, but you can't return a
// reference inside a RefCell because the reference will outlive the RefCell it was
// borrowed from.
pub(crate) struct TypeInternal {
	clr: Weak<RefCell<ClrInternal>>,
	name: String,
	namespace: String,
	full_name: String,
	kind: RefCell<Option<TypeKind>>,
	is_abstract: bool,
	base: RefCell<Option<Type>>,
	fields: RefCell<Vec<Field>>,
}
