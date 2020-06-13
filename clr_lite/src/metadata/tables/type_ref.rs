/// ECMA-335 II.22.38
#[derive(Debug)]
pub struct TypeRef {
	pub resolution_scope: ResolutionScope,
	pub type_name: StringHandle,
	pub type_namespace: StringHandle,
}

crate::def_table!(
	TypeRef,
	TypeRefHandle,
	fn read_row(reader: &mut TableReader<'_, '_>) -> io::Result<TypeRef> {
		Ok(TypeRef {
			resolution_scope: reader.read_resolution_scope()?,
			type_name: reader.read_string_handle()?,
			type_namespace: reader.read_string_handle()?,
		})
	}
);
