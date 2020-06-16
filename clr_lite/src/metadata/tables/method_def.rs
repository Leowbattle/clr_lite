#![allow(non_upper_case_globals)]

use crate::pe::Rva;

/// ECMA-335 II.22.26
#[derive(Debug)]
pub struct MethodDef {
	pub rva: Rva,
	pub impl_flags: MethodImplAttributes,
	pub flags: MethodAttributes,
	pub name: StringHandle,
	pub signature: BlobHandle,
	pub params: ParamHandle,
}

bitflags! {
	/// Note: The fmt::Debug implementation is wrong, but there is no way to override it
	pub struct MethodImplAttributes : u16 {
		const CodeTypeMask = 0x3;
		const IL = 0x1;
		const Native = 0x1;
		const Optil = 0x2;
		const Runtime = 0x3;

		const ManagedMask = 0x4;
		const Managed = 0x0;
		const Unmanaged = 0x4;

		const ForwardRef = 0x10;
		const PreserveSig = 0x80;
		const InternalCall = 0x1000;
		const Synchronised = 0x20;
		const NoInlining = 0x8;
		const MaxMethodImplVal = 0xffff;
		const NoOptimisation = 0x40;
	}
}

bitflags! {
	/// Note: The fmt::Debug implementation is wrong, but there is no way to override it
	pub struct MethodAttributes : u16 {
		const VisibilityMask = 0x7;
		const CompilerControlled = 0x0;
		const Private = 0x1;
		const PrivateProtected = 0x2;
		const Internal = 0x3;
		const Protected = 0x4;
		const ProtectedInternal = 0x5;
		const Public = 0x6;

		const Static = 0x10;
		const Final = 0x20;
		const Virtual = 0x40;
		const HideBySig = 0x80;

		const VTableLayoutMask = 0x100;
		const ReuseSlot = 0x0;
		const NewSlot = 0x100;

		const Strict = 0x200;
		const Abstract = 0x400;
		const SpecialName = 0x800;

		const PInvokeImpl = 0x2000;
		const UnmanagedExport = 0x8;

		const RtSpecialName = 0x1000;
		const HasSecurity = 0x4000;
		const RequireSecObject = 0x8000;
	}
}

crate::def_table!(
	MethodDef,
	MethodDefHandle,
	fn read_row(reader: &mut TableReader<'_, '_>) -> io::Result<MethodDef> {
		let rva = reader.reader.read::<Rva>()?;
		let impl_flags = MethodImplAttributes::from_bits(reader.reader.read::<u16>()?)
			.ok_or_else(|| io::Error::from(io::ErrorKind::InvalidData))?;
		let flags = MethodAttributes::from_bits(reader.reader.read::<u16>()?)
			.ok_or_else(|| io::Error::from(io::ErrorKind::InvalidData))?;
		let name = reader.read_string_handle()?;
		let signature = reader.read_blob_handle()?;
		let params = reader.read_param_handle()?;

		Ok(MethodDef {
			rva,
			impl_flags,
			flags,
			name,
			signature,
			params,
		})
	}
);
