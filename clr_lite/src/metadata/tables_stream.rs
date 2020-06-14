#![allow(non_upper_case_globals)]

use super::coded_index::*;
use super::tables::*;
use super::{BlobHandle, GuidHandle, StringHandle};

use binary_reader::*;

use std::io;

pub struct TableReader<'data, 'header> {
	pub reader: BinaryReader<'data>,
	pub header: &'header TablesHeader,

	wide_handles: HashMap<TableType, bool>,

	wide_string_handle: bool,
	wide_guid_handle: bool,
	wide_blob_handle: bool,
}

macro_rules! read_handle {
	($name:ident, $type:ty) => {
		pub fn $name(&mut self) -> io::Result<<$type as TableRow>::Handle> {
			// ECMA-335 II.22: "Each index is either 2 or 4 bytes wide.
			// The index points into the same or another table, or into one of the four heaps.
			// The size of each index column in a table is only made 4 bytes if it needs to be for that particular module.
			// So, if a particular column indexes a table, or tables, whose highest row number fits in a 2-byte value, the indexer column need only be 2 bytes wide.
			// Conversely, for tables containing 64K or more rows, an indexer of that table will be 4 bytes wide."

			match self.wide_handles.get(&<$type>::TYPE) {
				Some(true) => Ok((self.reader.read::<u32>()? as usize).into()),
				_ => Ok((self.reader.read::<u16>()? as usize).into()),
			}
		}
	};
}

macro_rules! read_coded_index {
	($name:ident, $type:ty, $tag_bitmask:expr, $($tag_value:expr => $tag_type:ident),*) => {
		pub fn $name(&mut self) -> io::Result<$type> {
			let data = self.reader.read::<u16>()?;
			let tag = data & $tag_bitmask;
			let index = ((data & !$tag_bitmask) >> ($tag_bitmask as u16).count_ones()) as usize;

			match tag {
				$($tag_value => Ok(<$type>::$tag_type(index.into()))),*,
				_ => Err(io::Error::from(io::ErrorKind::InvalidData)),
			}
		}
	};
}

impl<'data, 'header> TableReader<'data, 'header> {
	pub(crate) fn new(reader: BinaryReader<'data>, header: &'header TablesHeader) -> Self {
		let wide_handles = header
			.tables
			.iter()
			.map(|(&k, &v)| (k, v > 65535))
			.collect();

		if header.tables.iter().any(|(&_k, &v)| v > 65535) {
			panic!("Please implement 4 byte table indices");
		}

		Self {
			reader,
			header,

			wide_handles,

			wide_string_handle: header.heap_sizes.contains(HeapSizeFlags::StringBit),
			wide_guid_handle: header.heap_sizes.contains(HeapSizeFlags::GuidBit),
			wide_blob_handle: header.heap_sizes.contains(HeapSizeFlags::BlobBit),
		}
	}

	// pub fn read_handle(&mut self, table: TableType) -> io::Result<usize> {
	// 	// ECMA-335 II.22: "Each index is either 2 or 4 bytes wide.
	// 	// The index points into the same or another table, or into one of the four heaps.
	// 	// The size of each index column in a table is only made 4 bytes if it needs to be for that particular module.
	// 	// So, if a particular column indexes a table, or tables, whose highest row number fits in a 2-byte value, the indexer column need only be 2 bytes wide.
	// 	// Conversely, for tables containing 64K or more rows, an indexer of that table will be 4 bytes wide."

	// 	let size = self
	// 		.header
	// 		.tables
	// 		.get(&table)
	// 		.map(|&count| match count {
	// 			c if c > 65535 => 4,
	// 			_ => 2,
	// 		})
	// 		.ok_or(io::Error::from(io::ErrorKind::InvalidData))?;

	// 	match size {
	// 		4 => Ok(self.reader.read::<u32>()? as usize),
	// 		2 => Ok(self.reader.read::<u16>()? as usize),
	// 		_ => unreachable!(),
	// 	}
	// }

	read_handle!(read_field_handle, Field);
	read_handle!(read_method_def_handle, MethodDef);

	pub fn read_string_handle(&mut self) -> io::Result<StringHandle> {
		if self.wide_string_handle {
			Ok((self.reader.read::<u32>()? as usize).into())
		} else {
			Ok((self.reader.read::<u16>()? as usize).into())
		}
	}

	pub fn read_guid_handle(&mut self) -> io::Result<GuidHandle> {
		if self.wide_guid_handle {
			Ok((self.reader.read::<u32>()? as usize).into())
		} else {
			Ok((self.reader.read::<u16>()? as usize).into())
		}
	}

	pub fn read_blob_handle(&mut self) -> io::Result<BlobHandle> {
		if self.wide_blob_handle {
			Ok((self.reader.read::<u32>()? as usize).into())
		} else {
			Ok((self.reader.read::<u16>()? as usize).into())
		}
	}

	read_coded_index! {
		read_type_def_or_ref,
		TypeDefOrRef,
		0b11,
		0 => TypeDefHandle,
		1 => TypeRefHandle,
		2 => TypeSpecHandle
	}

	read_coded_index! {
		read_has_constant,
		HasConstant,
		0b11,
		0 => FieldHandle,
		1 => ParamHandle,
		2 => PropertyHandle
	}

	read_coded_index! {
		read_has_custom_attribute,
		HasCustomAttribute,
		0b11111,
		0 => MethodDefHandle,
		1 => FieldHandle,
		2 => TypeRefHandle,
		3 => TypeDefHandle,
		4 => ParamHandle,
		5 => InterfaceImplHandle,
		6 => MemberRefHandle,
		7 => ModuleHandle,
		// 8 => PermissionHandle,
		9 => PropertyHandle,
		10 => EventHandle,
		11 => StandaloneSigHandle,
		12 => ModuleRefHandle,
		13 => TypeSpecHandle,
		14 => AssemblyHandle,
		15 => AssemblyRefHandle,
		16 => FileHandle,
		17 => ExportedTypeHandle,
		18 => ManifestResourceHandle,
		19 => GenericParamHandle,
		20 => GenericParamConstraintHandle,
		21 => MethodSpecHandle
	}

	read_coded_index! {
		read_has_field_marshall,
		HasFieldMarshall,
		0b1,
		0 => FieldHandle,
		1 => ParamHandle
	}

	read_coded_index! {
		read_has_decl_security,
		HasDeclSecurity,
		0b11,
		0 => TypeDefHandle,
		1 => MethodDefHandle,
		2 => AssemblyHandle
	}

	read_coded_index! {
		read_member_ref_parent,
		MemberRefParent,
		0b111,
		0 => TypeDefHandle,
		1 => TypeRefHandle,
		2 => ModuleRefHandle,
		3 => MethodDefHandle,
		4 => TypeSpecHandle
	}

	read_coded_index! {
		read_has_semantics,
		HasSemantics,
		0b1,
		0 => EventHandle,
		1 => PropertyHandle
	}

	read_coded_index! {
		read_method_def_or_ref,
		MethodDefOrRef,
		0b1,
		0 => MethodDefHandle,
		1 => MemberRefHandle
	}

	read_coded_index! {
		read_member_forwarded,
		MemberForwarded,
		0b1,
		0 => FieldHandle,
		1 => MethodDefHandle
	}

	read_coded_index! {
		read_implementation,
		Implementation,
		0b11,
		0 => FileHandle,
		1 => AssemblyRefHandle,
		2 => ExportedTypeHandle
	}

	read_coded_index! {
		read_custom_attribute_type,
		CustomAttributeType,
		0b1,
		0 => MethodDefHandle,
		1 => MemberRefHandle
	}

	read_coded_index! {
		read_resolution_scope,
		ResolutionScope,
		0b11,
		0 => ModuleHandle,
		1 => ModuleRefHandle,
		2 => AssemblyRefHandle,
		3 => TypeRefHandle
	}

	read_coded_index! {
		read_type_or_method_def,
		TypeOrMethodDef,
		0b1,
		0 => TypeDefHandle,
		1 => MethodDefHandle
	}
}

pub trait TableRow: Sized + std::fmt::Debug {
	type Handle: Into<usize> + Copy;
	const TYPE: TableType;

	fn read_row(_reader: &mut TableReader<'_, '_>) -> io::Result<Self> {
		unimplemented!()
	}
}

#[derive(Debug)]
pub struct Table<T: TableRow> {
	rows: Box<[T]>,
}

#[macro_use]
mod macros {
	#[macro_export]
	macro_rules! def_table {
		($row:ident, $handle_name:ident) => {
			def_handle!($handle_name);

			impl crate::metadata::tables_stream::TableRow for $row {
				type Handle = $handle_name;
				const TYPE: crate::metadata::TableType = crate::metadata::TableType::$row;
			}
		};

		($row:ident, $handle_name:ident,$read:item) => {
			use crate::metadata::*;

			#[allow(unused_imports)]
			use crate::metadata::tables::*;

			def_handle!($handle_name);

			impl crate::metadata::tables_stream::TableRow for $row {
				type Handle = $handle_name;
				const TYPE: crate::metadata::TableType = crate::metadata::TableType::$row;

				$read
			}
		};
	}
}

impl<T: TableRow> Table<T> {
	pub fn rows(&self) -> &Box<[T]> {
		&self.rows
	}

	pub fn get(&self, index: T::Handle) -> Option<&T> {
		match index.into() {
			0 => None,
			i => self.rows.get(i - 1),
		}
	}
}

impl<T: TableRow> std::ops::Index<T::Handle> for Table<T> {
	type Output = T;

	fn index(&self, handle: T::Handle) -> &Self::Output {
		let index = handle.into();
		if index == 0 {
			panic!("Null index");
		}
		&self.rows[index - 1]
	}
}

/// ECMA-335 II.24.2.6
#[derive(Debug)]
pub struct TablesStream {
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
	pub decl_security: Option<Table<DeclSecurity>>,
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
	pub assembly_ref_os: Option<Table<AssemblyRefOS>>,
	pub file: Option<Table<File>>,
	pub exported_type: Option<Table<ExportedType>>,
	pub manifest_resource: Option<Table<ManifestResource>>,
	pub nested_class: Option<Table<NestedClass>>,
	pub generic_param: Option<Table<GenericParam>>,
	pub method_spec: Option<Table<MethodSpec>>,
	pub generic_param_constraint: Option<Table<GenericParamConstraint>>,
}

use super::TableType;

use std::collections::HashMap;

#[derive(Debug)]
pub struct TablesHeader {
	pub major_version: u8,
	pub minor_version: u8,
	pub heap_sizes: HeapSizeFlags,
	pub tables: HashMap<TableType, u32>,
	pub sorted: u64,
}

impl BinaryReadable for TablesHeader {
	fn read(reader: &mut BinaryReader<'_>) -> io::Result<Self> {
		let _reserved = reader.read::<u32>()?;
		let major_version = reader.read::<u8>()?;
		let minor_version = reader.read::<u8>()?;
		let heap_sizes = HeapSizeFlags::from_bits(reader.read::<u8>()?)
			.ok_or(io::Error::from(io::Error::from(io::ErrorKind::InvalidData)))?;

		let _reserved2 = reader.read::<u8>()?;

		let present_tables = reader.read::<u64>()?;

		let sorted = reader.read::<u64>()?;

		let mut tables = HashMap::new();
		for t in TableType::values() {
			if 1 & (present_tables >> t as u64) == 1 {
				let row_count = reader.read::<u32>()?;
				tables.insert(t, row_count);
			}
		}

		Ok(TablesHeader {
			major_version,
			minor_version,
			heap_sizes,
			tables,
			sorted,
		})
	}
}

pub struct RawTable<'data> {
	pub data: &'data [u8],
	pub row_count: usize,
}

bitflags! {
	pub struct HeapSizeFlags: u8 {
		const StringBit = 1;
		const GuidBit = 2;
		const BlobBit = 4;
	}
}

impl<'data> TablesStream {
	pub(crate) fn new(data: &'data [u8]) -> Option<TablesStream> {
		let mut reader = BinaryReader::new(data);

		let header = reader.read::<TablesHeader>().ok()?;
		let mut table_reader = TableReader::new(reader, &header);

		macro_rules! try_get_table {
			($name:ident) => {
				header.tables.get(&TableType::$name).and_then(|&count| {
					let mut table = Vec::with_capacity(count as usize);
					for _ in 0..count {
						table.push($name::read_row(&mut table_reader).ok()?);
						}
					Some(Table {
						rows: table.into_boxed_slice(),
						})
					})
			};
		}

		Some(TablesStream {
			module: try_get_table!(Module),
			type_ref: try_get_table!(TypeRef),
			type_def: try_get_table!(TypeDef),
			field: try_get_table!(Field),
			method_def: None,
			param: None,
			interface_impl: None,
			member_ref: None,
			constant: None,
			custom_attribute: None,
			field_marshal: None,
			decl_security: None,
			class_layout: None,
			field_layout: None,
			standalone_sig: None,
			event_map: None,
			event: None,
			property_map: None,
			property: None,
			method_semantic: None,
			method_impl: None,
			module_ref: None,
			type_spec: None,
			impl_map: None,
			field_rva: None,
			assembly: None,
			assembly_os: None,
			assembly_ref: None,
			assembly_ref_processor: None,
			assembly_ref_os: None,
			file: None,
			exported_type: None,
			manifest_resource: None,
			nested_class: None,
			generic_param: None,
			method_spec: None,
			generic_param_constraint: None,
		})
	}
}
