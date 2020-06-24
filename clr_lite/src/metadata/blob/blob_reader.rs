///! II.23.2
use binary_reader::*;

use crate::metadata::MetadataToken;

#[derive(Debug)]
pub enum BlobReaderError {
	BadBlob(String),
}

impl ToString for BlobReaderError {
	fn to_string(&self) -> String {
		match self {
			BlobReaderError::BadBlob(s) => s.clone(),
		}
	}
}

pub struct BlobReader<'data> {
	pub(crate) reader: BinaryReader<'data>,
}

impl<'data> BlobReader<'data> {
	pub(crate) fn new(data: &'data [u8]) -> Result<BlobReader, BlobReaderError> {
		let mut br = BlobReader {
			reader: BinaryReader::new(data),
		};
		let length = br
			.read_compressed_u32()
			.map_err(|_| BlobReaderError::BadBlob("Couldn't read blob length".to_string()))?
			as usize;
		br.reader = BinaryReader::new(&data[br.reader.pos()..br.reader.pos() + length]);
		Ok(br)
	}

	pub fn read<T: CopyFromBytes>(&mut self) -> Result<T, BlobReaderError> {
		self.reader
			.read::<T>()
			.ok_or_else(|| BlobReaderError::BadBlob("Unexpected EOF".to_string()))
	}

	pub fn peek<T: CopyFromBytes>(&mut self) -> Result<T, BlobReaderError> {
		self.reader
			.peek::<T>()
			.ok_or_else(|| BlobReaderError::BadBlob("Unexpected EOF".to_string()))
	}

	pub fn read_compressed_u32(&mut self) -> Result<u32, BlobReaderError> {
		let x = self.peek::<u8>()?;
		if x & 0xc0 == 0xc0 {
			Ok(u32::from_be(self.read::<u32>()?) - 0xc000_0000)
		} else if x & 0x80 == 0x80 {
			Ok(u16::from_be(self.read::<u16>()?) as u32 - 0x8000)
		} else {
			Ok(self.read::<u8>()? as u32)
		}
	}

	pub fn read_utf16_str(&mut self) -> Result<&'data [u16], BlobReaderError> {
		self.reader
			.read_array::<u16>(self.reader.data().len() / 2)
			.ok_or_else(|| {
				BlobReaderError::BadBlob("Unable to read UTF16 string from blob".to_string())
			})
	}

	pub fn read_metadata_token(&mut self) -> Result<MetadataToken, BlobReaderError> {
		Ok(MetadataToken(self.read_compressed_u32()?))
	}
}
