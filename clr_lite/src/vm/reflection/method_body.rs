///! ECMA-335 II.25.4
use crate::metadata::tables::{
	MethodDef, StandaloneSigHandle, TableType, TypeDefHandle, TypeRefHandle,
};
use crate::metadata::*;
use crate::vm::reflection::*;
use crate::vm::*;

use binary_reader::*;

use std::rc::Weak;

pub struct MethodBody {
	code: Box<[u8]>,
	max_stack: usize,
	local_variables: Box<[Type]>,
	init_locals: bool,
	exception_handling_clauses: Box<[ExceptionClause]>,
}

impl MethodBody {
	pub(crate) fn load<'a>(
		clr: ClrLite,
		metadata: &'a Metadata<'a>,
		def: &MethodDef,
	) -> Result<MethodBody, String> {
		let data_offset = metadata
			.pe_info()
			.resolve_rva(def.rva)
			.map_err(|_| "Cannot find method body")?;

		let mut br = BinaryReader::new(&metadata.pe_data()[data_offset..]);
		let b = br
			.read::<u8>()
			.ok_or_else(|| "Invalid method body".to_string())?;

		match b & 0x3 {
			// Small format
			0x2 => Ok(MethodBody {
				code: br
					.read_array::<u8>(((b & 0xfc) >> 2) as usize)
					.ok_or_else(|| "Invalid method body".to_string())?
					.to_vec()
					.into_boxed_slice(),
				max_stack: 8,
				local_variables: vec![].into_boxed_slice(),
				init_locals: b & 0x10 == 0x10,
				exception_handling_clauses: vec![].into_boxed_slice(),
			}),
			// Fat format
			0x3 => {
				br.advance(1);
				let max_stack = br
					.read::<u16>()
					.ok_or_else(|| "Invalid method body".to_string())? as usize;
				let code_size = br
					.read::<u32>()
					.ok_or_else(|| "Invalid method body".to_string())? as usize;

				// Get local variable info
				let locals = {
					let locals_token = br
						.read::<MetadataToken>()
						.ok_or_else(|| "Invalid method body".to_string())?;
					if locals_token.index() == 0 {
						vec![]
					} else {
						let mut locals_reader = metadata
							.blob()
							.new_reader(
								metadata.tables().standalone_sig
									[StandaloneSigHandle(locals_token.index())]
								.signature,
							)
							.map_err(|_| "Invalid method body".to_string())?;
						if locals_reader
							.read::<u8>()
							.map_err(|_| "Invalid method body".to_string())?
							!= 0x7
						{
							return Err("Invalid method body".to_string());
						}
						let locals_count = locals_reader
							.read_compressed_u32()
							.map_err(|_| "Invalid method body".to_string())?
							as usize;
						let mut locals = Vec::with_capacity(locals_count);
						for i in 0..locals_count {
							locals.push(Type::get_type_for_element_type(
								clr.clone(),
								metadata,
								&locals_reader
									.read_element_type()
									.map_err(|_| "Invalid method body".to_string())?,
							)?);
						}

						locals
					}
				};

				let code = br
					.read_array::<u8>(code_size)
					.ok_or_else(|| "Invalid method body".to_string())?
					.to_vec()
					.into_boxed_slice();

				let mut clauses = vec![];
				// There are exception handlers
				if b & 0x8 == 0x8 {
					while br.pos() % 4 != 0 {
						br.advance(1);
					}
					let next = br
						.read::<u8>()
						.ok_or_else(|| "Invalid method body".to_string())?;

					if next & 0x1 != 0x1 {
						return Err("Non exception extra section in method body".to_string());
					}

					let mut data_size = br
						.read::<u8>()
						.ok_or_else(|| "Invalid method body".to_string())? as usize;
					// Fat format
					if next & 0x40 == 0x40 {
						data_size += (br
							.read::<u16>()
							.ok_or_else(|| "Invalid method body".to_string())? as usize)
							<< 8;

						let count = data_size / 24;

						clauses.reserve(count);
						for _ in 0..count {
							let clause_code = br
								.read::<u32>()
								.ok_or_else(|| "Invalid method body".to_string())?;
							let clause_type = match clause_code {
								0x0 => ExceptionClauseType::Catch,
								0x1 => ExceptionClauseType::Filter,
								0x2 => ExceptionClauseType::Finally,
								0x4 => ExceptionClauseType::Fault,
								_ => {
									return Err(format!(
										"Invalid exception clause code {}",
										clause_code
									))
								}
							};

							let try_offset =
								br.read::<u32>()
									.ok_or_else(|| "Invalid method body".to_string())? as usize;
							let try_length =
								br.read::<u32>()
									.ok_or_else(|| "Invalid method body".to_string())? as usize;
							let handler_offset =
								br.read::<u32>()
									.ok_or_else(|| "Invalid method body".to_string())? as usize;
							let handler_length =
								br.read::<u32>()
									.ok_or_else(|| "Invalid method body".to_string())? as usize;
							let token_or_filter = br
								.read::<u32>()
								.ok_or_else(|| "Invalid method body".to_string())?;

							clauses.push(ExceptionClause::new(
								clr.clone(),
								metadata,
								clause_type,
								try_offset,
								try_length,
								handler_offset,
								handler_length,
								token_or_filter,
							)?);
						}
					}
					// Small format
					else {
						let count = data_size / 12;
						br.advance(2);

						clauses.reserve(count);
						for _ in 0..count {
							let clause_code = br
								.read::<u16>()
								.ok_or_else(|| "Invalid method body".to_string())?;
							let clause_type = match clause_code {
								0x0 => ExceptionClauseType::Catch,
								0x1 => ExceptionClauseType::Filter,
								0x2 => ExceptionClauseType::Finally,
								0x4 => ExceptionClauseType::Fault,
								_ => {
									return Err(format!(
										"Invalid exception clause code {}",
										clause_code
									))
								}
							};

							let try_offset =
								br.read::<u16>()
									.ok_or_else(|| "Invalid method body".to_string())? as usize;
							let try_length =
								br.read::<u8>()
									.ok_or_else(|| "Invalid method body".to_string())? as usize;
							let handler_offset =
								br.read::<u16>()
									.ok_or_else(|| "Invalid method body".to_string())? as usize;
							let handler_length =
								br.read::<u8>()
									.ok_or_else(|| "Invalid method body".to_string())? as usize;
							let token_or_filter = br
								.read::<u32>()
								.ok_or_else(|| "Invalid method body".to_string())?;

							clauses.push(ExceptionClause::new(
								clr.clone(),
								metadata,
								clause_type,
								try_offset,
								try_length,
								handler_offset,
								handler_length,
								token_or_filter,
							)?);
						}
					}
				}

				Ok(MethodBody {
					code,
					max_stack,
					local_variables: locals.into_boxed_slice(),
					init_locals: b & 0x10 == 0x10,
					exception_handling_clauses: clauses.into_boxed_slice(),
				})
			}
			_ => return Err("Invalid method body".to_string()),
		}
	}

	pub fn code<'a>(&'a self) -> &'a [u8] {
		&self.code
	}

	pub fn max_stack(&self) -> usize {
		self.max_stack()
	}

	pub fn local_variables<'a>(&'a self) -> &'a [Type] {
		&self.local_variables()
	}

	/// Should local variables be initialised to 0?
	pub fn init_locals(&self) -> bool {
		self.init_locals()
	}

	pub fn exception_handling_clauses<'a>(&'a self) -> &'a [ExceptionClause] {
		&self.exception_handling_clauses
	}
}

pub struct ExceptionClause {
	pub clause_type: ExceptionClauseType,
	pub try_offset: usize,
	pub try_length: usize,
	pub handler_offset: usize,
	pub handler_length: usize,
	catch_type: Option<Weak<TypeInternal>>,
	pub filter_offset: Option<usize>,
}

impl ExceptionClause {
	pub fn catch_type(&self) -> Option<Type> {
		match &self.catch_type {
			Some(c) => Some(Type(c.upgrade()?)),
			None => None,
		}
	}

	fn new<'a>(
		clr: ClrLite,
		metadata: &'a Metadata<'a>,
		clause_type: ExceptionClauseType,
		try_offset: usize,
		try_length: usize,
		handler_offset: usize,
		handler_length: usize,
		token_or_filter: u32,
	) -> Result<ExceptionClause, String> {
		Ok(ExceptionClause {
			clause_type,
			try_offset,
			try_length,
			handler_offset,
			handler_length,
			catch_type: if clause_type == ExceptionClauseType::Catch {
				let token = MetadataToken(token_or_filter);

				const TYPE_DEF: usize = TableType::TypeDef as usize;
				const TYPE_REF: usize = TableType::TypeRef as usize;
				let (name, namespace) = match token.table() {
					TYPE_DEF => (
						metadata.tables().type_def[TypeDefHandle(token.index())].name,
						metadata.tables().type_def[TypeDefHandle(token.index())].namespace,
					),
					TYPE_REF => (
						metadata.tables().type_ref[TypeRefHandle(token.index())].name,
						metadata.tables().type_ref[TypeRefHandle(token.index())].namespace,
					),
					_ => return Err(format!("Invalid metadata token for catch type {:?}", token)),
				};
				let name = metadata
					.strings()
					.get(name)
					.ok_or_else(|| "Cannot find exception handler type".to_string())?;
				let namespace = metadata
					.strings()
					.get(namespace)
					.ok_or_else(|| "Cannot find exception handler type".to_string())?;
				let full_name = if namespace.is_empty() {
					name.to_string()
				} else {
					format!("{}.{}", namespace, name)
				};
				Some(Rc::downgrade(
					&clr.get_type(&full_name)
						.ok_or_else(|| "Cannot find exception handler type".to_string())?
						.0,
				))
			} else {
				None
			},
			filter_offset: if clause_type == ExceptionClauseType::Filter {
				Some(token_or_filter as usize)
			} else {
				None
			},
		})
	}
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ExceptionClauseType {
	Catch,
	Filter,
	Finally,
	Fault,
}
