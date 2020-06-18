use crate::metadata::*;

#[derive(Debug)]
pub struct MethodDef {
	pub rva: Rva,
	pub impl_attributes: MethodImplAttributes,
	pub attributes: MethodAttributes,
	pub name: StringHandle,
	pub signature: BlobHandle,
	pub param_list: ParamHandle,
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct MethodDefHandle(pub(crate) usize);

impl From<MethodDefHandle> for usize {
	fn from(h: MethodDefHandle) -> usize {
		h.0
	}
}

impl From<usize> for MethodDefHandle {
	fn from(x: usize) -> MethodDefHandle {
		MethodDefHandle(x + 1)
	}
}

/// ECMA-335 II.23.1.11
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct MethodImplAttributes {
	pub code_type: MethodCodeType,
	pub managed: bool,
	pub forward_ref: bool,
	pub preserve_sig: bool,
	pub internal_call: bool,
	pub synchronised: bool,
	pub no_inlining: bool,
	pub no_optimisation: bool,
}

impl MethodImplAttributes {
	pub fn from_raw(raw: u16) -> Result<MethodImplAttributes, TableReaderError> {
		Ok(MethodImplAttributes {
			code_type: match raw & 0x3 {
				0x0 => MethodCodeType::IL,
				0x1 => MethodCodeType::Native,
				0x2 => MethodCodeType::OPTIL,
				0x3 => MethodCodeType::Runtime,
				_ => {
					return Err(TableReaderError::BadImageFormat(format!(
						"Invalid method code type {}",
						raw & 0x3
					)))
				}
			},
			managed: match raw & 0x4 {
				0x0 => true,
				0x4 => false,
				_ => {
					return Err(TableReaderError::BadImageFormat(format!(
						"Invalid method managed mask {}",
						raw & 0x4
					)))
				}
			},
			forward_ref: raw & 0x10 == 0x10,
			preserve_sig: raw & 0x80 == 0x80,
			internal_call: raw & 0x1000 == 0x1000,
			synchronised: raw & 0x20 == 0x20,
			no_inlining: raw & 0x8 == 0x8,
			no_optimisation: raw & 0x40 == 0x40,
		})
	}
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum MethodCodeType {
	IL,
	Native,
	OPTIL,
	Runtime,
}

/// ECMA-335 II.23.1.10
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct MethodAttributes {
	pub visibility: MethodVisibility,
	pub is_static: bool,
	pub is_final: bool,
	pub is_virtual: bool,
	pub hide_by_sig: bool,
	pub reuse_vtable_slot: bool,
	pub strict: bool,
	pub is_abstract: bool,
	pub special_name: bool,
	pub pinvoke_impl: bool,
	pub unmanaged_export: bool,
	pub rt_special_name: bool,
	pub has_security: bool,
	pub require_sec_object: bool,
}

impl MethodAttributes {
	pub fn from_raw(raw: u16) -> Result<MethodAttributes, TableReaderError> {
		Ok(MethodAttributes {
			visibility: match raw & 0x7 {
				0x0 => MethodVisibility::CompilerControlled,
				0x1 => MethodVisibility::Private,
				0x2 => MethodVisibility::PrivateProtected,
				0x3 => MethodVisibility::Internal,
				0x4 => MethodVisibility::Protected,
				0x5 => MethodVisibility::ProtectedInternal,
				0x6 => MethodVisibility::Public,
				_ => {
					return Err(TableReaderError::BadImageFormat(format!(
						"Invalid method visibility {}",
						raw & 0x7
					)))
				}
			},
			is_static: raw & 0x10 == 0x10,
			is_final: raw & 0x20 == 0x20,
			is_virtual: raw & 0x40 == 0x40,
			hide_by_sig: raw & 0x80 == 0x80,
			reuse_vtable_slot: match raw & 0x100 {
				0x0 => true,
				0x100 => false,
				_ => {
					return Err(TableReaderError::BadImageFormat(format!(
						"Invalid method vtable layout mask {}",
						raw & 0x100
					)))
				}
			},
			strict: raw & 0x200 == 0x200,
			is_abstract: raw & 0x400 == 0x400,
			special_name: raw & 0x800 == 0x800,
			pinvoke_impl: raw & 0x2000 == 0x2000,
			unmanaged_export: raw & 0x8 == 0x8,
			rt_special_name: raw & 0x1000 == 0x1000,
			has_security: raw & 0x4000 == 0x4000,
			require_sec_object: raw & 0x8000 == 0x8000,
		})
	}
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum MethodVisibility {
	CompilerControlled,
	Private,
	PrivateProtected,
	Internal,
	Protected,
	ProtectedInternal,
	Public,
}

impl TableRow for MethodDef {
	type Handle = MethodDefHandle;
	const TYPE: TableType = TableType::MethodDef;

	fn read_row(reader: &mut TableReader<'_>) -> Result<MethodDef, TableReaderError> {
		Ok(MethodDef {
			rva: reader.read_rva()?,
			impl_attributes: MethodImplAttributes::from_raw(reader._read::<u16>()?)?,
			attributes: MethodAttributes::from_raw(reader._read::<u16>()?)?,
			name: reader.read_string_handle()?,
			signature: reader.read_blob_handle()?,
			param_list: reader.read_param_handle()?,
		})
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::metadata::*;

	use std::collections::HashMap;

	#[test]
	fn test_field() {
		let data = include_bytes!(
			"../../../../tests/metadata/tables/MethodDefTests/bin/Debug/netcoreapp3.1/MethodDefTests.dll"
		);
		let metadata = Metadata::read(data).unwrap();

		let methods = metadata
			.tables()
			.method_def
			.rows()
			.iter()
			.map(|m| (metadata.strings().get(m.name).unwrap(), m))
			.collect::<HashMap<&str, &MethodDef>>();

		assert!(methods[".ctor"].attributes.rt_special_name);
		assert_eq!(
			methods["Doit"].attributes.visibility,
			MethodVisibility::Public
		);
		assert!(methods["Static"].attributes.is_static);
		assert!(methods["Virtual"].attributes.is_virtual);
		assert!(methods["Abstract"].attributes.is_abstract);
		assert!(methods["PInvoke"].attributes.pinvoke_impl);
	}
}
