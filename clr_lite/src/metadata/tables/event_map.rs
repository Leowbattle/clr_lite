use crate::metadata::*;

#[derive(Debug)]
pub struct EventMap {}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct EventMapHandle(pub(crate) usize);

impl From<EventMapHandle> for usize {
	fn from(h: EventMapHandle) -> usize {
		h.0
	}
}

impl From<usize> for EventMapHandle {
	fn from(x: usize) -> EventMapHandle {
		EventMapHandle(x)
	}
}

impl TableRow for EventMap {
	type Handle = EventMapHandle;
	const TYPE: TableType = TableType::EventMap;

	fn read_row(reader: &mut TableReader<'_>) -> Result<EventMap, TableReaderError> {
		unimplemented!()
	}
}
