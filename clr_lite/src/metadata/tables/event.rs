use crate::metadata::tables::*;

#[derive(Debug)]
pub struct Event {}

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

impl TableRow for Event {
	type Handle = EventHandle;
	const TYPE: TableType = TableType::Event;

	fn read_row(reader: &mut TableReader<'_>) -> Result<Event, TableReaderError> {
		unimplemented!()
	}
}
