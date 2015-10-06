use bitreader;

static CODE_LENGTH_CODE_ORDER: [u8; 18] = [1, 2, 3, 4, 0, 5, 17, 6, 16, 7, 8, 9, 10, 11, 12, 13, 14, 15 ];




/*
impl<BR: BitReader> Read for Decoder<BR> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        unimplemented!();
    }
    
    fn decode_window_bits( &mut self ) -> u32 
    {
        if self.brotli_read_bits(1) == 0 {
            return 16;
        }
        else
        {
            let n = self.brotli_read_bits(3);
            if n != 0 {
                return n + 17;
            }
            else
            {
                let n = self.brotli_read_bits(3);
                if n != 0 {
                    return n + 8;
                }
                else
                {
                    return 17;
                }
            }
        }
    }
}*/