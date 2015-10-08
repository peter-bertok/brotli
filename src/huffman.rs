
const HUFFMAN_MAX_CODE_LENGTH: u8 = 15;

/// For current format this constant equals to kNumInsertAndCopyCodes
const HUFFMAN_MAX_CODE_LENGTHS_SIZE: u32 = 704;

/// Maximum possible Huffman table size for an alphabet size of 704, max code length 15 and root table bits 8.
const HUFFMAN_MAX_TABLE_SIZE: usize = 1080;

const HUFFMAN_MAX_CODE_LENGTH_CODE_LENGTH: u8 = 5;

/// An entry in a Huffman table.
#[derive(Debug, Copy, Clone,Default)]
pub struct Code {
    /// Number of bits used for this symbol
    bits: u8,

    /// Symbol value or table offset
    value: u16
}

pub struct Table {
    codes: [Code;HUFFMAN_MAX_TABLE_SIZE],
    alphabet_size: u16
}

impl Default for Table  {
    fn default() -> Self {
        Table { codes: [Code::default();HUFFMAN_MAX_TABLE_SIZE], alphabet_size: 0 }
    }
}

fn get_next_key( key:u32 , len: u32 ) -> u32
{
    let mut step = 1 << (len - 1);
    while ( key & step ) != 0 {
        step >>= 1;
    }
    
    (key & (step - 1)) + step
}