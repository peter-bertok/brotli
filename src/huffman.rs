use bitreader::BitReader;
use std::ops::Index;
use BrotliError;
use Result;

/// Maximum number of bits used for a code
const HUFFMAN_MAX_CODE_LENGTH: usize = 15;

/// For current format this constant equals to kNumInsertAndCopyCodes
const HUFFMAN_MAX_CODE_LENGTHS_SIZE: usize = 704;

/// Maximum possible Huffman table size for an alphabet size of 704, max code length 15 and root table bits 8.
const HUFFMAN_MAX_TABLE_SIZE: usize = 1080;

/// Maximum number of bits used to represent a (15-bit) code.
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

    /// The `codes` field contains repeats, so the real alphabet size needs to be seperately tracked.
    alphabet_size: u16
}

impl Default for Table  {
    fn default() -> Self {
        Table { codes: [Code::default();HUFFMAN_MAX_TABLE_SIZE], alphabet_size: 0 }
    }
}

impl Index<u32> for Table {
    type Output = Code;

    /// Relies on the table having elements repeated. Looks up the first 8 bits,
    /// which works only because short codes are duplicated such that the unused 
    /// trailing bits can have any value. Any remaining bits are looked
    /// up with an exact mask.
    #[inline(always)]
    fn index(&self, bits: u32 ) -> &Code {
        const HUFFMAN_TABLE_BITS: u8 = 8;
        const HUFFMAN_TABLE_MASK: u32 = 0xff;

        let mut offs = ( bits & HUFFMAN_TABLE_MASK ) as usize;
      
        if self.codes[offs].bits > HUFFMAN_TABLE_BITS {
            let nbits = self.codes[offs].bits - HUFFMAN_TABLE_BITS;           
            offs += ( self.codes[offs].value as usize ) + ((bits >> HUFFMAN_TABLE_BITS) & !((0xffffffff) << nbits )) as usize;
        }
        
        &self.codes[offs]
    }
}

impl Table {

    /// Decodes the next Huffman code from a BitReader. Reads 0 - 15 bits.
    #[inline(always)]
    pub fn read_huffman_symbol<BR:BitReader>( &self, br: &mut BR ) -> u16 {
        let code = self[br.get_bits_unmasked()];
        br.drop_bits( code.bits as u32 );
        
        code.value
    }

    fn get_next_key( key:u32 , len: u32 ) -> u32 {
        let mut step = 1 << (len - 1);
        while ( key & step ) != 0 {
            step >>= 1;
        }
    
        (key & (step - 1)) + step
    }

    /// Copy a value into the destination slice starting at an offset,
    /// repeated every `repeat` steps. Used to construct tables that
    /// can consume more than 1 bit at a time.
    #[inline(always)]
    fn replicate( &mut self, offs: usize, repeat: usize ) {
        // TODO: unstable feature "step_by()" can make this more elegant.
        let mut i = offs + repeat;
        let t = self.codes[offs];
        while i < self.codes.len() {
            self.codes[i] = t;
            i += repeat;
        }
    }   

    /// Because of the way the Brotli tables are encoded, they can be decoded
    /// from an array of code-lengths only.
    pub fn build_from_code_lengths( &mut self, lengths: &[u8] ) -> Result<()> {

        // TODO: Check if this is > or >=
        if lengths.len() > HUFFMAN_MAX_CODE_LENGTHS_SIZE {
            return Err( BrotliError::InvalidEncoding)
        }

        // Compute histogram of bit-lengths
        let mut code_length_histogram = [0;HUFFMAN_MAX_CODE_LENGTH+1];
        for bits in lengths {
            if *bits > HUFFMAN_MAX_CODE_LENGTH as u8 {
                return Err( BrotliError::InvalidEncoding )
            }

            code_length_histogram[*bits as usize] += 1; 
        }

        // Find the numerical value of the smallest code for each code length
        let mut next_code = [0u16;HUFFMAN_MAX_CODE_LENGTH+1];
        let mut code = 0;
        for bits in 1 .. HUFFMAN_MAX_CODE_LENGTH+1 {
            code = (code + code_length_histogram[bits-1]) << 1;
            next_code[bits as usize] = code;
        }

        for symbol_value in 0..lengths.len() {
            let bits = lengths[symbol_value] as usize;
            let code = next_code[bits] as usize;
            next_code[bits] += 1;

            // @@@ TODO: This is logically correct, but "not it".
            // Brotli uses a fancy 8-bit prefix decode 
            self.codes[code].bits = bits as u8;
            self.codes[code].value = symbol_value as u16;
            
            // Repeat values to allow lookup
            self.replicate( code, 1 << bits );
        }

        Ok( () )
    }
}

