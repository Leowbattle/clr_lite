/// ECMA-335 II.22.30
#[derive(Debug)]
pub struct Module {
	pub name: StringHandle,
	pub mvid: GuidHandle,
}

def_table!(
	Module,
	ModuleHandle,
	fn read_row(reader: &mut TableReader<'_, '_>) -> io::Result<Module> {
		let _generation = reader.reader.read::<u16>()?;
		let name = reader.read_string_handle()?;
		let mvid = reader.read_guid_handle()?;
		let _encid = reader.read_guid_handle()?;
		let _endbaseid = reader.read_guid_handle()?;

		Ok(Module { name, mvid })
	}
);
