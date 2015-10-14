use bitreader::BitReader;
use huffman::Code; 
use std::io::{Read,Result};

static CODE_LENGTH_CODE_ORDER: [u8; 18] = [1, 2, 3, 4, 0, 5, 17, 6, 16, 7, 8, 9, 10, 11, 12, 13, 14, 15 ];

struct Decoder<BR> {
    br: BR
}

impl<BR: BitReader> Read for Decoder<BR> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        unimplemented!();
    }
}

impl <BR: BitReader> Decoder<BR> {

    
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