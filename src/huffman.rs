
const HUFFMAN_MAX_CODE_LENGTH: u8 = 15;

/// For current format this constant equals to kNumInsertAndCopyCodes
const HUFFMAN_MAX_CODE_LENGTHS_SIZE: u32 = 704;

/// Maximum possible Huffman table size for an alphabet size of 704, max code length 15 and root table bits 8.
const HUFFMAN_MAX_TABLE_SIZE: usize = 1080;

const HUFFMAN_MAX_CODE_LENGTH_CODE_LENGTH: u8 = 5;

/// An entry in a Huffman table.
#[derive(Debug, Copy, Clone,Default)]
struct Code {
    /// Number of bits used for this symbol
    bits: u8,

    /// Symbol value or table offset
    value: u16
}

struct TreeGroup {
    codes: Vec<Code>,
    trees: u16,
    alphabet_size: u16
}

fn get_next_key( key:u32 , len: u32 ) -> u32
{
    let mut step = 1 << (len - 1);
    while ( key & step ) != 0 {
        step >>= 1;
    }
    
    (key & (step - 1)) + step
}

trait HuffmanTable {
    fn read_symbol( &self, bits:u16 ) -> u16;
}

impl <'a> HuffmanTable for &'a [Code] {

    // Decodes the next Huffman code from bit-stream. Reads 0 - 15 bits.
    fn read_symbol( &self, bits:u16 ) -> u16
    {
        return 0;
      /* Read the bits for two reads at once. 
      uint32_t val = BrotliGetBitsUnmasked(br, 15);
      table += val & HUFFMAN_TABLE_MASK;
      if (table->bits > HUFFMAN_TABLE_BITS) {
        int nbits = table->bits - HUFFMAN_TABLE_BITS;
        BrotliDropBits(br, HUFFMAN_TABLE_BITS);
        table += table->value;
        table += (int)(val >> HUFFMAN_TABLE_BITS) & (int)BitMask(nbits);
      }
      BrotliDropBits(br, table->bits);
      return table->value;*/
    }
}

impl TreeGroup {

    pub fn tree( &self, n: u16 ) -> &[Code]
    {
        let offset = (n as usize) * HUFFMAN_MAX_TABLE_SIZE;

        &self.codes[offset .. offset + HUFFMAN_MAX_TABLE_SIZE]
    }

    pub fn tree_mut( &mut self, n: u16 ) -> &mut [Code]
    {
        let offset = (n as usize) * HUFFMAN_MAX_TABLE_SIZE;

        &mut self.codes[offset .. offset + HUFFMAN_MAX_TABLE_SIZE]
    }

    pub fn new( alphabet_size: u16, trees: u16 ) -> TreeGroup
    {
        let n = HUFFMAN_MAX_TABLE_SIZE * ( trees as usize );
        let mut t = TreeGroup {
            codes: Vec::new(),
            trees: trees,
            alphabet_size: alphabet_size
        };

        t.codes.reserve_exact( n );

        t
    }
}
