use bitreader::BitReader;
use std::io::{Read,Result};

static CODE_LENGTH_CODE_ORDER: [u8; 18] = [1, 2, 3, 4, 0, 5, 17, 6, 16, 7, 8, 9, 10, 11, 12, 13, 14, 15 ];

struct Decoder<BR> {
    br: BR
}

impl<BR: BitReader> Read for Decoder<BR> {
    fn read(&mut self, _buf: &mut [u8]) -> Result<usize> {
        unimplemented!();
    }
}

impl <BR: BitReader> Decoder<BR> {

    pub fn new(_source: BR) -> Decoder<BR> {
        unimplemented!();
    }

    /* @@@ TODO
    pub fn new_with_custom_dict<'a>( source: BR, dict: &'a [u8] ) -> Decoder<'a,BR> {
        unimplemented!();
    } */

    fn decode_window_bits(br: &mut BR) -> u32 {
        if br.read_bits(1) == 0 { 
            16
        } else {
            let n = br.read_bits(3);
            if n != 0 {
                n + 17
            } else {
                let n = br.read_bits(3);
                if n != 0 { n + 8 } else { 17 }
            }
        }
    }
}
