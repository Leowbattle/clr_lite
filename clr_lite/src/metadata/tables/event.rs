use crate::metadata::*;

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
		EventHandle(x)
	}
}

impl TableRow for Event {
	type Handle = EventHandle;

	fn read_row(reader: &mut TableReader<'_>) -> Result<Event, TableReaderError> {
		unimplemented!()
	}
}
