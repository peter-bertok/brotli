use bitreader::BitReader;
use huffman::Code; 
use std::io::{Read,Result};

static CODE_LENGTH_CODE_ORDER: [u8; 18] = [1, 2, 3, 4, 0, 5, 17, 6, 16, 7, 8, 9, 10, 11, 12, 13, 14, 15 ];

struct Decoder<BR> {
    br: BR
}

const HUFFMAN_TABLE_BITS: u8 = 8;

const HUFFMAN_TABLE_MASK: u32 = 0xff;

impl<BR: BitReader> Read for Decoder<BR> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        unimplemented!();
    }
}

impl <BR: BitReader> Decoder<BR> {
    /// Decodes the next Huffman code from bit-stream. Reads 0 - 15 bits.
    /// Returns the number of bits consumed from the input
    fn read_huffman_symbol<BitReader>( br: &mut BR, table: &[Code]  ) -> u16 {
        unimplemented!();
        /*let bits = br.get_bits_unmasked();
        let mut offs = ( bits & HUFFMAN_TABLE_MASK ) as usize;
      
        if table[offs].bits > HUFFMAN_TABLE_BITS {
            let nbits = table[offs].bits - HUFFMAN_TABLE_BITS;
            br.drop_bits( HUFFMAN_TABLE_BITS as u32 );
            offs += table[offs].value + ((bits >> HUFFMAN_TABLE_BITS) & !((0xffffffff) << nbits ));
        }
        br.drop_bits( table[offs].bits );
        
        return table[offs].value; */
    }
    
    fn decode_window_bits( br: &mut BR ) -> u32 
    {
        if br.read_bits(1) == 0 {
            return 16;
        }
        else
        {
            let n = br.read_bits(3);
            if n != 0 {
                return n + 17;
            }
            else
            {
                let n = br.read_bits(3);
                if n != 0 {
                    return n + 8;
                }
                else {
                    return 17;
                }
            }
        }
    }
}