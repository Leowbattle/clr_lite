///! ECMA-335 II.22.13
use crate::metadata::tables::*;

#[derive(Debug)]
pub struct Event {
	pub attributes: EventAttributes,
	pub name: StringHandle,
	pub event_type: TypeDefOrRefHandle,
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct EventHandle(pub(crate) usize);

impl From<EventHandle> for usize {
	fn from(h: EventHandle) -> usize {
		h.0
	}
}

impl From<usize> for EventHandle {
	fn from(x: usize) -> EventHandle {
		EventHandle(x + 1)
	}
}

/// ECMA-335 II.23.1.4
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct EventAttributes {
	pub special_name: bool,
	pub rt_special_name: bool,
}

impl EventAttributes {
	fn from_raw(raw: u16) -> Result<EventAttributes, TableReaderError> {
		Ok(EventAttributes {
			special_name: raw & 0x200 == 0x200,
			rt_special_name: raw & 0x400 == 0x400,
		})
	}
}

impl TableRow for Event {
	type Handle = EventHandle;
	const TYPE: TableType = TableType::Event;

	fn read_row(reader: &mut TableReader<'_>) -> Result<Event, TableReaderError> {
		Ok(Event {
			attributes: EventAttributes::from_raw(reader._read::<u16>()?)?,
			name: reader.read_string_handle()?,
			event_type: reader.read_type_def_or_ref_handle()?,
		})
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::metadata::tables::*;

	use std::collections::HashMap;

	#[test]
	fn test_event() {
		let data = include_bytes!(
			"../../../../tests/metadata/tables/EventTests/bin/Debug/netcoreapp3.1/EventTests.dll"
		);
		let metadata = Metadata::read(data).unwrap();

		let events = metadata
			.tables()
			.event_map
			.rows()
			.iter()
			.enumerate()
			.map(|(i, e)| {
				(
					metadata
						.strings()
						.get(metadata.tables().type_def[e.parent].name)
						.unwrap(),
					{
						let event_end = if i == metadata.tables().event_map.rows().len() - 1 {
							metadata.tables().event.rows().len()
						} else {
							usize::from(metadata.tables().event_map.rows()[i + 1].event_list) - 1
						};
						(usize::from(e.event_list) - 1..event_end)
							.map(|e2| {
								metadata
									.strings()
									.get(metadata.tables().event[e2.into()].name)
									.unwrap()
							})
							.collect::<Box<[&str]>>()
					},
				)
			})
			.collect::<HashMap<&str, Box<[&str]>>>();

		assert_eq!(events["Class1"].as_ref(), &["Hello", "Hello2"]);
		assert_eq!(events["Class2"].as_ref(), &["Hi3", "Hi4"]);
		assert_eq!(events["Class3"].as_ref(), &["AAA"]);
	}
}
