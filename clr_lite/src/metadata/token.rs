use binary_reader::*;

use std::io;

#[derive(Copy, Clone, Debug)]
pub struct Token(pub u32);

impl BinaryReadable for Token {
	fn read(reader: &mut BinaryReader<'_>) -> io::Result<Token> {
		Ok(Token(reader.read::<u32>()?))
	}
}
