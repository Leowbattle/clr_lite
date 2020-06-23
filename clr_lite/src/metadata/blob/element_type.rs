///! ECMA-335 II.23.1.16
use super::*;
use crate::metadata::tables::TypeDefOrRefHandle;

#[derive(Debug, PartialEq, Eq)]
pub enum ElementType {
	Void,
	Bool,
	Char,
	SByte,
	Byte,
	Int,
	UInt,
	Short,
	UShort,
	Long,
	ULong,
	Float,
	Double,
	String,
	Pointer(Box<ElementType>),
	Reference(Box<ElementType>),
	ValueType(TypeDefOrRefHandle),
	Class(TypeDefOrRefHandle),
	TypeGenericParam(usize),
	Array {
		r#type: Box<ElementType>,
		shape: ArrayShape,
	},
	Generic {
		r#type: Box<ElementType>,
		args: Box<[ElementType]>,
	},
	TypedReference,
	IntPtr,
	UIntPtr,
	FnPtr, // TODO
	Object,

	/// One-dimensional array
	SzArray(Box<ElementType>),
	MethodGenericParam(usize),
	VarargSentinel,
}

impl BlobReader<'_> {
	pub fn read_element_type(&mut self) -> Result<ElementType, BlobReaderError> {
		let b = self.read::<u8>()?;
		Ok(match b {
			0x1 => ElementType::Void,
			0x2 => ElementType::Bool,
			0x3 => ElementType::Char,
			0x4 => ElementType::SByte,
			0x5 => ElementType::Byte,
			0x6 => ElementType::Short,
			0x7 => ElementType::UShort,
			0x8 => ElementType::Int,
			0x9 => ElementType::UInt,
			0xa => ElementType::Long,
			0xb => ElementType::ULong,
			0xc => ElementType::Float,
			0xd => ElementType::Double,
			0xe => ElementType::String,
			0xf => ElementType::Pointer(Box::new(self.read_element_type()?)),
			0x10 => ElementType::Reference(Box::new(self.read_element_type()?)),
			0x11 => ElementType::ValueType(self.read_type_def_or_ref()?),
			0x12 => ElementType::Class(self.read_type_def_or_ref()?),
			0x13 => ElementType::TypeGenericParam(self.read_compressed_u32()? as usize),
			0x14 => ElementType::Array {
				r#type: Box::new(self.read_element_type()?),
				shape: self.read_array_shape()?,
			},
			0x15 => ElementType::Generic {
				r#type: Box::new(self.read_element_type()?),
				args: {
					let count = self.read_compressed_u32()? as usize;
					let mut args = Vec::with_capacity(count);
					for _ in 0..count {
						args.push(self.read_element_type()?);
					}
					args.into_boxed_slice()
				},
			},
			0x16 => ElementType::TypedReference,
			0x18 => ElementType::IntPtr,
			0x19 => ElementType::UIntPtr,
			0x1c => ElementType::Object,
			0x1d => ElementType::SzArray(Box::new(self.read_element_type()?)),
			0x1e => ElementType::MethodGenericParam(self.read_compressed_u32()? as usize),
			0x41 => ElementType::VarargSentinel,
			_ => {
				return Err(BlobReaderError::BadBlob(format!(
					"Invalid ELEMENT_TYPE {:#x}",
					b
				)))
			}
		})
	}
}
