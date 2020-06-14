#![allow(non_upper_case_globals)]

// ECMA-335 II.22.15
#[derive(Debug, PartialEq, Eq)]
pub struct Field {
	pub flags: FieldAttributes,
	pub name: StringHandle,
	pub signature: BlobHandle,
}

bitflags! {
	pub struct FieldAttributes: u16 {
		const VisibilityMask = 0x7;
		const CompilerControlled = 0x0;
		const Private = 0x1;
		const PrivateProtected = 0x2;
		const Internal = 0x3;
		const Protected = 0x4;
		const ProtectedInternal = 0x5;
		const Public = 0x6;

		const Static = 0x10;
		const InitOnly = 0x20;
		const Literal = 0x40;
		const NotSerialised = 0x80;
		const SpecialName = 0x200;

		const PInvokeImpl = 0x2000;

		const RtSpecialName = 0x400;
		const HasFieldMarshal = 0x1000;
		const HasDefault = 0x8000;
		const HasFieldRva = 0x100;
	}
}

crate::def_table!(
	Field,
	FieldHandle,
	fn read_row(reader: &mut TableReader<'_, '_>) -> io::Result<Field> {
		let flags = FieldAttributes::from_bits(reader.reader.read::<u16>()?)
			.ok_or(io::Error::from(io::ErrorKind::InvalidData))?;
		let name = reader.read_string_handle()?;
		let signature = reader.read_blob_handle()?;

		Ok(Field {
			flags,
			name,
			signature,
		})
	}
);
