/// `Format` represent a compression format of a file. Currently BGzip are supported.
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Format {
    BGzip,
    No,
}

pub(crate) fn bytes2type(bytes: [u8; 17]) -> Format {
    match bytes {
        [0x1F, 0x8B, 0x8, 0x4, 0x0, 0x0, 0x0, 0x0, 0x0, 0xFF, 0x6, 0x0, 0x42, 0x43, 0x2, 0x0, 0x0] => {
            Format::BGzip
        }
        _ => Format::No,
    }
}
