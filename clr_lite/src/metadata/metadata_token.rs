#[derive(Copy, Clone, Debug)]
pub struct MetadataToken(u32);

unsafe impl binary_reader::CopyFromBytes for MetadataToken {}
