use crate::metadata::blob::ElementType;
use crate::metadata::tables::{TypeDefHandle, TypeDefOrRefHandle, TypeSemantics};
use crate::metadata::*;
use crate::vm::*;

use std::cell::{Ref, RefCell};
use std::fmt;
use std::ops::Deref;
use std::rc::{Rc, Weak};

#[derive(Clone)]
pub struct Type(pub(crate) Rc<TypeInternal>);

impl Type {
	pub(crate) fn load<'a>(
		clr: ClrLite,
		assembly: Assembly,
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
			assembly: Rc::downgrade(&assembly.0),
			name,
			namespace,
			full_name,
			kind: RefCell::new(None),
			is_abstract: def.attributes.is_abstract,
			base: RefCell::new(None),
			fields: RefCell::new(vec![]),
			methods: RefCell::new(vec![]),
			interfaces: RefCell::new(vec![]),
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
		self.resolve_methods(clr.clone(), i, metadata)?;

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

	fn resolve_methods<'a>(
		&mut self,
		clr: ClrLite,
		i: usize,
		metadata: &'a Metadata<'a>,
	) -> Result<(), String> {
		let mut methods = self.0.methods.borrow_mut();

		let def = &metadata.tables().type_def[i.into()];
		let method_count = if i == metadata.tables().type_def.rows().len() - 1 {
			metadata.tables().method_def.rows().len() - (def.method_list.0 - 1)
		} else {
			metadata.tables().type_def[(i + 1).into()].method_list.0 - def.method_list.0
		};

		methods.reserve(method_count);

		let method_start = def.method_list.0;
		let method_end = method_start + method_count;

		for i in method_start..method_end {
			methods.push(Method::load(clr.clone(), self.clone(), i, metadata)?);
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
		e: &ElementType,
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
				let name = Type::type_def_or_ref_name(clr.clone(), metadata, *t)
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
				Type::get_type_for_element_type(clr, metadata, et)?,
			),
			ElementType::MethodGenericParam(i) => unimplemented!("Generics not yet supported"),
			_ => return Err(format!("Invalid element type {:?}", e)),
		})
	}

	fn get_or_create_array_type(clr: ClrLite, element: Type) -> Type {
		// TODO Make all reference types use the same array type
		let full_name = format!("{}[]", element.full_name());
		if let Some(t) = clr.get_type(&full_name) {
			return t;
		}

		let t = Type(Rc::new(TypeInternal {
			clr: Rc::downgrade(&clr.0),
			assembly: element.0.assembly.clone(),
			name: format!("{}[]", element.name()),
			namespace: element.namespace().to_string(),
			full_name,
			kind: RefCell::new(Some(TypeKind::Array)),
			is_abstract: false,
			base: RefCell::new(clr.get_type("System.Array")),
			fields: RefCell::new(vec![]),
			methods: RefCell::new(vec![]),
			interfaces: RefCell::new(vec![]),
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

	pub fn assembly(&self) -> Option<Assembly> {
		Some(Assembly(self.0.assembly.upgrade().unwrap()))
	}

	pub fn fields<'a>(&'a self) -> Fields<'a> {
		Fields {
			fields: self.0.fields.borrow(),
		}
	}

	/// Fields from this class and its base classes.
	pub fn all_fields<'a>(&'a self) -> Box<[Field]> {
		let mut fields = self.fields().to_vec();
		if let Some(base) = self.base() {
			fields.extend_from_slice(&base.all_fields());
		}
		fields.into_boxed_slice()
	}

	pub fn methods<'a>(&'a self) -> Methods<'a> {
		Methods {
			methods: self.0.methods.borrow(),
		}
	}

	pub fn all_methods<'a>(&'a self) -> Box<[Method]> {
		let mut methods = self.methods().to_vec();
		if let Some(base) = self.base() {
			methods.extend_from_slice(&base.all_methods());
		}
		for i in self.interfaces().iter() {
			methods.extend_from_slice(&i.all_methods());
		}
		methods.into_boxed_slice()
	}

	pub fn interfaces<'a>(&'a self) -> Interfaces<'a> {
		Interfaces {
			interfaces: self.0.interfaces.borrow(),
		}
	}

	pub fn is_abstract(&self) -> bool {
		self.0.is_abstract
	}

	pub fn kind(&self) -> TypeKind {
		self.0.kind.borrow().unwrap()
	}

	pub fn instance_of(&self, other: Type) -> bool {
		if *self == other {
			true
		} else if let Some(base) = self.base() {
			base.instance_of(other)
		} else if self.interfaces().contains(&other) {
			true
		} else {
			false
		}
	}

	pub fn is_reference_type(&self) -> bool {
		let clr = ClrLite(self.0.clr.upgrade().unwrap());
		!self.instance_of(clr.get_type("System.ValueType").unwrap())
	}

	/// Returns the size in bytes occupied by fields
	pub fn size(&self) -> usize {
		let clr = ClrLite(self.0.clr.upgrade().unwrap());
		if *self == clr.get_type("System.Boolean").unwrap() {
			1
		} else if *self == clr.get_type("System.Char").unwrap() {
			2
		} else if *self == clr.get_type("System.SByte").unwrap() {
			1
		} else if *self == clr.get_type("System.Byte").unwrap() {
			1
		} else if *self == clr.get_type("System.Int16").unwrap() {
			2
		} else if *self == clr.get_type("System.UInt16").unwrap() {
			2
		} else if *self == clr.get_type("System.Int32").unwrap() {
			4
		} else if *self == clr.get_type("System.UInt32").unwrap() {
			4
		} else if *self == clr.get_type("System.Int64").unwrap() {
			8
		} else if *self == clr.get_type("System.UInt64").unwrap() {
			8
		} else if *self == clr.get_type("System.Single").unwrap() {
			4
		} else if *self == clr.get_type("System.Double").unwrap() {
			8
		} else {
			let size = self
				.fields()
				.iter()
				.filter(|f| !f.is_static())
				.fold(0, |a, f| {
					let field_type = f.field_type().unwrap();
					if field_type.is_reference_type() {
						a + 8
					} else {
						a + f.field_type().unwrap().size()
					}
				});
			if let Some(base) = self.base() {
				size + base.size()
			} else {
				size
			}
		}
	}
}

impl PartialEq for Type {
	fn eq(&self, other: &Type) -> bool {
		self.full_name() == other.full_name()
	}
}

impl Eq for Type {}

pub struct Fields<'a> {
	fields: Ref<'a, Vec<Field>>,
}

impl<'a> Deref for Fields<'a> {
	type Target = [Field];

	fn deref(&self) -> &Self::Target {
		&self.fields
	}
}

pub struct Methods<'a> {
	methods: Ref<'a, Vec<Method>>,
}

impl<'a> Deref for Methods<'a> {
	type Target = [Method];

	fn deref(&self) -> &Self::Target {
		&self.methods
	}
}

pub struct Interfaces<'a> {
	interfaces: Ref<'a, Vec<Type>>,
}

impl<'a> Deref for Interfaces<'a> {
	type Target = [Type];

	fn deref(&self) -> &Self::Target {
		&self.interfaces
	}
}

impl fmt::Display for Type {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}", self.full_name())
	}
}

impl fmt::Debug for Type {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}", self)
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
	assembly: Weak<AssemblyInternal>,
	name: String,
	namespace: String,
	full_name: String,
	kind: RefCell<Option<TypeKind>>,
	is_abstract: bool,
	base: RefCell<Option<Type>>,
	fields: RefCell<Vec<Field>>,
	methods: RefCell<Vec<Method>>,
	pub(super) interfaces: RefCell<Vec<Type>>,
}
