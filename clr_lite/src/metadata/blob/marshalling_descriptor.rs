///! ECMA-335 II.23.4
use super::*;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum MarshallingDescriptorScalar {
	Bool,
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
	WString,
	NativeInt,
	NativeUInt,
	FunctionPointer,
	Unknown,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum MarshallingDescriptor {
	Scalar(MarshallingDescriptorScalar),
	Array {
		element_type: MarshallingDescriptorScalar,
		size_param_num: Option<usize>,
		num_elements: Option<usize>,
	},
}

impl BlobReader<'_> {
	pub fn read_marshalling_descriptor_scalar(
		&mut self,
	) -> Result<MarshallingDescriptorScalar, BlobReaderError> {
		let descriptor = self.read::<u8>()?;
		Ok(match descriptor {
			0x2 => MarshallingDescriptorScalar::Bool,
			0x3 => MarshallingDescriptorScalar::SByte,
			0x4 => MarshallingDescriptorScalar::Byte,
			0x5 => MarshallingDescriptorScalar::Short,
			0x6 => MarshallingDescriptorScalar::UShort,
			0x7 => MarshallingDescriptorScalar::Int,
			0x8 => MarshallingDescriptorScalar::UInt,
			0x9 => MarshallingDescriptorScalar::Long,
			0xa => MarshallingDescriptorScalar::ULong,
			0xb => MarshallingDescriptorScalar::Float,
			0xc => MarshallingDescriptorScalar::Double,
			0x14 => MarshallingDescriptorScalar::String,
			0x15 => MarshallingDescriptorScalar::WString,
			0x1f => MarshallingDescriptorScalar::NativeInt,
			0x20 => MarshallingDescriptorScalar::NativeUInt,
			0x26 => MarshallingDescriptorScalar::FunctionPointer,
			0x50 => MarshallingDescriptorScalar::Unknown,
			_ => {
				return Err(BlobReaderError::BadBlob(format!(
					"Invalid marshalling descriptor {:#x}",
					descriptor
				)))
			}
		})
	}

	pub fn read_marshalling_descriptor(
		&mut self,
	) -> Result<MarshallingDescriptor, BlobReaderError> {
		if self.peek::<u8>()? == 0x2a {
			self.reader.advance(1);
			let element_type = self.read_marshalling_descriptor_scalar()?;

			let size_param_num = self.read_compressed_u32();
			let size_param_num = if let Ok(s) = size_param_num {
				if s == 0 {
					None
				} else {
					Some(s as usize)
				}
			} else {
				None
			};

			let num_elements = self.read_compressed_u32();
			let num_elements = if let Ok(n) = num_elements {
				if n == 0 {
					None
				} else {
					Some(n as usize)
				}
			} else {
				None
			};

			Ok(MarshallingDescriptor::Array {
				element_type,
				size_param_num,
				num_elements,
			})
		} else {
			Ok(MarshallingDescriptor::Scalar(
				self.read_marshalling_descriptor_scalar()?,
			))
		}
	}
}
