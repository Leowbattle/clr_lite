use super::tables::*;

pub trait TableRow {
	type Handle: Into<usize>;
}

pub struct Table<T: TableRow> {
	rows: Box<[T]>,
}

#[macro_use]
mod macros {
	#[macro_export]
	macro_rules! def_table {
		($row:ty, $handle_name:ident) => {
			def_handle!($handle_name);

			impl crate::metadata::tables_stream::TableRow for $row {
				type Handle = $handle_name;
			}
		};
	}
}

impl<T: TableRow> std::ops::Index<T::Handle> for Table<T> {
	type Output = T;

	fn index(&self, handle: T::Handle) -> &Self::Output {
		&self.rows[handle.into()]
	}
}

/// ECMA-335 II.24.2.6
pub struct TablesStream<'data> {
	data: &'data [u8],

	pub module: Option<Table<Module>>,
	pub type_ref: Option<Table<TypeRef>>,
	pub type_def: Option<Table<TypeDef>>,
	pub field: Option<Table<Field>>,
	pub method_def: Option<Table<MethodDef>>,
	pub param: Option<Table<Param>>,
	pub interface_impl: Option<Table<InterfaceImpl>>,
	pub member_ref: Option<Table<MemberRef>>,
	pub constant: Option<Table<Constant>>,
	pub custom_attribute: Option<Table<CustomAttribute>>,
	pub field_marshal: Option<Table<FieldMarshal>>,
	pub decl_securitie: Option<Table<DeclSecurity>>,
	pub class_layout: Option<Table<ClassLayout>>,
	pub field_layout: Option<Table<FieldLayout>>,
	pub standalone_sig: Option<Table<StandaloneSig>>,
	pub event_map: Option<Table<EventMap>>,
	pub event: Option<Table<Event>>,
	pub property_map: Option<Table<PropertyMap>>,
	pub property: Option<Table<Property>>,
	pub method_semantic: Option<Table<MethodSemantics>>,
	pub method_impl: Option<Table<MethodImpl>>,
	pub module_ref: Option<Table<ModuleRef>>,
	pub type_spec: Option<Table<TypeSpec>>,
	pub impl_map: Option<Table<ImplMap>>,
	pub field_rva: Option<Table<FieldRva>>,
	pub assembly: Option<Table<Assembly>>,
	pub assembly_os: Option<Table<AssemblyOS>>,
	pub assembly_ref: Option<Table<AssemblyRef>>,
	pub assembly_ref_processor: Option<Table<AssemblyRefProcessor>>,
	pub assembly_ref_os: Option<Table<AssemblRefOS>>,
	pub file: Option<Table<File>>,
	pub exported_type: Option<Table<ExportedType>>,
	pub manifest_resource: Option<Table<ManifestResource>>,
	pub nested_class: Option<Table<NestedClass>>,
	pub generic_param: Option<Table<GenericParam>>,
	pub method_spec: Option<Table<MethodSpec>>,
	pub generic_param_constraint: Option<Table<GenericParamConstraint>>,
}
