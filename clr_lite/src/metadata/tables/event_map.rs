///! ECMA-335 II.22.12
use crate::metadata::tables::*;

#[derive(Debug)]
pub struct EventMap {
	pub parent: TypeDefHandle,
	pub event_list: EventHandle,
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct EventMapHandle(pub(crate) usize);

impl From<EventMapHandle> for usize {
	fn from(h: EventMapHandle) -> usize {
		h.0
	}
}

impl From<usize> for EventMapHandle {
	fn from(x: usize) -> EventMapHandle {
		EventMapHandle(x + 1)
	}
}

impl TableRow for EventMap {
	type Handle = EventMapHandle;
	const TYPE: TableType = TableType::EventMap;

	fn read_row(reader: &mut TableReader<'_>) -> Result<EventMap, TableReaderError> {
		Ok(EventMap {
			parent: reader.read_type_def_handle()?,
			event_list: reader.read_event_handle()?,
		})
	}
}
