#![allow(non_upper_case_globals)]

pub mod table_type;
pub use table_type::*;

pub mod coded_index;
pub use coded_index::*;

pub mod module;
pub use module::*;

pub mod type_ref;
pub use type_ref::*;

pub mod type_def;
pub use type_def::*;

pub mod field;
pub use field::*;

pub mod method_def;
pub use method_def::*;

pub mod param;
pub use param::*;

pub mod interface_impl;
pub use interface_impl::*;

pub mod member_ref;
pub use member_ref::*;

pub mod constant;
pub use constant::*;

pub mod custom_attribute;
pub use custom_attribute::*;

pub mod field_marshal;
pub use field_marshal::*;

pub mod decl_security;
pub use decl_security::*;

pub mod class_layout;
pub use class_layout::*;

pub mod field_layout;
pub use field_layout::*;

pub mod standalone_sig;
pub use standalone_sig::*;

pub mod event_map;
pub use event_map::*;

pub mod event;
pub use event::*;

pub mod property_map;
pub use property_map::*;

pub mod property;
pub use property::*;

pub mod method_semantics;
pub use method_semantics::*;

pub mod method_impl;
pub use method_impl::*;

pub mod module_ref;
pub use module_ref::*;

pub mod type_spec;
pub use type_spec::*;

pub mod impl_map;
pub use impl_map::*;

pub mod field_rva;
pub use field_rva::*;

pub mod assembly;
pub use assembly::*;

pub mod assembly_processor;
pub use assembly_processor::*;

pub mod assembly_os;
pub use assembly_os::*;

pub mod assembly_ref;
pub use assembly_ref::*;

pub mod assembly_ref_processor;
pub use assembly_ref_processor::*;

pub mod assembly_ref_os;
pub use assembly_ref_os::*;

pub mod file;
pub use file::*;

pub mod exported_type;
pub use exported_type::*;

pub mod manifest_resource;
pub use manifest_resource::*;

pub mod nested_class;
pub use nested_class::*;

pub mod generic_param;
pub use generic_param::*;

pub mod method_spec;
pub use method_spec::*;

pub mod generic_param_constraint;
pub use generic_param_constraint::*;

use crate::metadata::*;

#[derive(Debug)]
pub struct Table<T: TableRow>(Box<[T]>);

impl<T: TableRow> Table<T> {
	pub fn rows(&self) -> &[T] {
		&self.0
	}
}

impl<T: TableRow> std::ops::Index<T::Handle> for Table<T> {
	type Output = T;

	fn index(&self, h: T::Handle) -> &Self::Output {
		&self.0[h.into() - 1]
	}
}

pub struct Tables {
	pub module: Table<Module>,
	pub type_ref: Table<TypeRef>,
	pub type_def: Table<TypeDef>,
	pub field: Table<Field>,
	pub method_def: Table<MethodDef>,
	pub param: Table<Param>,
	pub interface_impl: Table<InterfaceImpl>,
	pub member_ref: Table<MemberRef>,
	pub constant: Table<Constant>,
	pub custom_attribute: Table<CustomAttribute>,
	pub field_marshal: Table<FieldMarshal>,
	pub decl_security: Table<DeclSecurity>,
}

pub trait TableRow: Sized + std::fmt::Debug {
	type Handle: Into<usize> + Copy;
	const TYPE: TableType;

	fn read_row(reader: &mut TableReader<'_>) -> Result<Self, TableReaderError>;
}

use binary_reader::*;

#[derive(Debug)]
pub enum TableReaderError {
	BadImageFormat(String),
}

impl ToString for TableReaderError {
	fn to_string(&self) -> String {
		match &self {
			TableReaderError::BadImageFormat(s) => s.clone(),
		}
	}
}

pub struct TableReader<'data> {
	reader: BinaryReader<'data>,
	row_counts: [usize; 64],

	// The following variables say if an index is encoded with 2 or 4 bytes
	wide_table_handles: [bool; 64],

	wide_string: bool,
	wide_guid: bool,
	wide_blob: bool,

	// Coded indices
	wide_type_def_or_ref: bool,
	wide_has_constant: bool,
	wide_has_custom_attribute: bool,
	wide_has_field_marshal: bool,
	wide_has_decl_security: bool,
	wide_member_ref_parent: bool,
	wide_custom_attribute_type: bool,
	wide_resolution_scope: bool,
}

bitflags! {
	struct HeapSizeFlags : u8 {
		const String = 1;
		const Guid = 2;
		const Blob = 4;
	}
}

impl<'data> TableReader<'data> {
	pub(crate) fn read(data: &'data [u8]) -> Result<Tables, TableReaderError> {
		// Read tables header
		// ECMA II.24.2.6

		let mut br = BinaryReader::new(data);

		// Skip unused data
		br.advance(6);

		let heap_sizes = br
			.read::<u8>()
			.and_then(|bits| HeapSizeFlags::from_bits(bits))
			.ok_or_else(|| {
				TableReaderError::BadImageFormat("Invalid metadata header".to_string())
			})?;

		// Skip unused data
		br.advance(1);

		let present_tables = br.read::<u64>().ok_or_else(|| {
			TableReaderError::BadImageFormat("Invalid metadata header".to_string())
		})?;
		let _sorted_tables = br.read::<u64>().ok_or_else(|| {
			TableReaderError::BadImageFormat("Invalid metadata header".to_string())
		})?;

		let row_counts_raw = br
			.read_array::<u32>(present_tables.count_ones() as usize)
			.ok_or_else(|| {
				TableReaderError::BadImageFormat("Invalid metadata header".to_string())
			})?;

		let mut used = 0;
		let mut row_counts = [0; 64];
		for i in 0..64 {
			if present_tables & (1 << i) != 0 {
				row_counts[i] = row_counts_raw[used] as usize;
				used += 1;
			}
		}

		let mut wide_table_handles = [false; 64];
		for i in 0..64 {
			wide_table_handles[i] = row_counts[i] > 65535;
		}

		TableReader {
			reader: br,
			row_counts,

			wide_table_handles,
			wide_string: heap_sizes.contains(HeapSizeFlags::String),
			wide_guid: heap_sizes.contains(HeapSizeFlags::Guid),
			wide_blob: heap_sizes.contains(HeapSizeFlags::Blob),

			wide_type_def_or_ref: is_coded_index_wide(
				TypeDefOrRefHandle::LARGE_ROW_SIZE,
				TypeDefOrRefHandle::TABLES,
				&row_counts,
			),
			wide_has_constant: is_coded_index_wide(
				HasConstantHandle::LARGE_ROW_SIZE,
				HasConstantHandle::TABLES,
				&row_counts,
			),
			wide_has_custom_attribute: is_coded_index_wide(
				HasCustomAttributeHandle::LARGE_ROW_SIZE,
				HasCustomAttributeHandle::TABLES,
				&row_counts,
			),
			wide_has_field_marshal: is_coded_index_wide(
				HasFieldMarshalHandle::LARGE_ROW_SIZE,
				HasFieldMarshalHandle::TABLES,
				&row_counts,
			),
			wide_has_decl_security: is_coded_index_wide(
				HasDeclSecurityHandle::LARGE_ROW_SIZE,
				HasDeclSecurityHandle::TABLES,
				&row_counts,
			),
			wide_member_ref_parent: is_coded_index_wide(
				MemberRefParentHandle::LARGE_ROW_SIZE,
				MemberRefParentHandle::TABLES,
				&row_counts,
			),
			wide_custom_attribute_type: is_coded_index_wide(
				CustomAttributeTypeHandle::LARGE_ROW_SIZE,
				CustomAttributeTypeHandle::TABLES,
				&row_counts,
			),
			wide_resolution_scope: is_coded_index_wide(
				ResolutionScopeHandle::LARGE_ROW_SIZE,
				ResolutionScopeHandle::TABLES,
				&row_counts,
			),
		}
		.read_tables()
	}

	fn read_table<T: TableRow>(&mut self) -> Result<Table<T>, TableReaderError> {
		let count = self.row_counts[T::TYPE as usize];
		let mut table = Vec::with_capacity(count);

		for _ in 0..count {
			table.push(T::read_row(self)?);
		}
		Ok(Table::<T>(table.into_boxed_slice()))
	}

	fn read_tables(mut self) -> Result<Tables, TableReaderError> {
		Ok(Tables {
			module: self.read_table::<Module>()?,
			type_ref: self.read_table::<TypeRef>()?,
			type_def: self.read_table::<TypeDef>()?,
			field: self.read_table::<Field>()?,
			method_def: self.read_table::<MethodDef>()?,
			param: self.read_table::<Param>()?,
			interface_impl: self.read_table::<InterfaceImpl>()?,
			member_ref: self.read_table::<MemberRef>()?,
			constant: self.read_table::<Constant>()?,
			custom_attribute: self.read_table::<CustomAttribute>()?,
			field_marshal: self.read_table::<FieldMarshal>()?,
			decl_security: self.read_table::<DeclSecurity>()?,
		})
	}

	fn _read<T: CopyFromBytes>(&mut self) -> Result<T, TableReaderError> {
		self.reader
			.read::<T>()
			.ok_or_else(|| TableReaderError::BadImageFormat("Unexpected EOF".to_string()))
	}

	pub fn read_rva(&mut self) -> Result<Rva, TableReaderError> {
		self._read::<Rva>()
	}

	pub fn read_type_def_handle(&mut self) -> Result<TypeDefHandle, TableReaderError> {
		match self.wide_table_handles[TableType::TypeDef as usize] {
			true => Ok(TypeDefHandle(self._read::<u32>()? as usize)),
			false => Ok(TypeDefHandle(self._read::<u16>()? as usize)),
		}
	}

	pub fn read_field_handle(&mut self) -> Result<FieldHandle, TableReaderError> {
		match self.wide_table_handles[TableType::Field as usize] {
			true => Ok(FieldHandle(self._read::<u32>()? as usize)),
			false => Ok(FieldHandle(self._read::<u16>()? as usize)),
		}
	}

	pub fn read_method_def_handle(&mut self) -> Result<MethodDefHandle, TableReaderError> {
		match self.wide_table_handles[TableType::MethodDef as usize] {
			true => Ok(MethodDefHandle(self._read::<u32>()? as usize)),
			false => Ok(MethodDefHandle(self._read::<u16>()? as usize)),
		}
	}

	pub fn read_param_handle(&mut self) -> Result<ParamHandle, TableReaderError> {
		match self.wide_table_handles[TableType::Param as usize] {
			true => Ok(ParamHandle(self._read::<u32>()? as usize)),
			false => Ok(ParamHandle(self._read::<u16>()? as usize)),
		}
	}

	pub fn read_string_handle(&mut self) -> Result<StringHandle, TableReaderError> {
		match self.wide_string {
			true => Ok(StringHandle(self._read::<u32>()? as usize)),
			false => Ok(StringHandle(self._read::<u16>()? as usize)),
		}
	}

	pub fn read_guid_handle(&mut self) -> Result<GuidHandle, TableReaderError> {
		match self.wide_guid {
			true => Ok(GuidHandle(self._read::<u32>()? as usize)),
			false => Ok(GuidHandle(self._read::<u16>()? as usize)),
		}
	}

	pub fn read_blob_handle(&mut self) -> Result<BlobHandle, TableReaderError> {
		match self.wide_blob {
			true => Ok(BlobHandle(self._read::<u32>()? as usize)),
			false => Ok(BlobHandle(self._read::<u16>()? as usize)),
		}
	}

	pub fn read_type_def_or_ref_handle(&mut self) -> Result<TypeDefOrRefHandle, TableReaderError> {
		let data = match self.wide_type_def_or_ref {
			true => self._read::<u32>()? as usize,
			false => self._read::<u16>()? as usize,
		};

		let tag = data & TypeDefOrRefHandle::TAG_MASK;
		let index =
			(data & !TypeDefOrRefHandle::TAG_MASK) >> (TypeDefOrRefHandle::TAG_MASK.count_ones());

		Ok(match tag {
			0 => TypeDefOrRefHandle::TypeDefHandle(TypeDefHandle(index)),
			1 => TypeDefOrRefHandle::TypeRefHandle(TypeRefHandle(index)),
			2 => TypeDefOrRefHandle::TypeSpecHandle(TypeSpecHandle(index)),
			_ => {
				return Err(TableReaderError::BadImageFormat(format!(
					"Invalid TypeDefOrRef tag {}",
					tag
				)))
			}
		})
	}

	pub fn read_has_constant_handle(&mut self) -> Result<HasConstantHandle, TableReaderError> {
		let data = match self.wide_has_constant {
			true => self._read::<u32>()? as usize,
			false => self._read::<u16>()? as usize,
		};
		let tag = data & HasConstantHandle::TAG_MASK;
		let index =
			(data & !HasConstantHandle::TAG_MASK) >> (HasConstantHandle::TAG_MASK.count_ones());
		Ok(match tag {
			0 => HasConstantHandle::FieldHandle(FieldHandle(index)),
			1 => HasConstantHandle::ParamHandle(ParamHandle(index)),
			2 => HasConstantHandle::PropertyHandle(PropertyHandle(index)),
			_ => {
				return Err(TableReaderError::BadImageFormat(format!(
					"Invalid HasConstant tag {}",
					tag
				)))
			}
		})
	}

	pub fn read_has_custom_attribute_handle(
		&mut self,
	) -> Result<HasCustomAttributeHandle, TableReaderError> {
		let data = match self.wide_has_custom_attribute {
			true => self._read::<u32>()? as usize,
			false => self._read::<u16>()? as usize,
		};

		let tag = data & HasCustomAttributeHandle::TAG_MASK;
		let index = (data & !HasCustomAttributeHandle::TAG_MASK)
			>> (HasCustomAttributeHandle::TAG_MASK.count_ones());

		Ok(match tag {
			0 => HasCustomAttributeHandle::MethodDefHandle(MethodDefHandle(index)),
			1 => HasCustomAttributeHandle::FieldHandle(FieldHandle(index)),
			2 => HasCustomAttributeHandle::TypeRefHandle(TypeRefHandle(index)),
			3 => HasCustomAttributeHandle::TypeDefHandle(TypeDefHandle(index)),
			4 => HasCustomAttributeHandle::ParamHandle(ParamHandle(index)),
			5 => HasCustomAttributeHandle::InterfaceImplHandle(InterfaceImplHandle(index)),
			6 => HasCustomAttributeHandle::MemberRefHandle(MemberRefHandle(index)),
			7 => HasCustomAttributeHandle::ModuleHandle(ModuleHandle(index)),
			9 => HasCustomAttributeHandle::PropertyHandle(PropertyHandle(index)),
			10 => HasCustomAttributeHandle::EventHandle(EventHandle(index)),
			11 => HasCustomAttributeHandle::StandaloneSigHandle(StandaloneSigHandle(index)),
			12 => HasCustomAttributeHandle::ModuleRefHandle(ModuleRefHandle(index)),
			13 => HasCustomAttributeHandle::TypeSpecHandle(TypeSpecHandle(index)),
			14 => HasCustomAttributeHandle::AssemblyHandle(AssemblyHandle(index)),
			15 => HasCustomAttributeHandle::AssemblyRefHandle(AssemblyRefHandle(index)),
			16 => HasCustomAttributeHandle::FileHandle(FileHandle(index)),
			17 => HasCustomAttributeHandle::ExportedTypeHandle(ExportedTypeHandle(index)),
			18 => HasCustomAttributeHandle::ManifestResourceHandle(ManifestResourceHandle(index)),
			19 => HasCustomAttributeHandle::GenericParamHandle(GenericParamHandle(index)),
			20 => HasCustomAttributeHandle::GenericParamConstraintHandle(
				GenericParamConstraintHandle(index),
			),
			21 => HasCustomAttributeHandle::MethodSpecHandle(MethodSpecHandle(index)),
			_ => {
				return Err(TableReaderError::BadImageFormat(format!(
					"Invalid HasCustomAttribute tag {}",
					tag
				)))
			}
		})
	}

	pub fn read_has_field_marshal_handle(
		&mut self,
	) -> Result<HasFieldMarshalHandle, TableReaderError> {
		let data = match self.wide_has_field_marshal {
			true => self._read::<u32>()? as usize,
			false => self._read::<u16>()? as usize,
		};

		let tag = data & HasFieldMarshalHandle::TAG_MASK;
		let index = (data & !HasFieldMarshalHandle::TAG_MASK)
			>> (HasFieldMarshalHandle::TAG_MASK.count_ones());

		Ok(match tag {
			0 => HasFieldMarshalHandle::FieldHandle(FieldHandle(index)),
			1 => HasFieldMarshalHandle::ParamHandle(ParamHandle(index)),
			_ => {
				return Err(TableReaderError::BadImageFormat(format!(
					"Invalid TypeDefOrRef tag {}",
					tag
				)))
			}
		})
	}

	pub fn read_has_decl_security_handle(
		&mut self,
	) -> Result<HasDeclSecurityHandle, TableReaderError> {
		let data = match self.wide_has_decl_security {
			true => self._read::<u32>()? as usize,
			false => self._read::<u16>()? as usize,
		};

		let tag = data & HasDeclSecurityHandle::TAG_MASK;
		let index = (data & !HasDeclSecurityHandle::TAG_MASK)
			>> (HasDeclSecurityHandle::TAG_MASK.count_ones());

		Ok(match tag {
			0 => HasDeclSecurityHandle::TypeDefHandle(TypeDefHandle(index)),
			1 => HasDeclSecurityHandle::MethodDefHandle(MethodDefHandle(index)),
			2 => HasDeclSecurityHandle::AssemblyHandle(AssemblyHandle(index)),
			_ => {
				return Err(TableReaderError::BadImageFormat(format!(
					"Invalid HasDeclSecurity tag {}",
					tag
				)))
			}
		})
	}

	pub fn read_member_ref_parent_handle(
		&mut self,
	) -> Result<MemberRefParentHandle, TableReaderError> {
		let data = match self.wide_member_ref_parent {
			true => self._read::<u32>()? as usize,
			false => self._read::<u16>()? as usize,
		};

		let tag = data & MemberRefParentHandle::TAG_MASK;
		let index = (data & !MemberRefParentHandle::TAG_MASK)
			>> (MemberRefParentHandle::TAG_MASK.count_ones());

		Ok(match tag {
			0 => MemberRefParentHandle::TypeDefHandle(TypeDefHandle(index)),
			1 => MemberRefParentHandle::TypeRefHandle(TypeRefHandle(index)),
			2 => MemberRefParentHandle::ModuleRefHandle(ModuleRefHandle(index)),
			3 => MemberRefParentHandle::MethodDefHandle(MethodDefHandle(index)),
			4 => MemberRefParentHandle::TypeSpecHandle(TypeSpecHandle(index)),
			_ => {
				return Err(TableReaderError::BadImageFormat(format!(
					"Invalid MemberRefParent tag {}",
					tag
				)))
			}
		})
	}

	pub fn read_custom_attribute_type_handle(
		&mut self,
	) -> Result<CustomAttributeTypeHandle, TableReaderError> {
		let data = match self.wide_custom_attribute_type {
			true => self._read::<u32>()? as usize,
			false => self._read::<u16>()? as usize,
		};

		let tag = data & CustomAttributeTypeHandle::TAG_MASK;
		let index = (data & !CustomAttributeTypeHandle::TAG_MASK)
			>> (CustomAttributeTypeHandle::TAG_MASK.count_ones());

		Ok(match tag {
			2 => CustomAttributeTypeHandle::MethodDefHandle(MethodDefHandle(index)),
			3 => CustomAttributeTypeHandle::MemberRefHandle(MemberRefHandle(index)),
			_ => {
				return Err(TableReaderError::BadImageFormat(format!(
					"Invalid CustomAttributeType tag {}",
					tag
				)))
			}
		})
	}

	pub fn read_resolution_scope(&mut self) -> Result<ResolutionScopeHandle, TableReaderError> {
		let data = match self.wide_resolution_scope {
			true => self._read::<u32>()? as usize,
			false => self._read::<u16>()? as usize,
		};

		let tag = data & ResolutionScopeHandle::TAG_MASK;
		let index = (data & !ResolutionScopeHandle::TAG_MASK)
			>> (ResolutionScopeHandle::TAG_MASK.count_ones());

		Ok(match tag {
			0 => ResolutionScopeHandle::ModuleHandle(ModuleHandle(index)),
			1 => ResolutionScopeHandle::ModuleRefHandle(ModuleRefHandle(index)),
			2 => ResolutionScopeHandle::AssemblyRefHandle(AssemblyRefHandle(index)),
			3 => ResolutionScopeHandle::TypeRefHandle(TypeRefHandle(index)),
			_ => {
				return Err(TableReaderError::BadImageFormat(format!(
					"Invalid ResolutionScope tag {}",
					tag
				)))
			}
		})
	}
}

fn is_coded_index_wide(large_row_size: usize, tables: &[TableType], row_counts: &[usize]) -> bool {
	tables
		.iter()
		.any(|&t| row_counts[t as usize] > large_row_size)
}
