use crate::metadata;

use std::io;

use super::*;

/// ECMA-335 II.25.1.16
#[derive(Debug)]
pub enum ElementType {
	Void,
	Bool,
	Char,
	SByte,
	Byte,
	Short,
	UShort,
	Int,
	UInt,
	Long,
	ULong,
	Float,
	Double,
	String,
	Pointer(Box<ElementType>),
	ByRef(Box<ElementType>),
	ValueType(metadata::Token),
	Class(metadata::Token),

	/// The type is a generic parameter with the specified index.
	/// ```cs
	/// class G<T, U>
	/// {
	/// 	T t; // Element type is Generic(0)
	/// 	U u; // Element type is Generic(1)
	/// }
	/// ```
	FieldGenericParameter(u32),

	Array {
		element_type: Box<ElementType>,
		shape: ArrayShape,
	},

	/// The type is a specialisation of a generic type.
	/// ```cs
	/// // Element type is GenericInstantiation {
	/// // 		r#type: Class(metadata token for System.Collections.Generic.List)
	/// // 		generic_args: [Int]
	/// // }
	/// List<int> l;
	/// ```
	GenericInstantiation {
		r#type: Box<ElementType>,
		generic_args: Box<[ElementType]>,
	},

	TypedByRef,
	IntPtr,
	UIntPtr,
	FnPtr, // TODO
	Object,
	SzArray,
	MethodGenericParameter(u32),
	RequiredModifier(metadata::Token),
	OptionalModifier(metadata::Token),
	Internal,
	VarargsSentinel,
	Pinned(Box<ElementType>),
	Type, // System.Type

	AttributeBoxed(Box<ElementType>),
	AttributeField(Box<ElementType>),
	AttributeProperty(Box<ElementType>),
	AttributeEnum(Box<ElementType>),
}

pub trait ReadElementType {
	fn read_element_type(&mut self) -> io::Result<ElementType>;
}

impl ReadElementType for BlobReader<'_> {
	fn read_element_type(&mut self) -> io::Result<ElementType> {
		match self.reader.read::<u8>()? {
			0x1 => Ok(ElementType::Void),
			0x2 => Ok(ElementType::Bool),
			0x3 => Ok(ElementType::Char),
			0x4 => Ok(ElementType::SByte),
			0x5 => Ok(ElementType::Byte),
			0x6 => Ok(ElementType::Short),
			0x7 => Ok(ElementType::UShort),
			0x8 => Ok(ElementType::Int),
			0x9 => Ok(ElementType::UInt),
			0xa => Ok(ElementType::Long),
			0xb => Ok(ElementType::ULong),
			0xc => Ok(ElementType::Float),
			0xd => Ok(ElementType::Double),
			0xe => Ok(ElementType::String),
			0xf => Ok(ElementType::Pointer(Box::new(self.read_element_type()?))),
			0x10 => Ok(ElementType::ByRef(Box::new(self.read_element_type()?))),
			0x11 => Ok(ElementType::ValueType(self.read_metadata_token()?)),
			0x12 => Ok(ElementType::Class(self.read_metadata_token()?)),
			0x13 => Ok(ElementType::FieldGenericParameter(
				self.read_compressed_u32()?,
			)),
			0x14 => Ok(ElementType::Array {
				element_type: Box::new(self.read_element_type()?),
				shape: self.read_array_shape()?,
			}),
			0x15 => Ok(ElementType::GenericInstantiation {
				r#type: Box::new(self.read_element_type()?),
				generic_args: {
					let count = self.read_compressed_u32()? as usize;
					let mut args = Vec::with_capacity(count);
					for _ in 0..count {
						args.push(self.read_element_type()?);
					}
					args.into_boxed_slice()
				},
			}),
			0x16 => Ok(ElementType::TypedByRef),
			0x18 => Ok(ElementType::IntPtr),
			0x19 => Ok(ElementType::UIntPtr),
			0x1b => unimplemented!(), // TODO FnPtr
			0x1c => Ok(ElementType::Object),
			0x1d => Ok(ElementType::SzArray),
			0x1e => Ok(ElementType::MethodGenericParameter(
				self.read_compressed_u32()?,
			)),
			0x1f => Ok(ElementType::RequiredModifier(self.read_metadata_token()?)),
			0x20 => Ok(ElementType::OptionalModifier(self.read_metadata_token()?)),
			0x21 => Ok(ElementType::Internal),
			0x41 => Ok(ElementType::VarargsSentinel),
			0x45 => Ok(ElementType::Pinned(Box::new(self.read_element_type()?))),
			0x50 => Ok(ElementType::Type),
			0x51 => Ok(ElementType::AttributeBoxed(Box::new(
				self.read_element_type()?,
			))),
			0x53 => Ok(ElementType::AttributeField(Box::new(
				self.read_element_type()?,
			))),
			0x54 => Ok(ElementType::AttributeProperty(Box::new(
				self.read_element_type()?,
			))),
			0x55 => Ok(ElementType::AttributeEnum(Box::new(
				self.read_element_type()?,
			))),
			_ => Err(io::Error::new(
				io::ErrorKind::InvalidData,
				"Invalid element type",
			)),
		}
	}
}
