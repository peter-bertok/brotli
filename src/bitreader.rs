use std::io::{ Read, Write, ErrorKind };

// TODO: Move into BitReader trait (associated constants)
const MAX_NUM_BIT_READ : u32 = 25;
const READ_SIZE: usize = 1024;
const IMPLICIT_ZEROES: usize = 128;
const IBUF_SIZE: usize = (READ_SIZE + IMPLICIT_ZEROES);
const IBUF_MASK: usize =  (READ_SIZE - 1);

// TODO: This reader might have cleaner code if the input can be read from EITHER
// the bit buffer OR the byte buffer. That is, the bit offset can never be more than
// 7 bits.
// Also, I'm not a fan of the way implicit zeroes is implemented. Might be more elegant
// to use one of the Read implementations that chains the base reader with 0 repeated.

/// Data source for the Brotli decoder. The default implementation wraps `std::io::Read`,
/// or a custom implementation can be provided.
pub trait BitReader {

    /// Ensures that accumulator is not empty. May consume one byte of input.
    /// Returns `false` if data is required but there is no input available.
    #[must_use()]
    fn warmup( &mut self ) -> bool;

    /// returns the number of unread bytes remaining in the buffer.
    #[must_use()]
    fn get_remaining_bytes( &self ) -> usize;

    /// Checks if there is at least num bytes left in the input ringbuffer (excluding
    /// the bits remaining in the buffer). The maximum value for num is `IMPLICIT_ZEROES` bytes.
    #[must_use()]
    fn check_input_amount( &self, bytes: usize ) -> bool;

    /// Like `get_bits()`, but does not mask the result, it is only guaranteed that it has minimum of 24 bits.
    #[must_use()]
    fn get_bits_unmasked( &mut self ) -> u32;

    /// Returns the specified number of bits without advancing bit pos.
    #[inline(always)]
    #[must_use()]
    fn get_bits( &mut self, bits: u32 ) -> u32
    {
        debug_assert!( bits <= 32 );
        self.get_bits_unmasked() & !((0xffffffff) << bits )
    }

    /// Advances the bit position by `bits`.
    fn drop_bits( &mut self, bits: u32 );

    /// Reads the specified number of bits and advances the bit position.
    /// Precondition: accumulator MUST contain the required number of bits.
    /// #panic
    /// If there are insufficient number of bits remaining in the input, this function will panic in debug builds.
    #[must_use()]
    fn take_bits( &mut self, bits: u32 ) -> u32;

    /// Returns the specified number of bits without advancing the bit position.
    /// #panic
    /// If there are insufficient number of bits remaining in the input, this function will panic in debug builds.
    #[must_use()]
    fn read_bits( &mut self, bits: u32 ) -> u32;
    
    /// Same as `read_bits`, but returns `None` if there is insufficient input available.
    #[must_use()]
    fn safe_read_bits( &mut self, bits: u32 ) -> Option<u32>;

    /// Advances the bit reader position to the next byte boundary and verifies that any skipped bits are set to zero.
    #[must_use()]
    fn jump_to_byte_boundary( &mut self ) -> bool;

    /// Peeks a byte at specified offset. Precondition: bit reader is parked to a byte boundry.
    #[must_use()]
    fn peek_byte( &self, offset: usize ) -> Option<u8>;

    /// Copies remaining input bytes stored in the bit reader to the output, which may not be 
    /// larger than get_remaining_bytes(). The bit reader must be warmed up again after this.
    fn copy_bytes( &mut self, mut dst: &mut [u8] );

    /// Checks that bit reader hasn't read after the end of input.
    /// Returns false if bit reader has used implicit zeroes after the end of input.
    #[must_use()]
    fn is_reader_okay( &self ) -> bool;
}

/// Wraps a `Read` stream to implement the `BitReader` trait for the Brotli decoder.
pub struct StreamBitReader<R> {
    pub reader: R,
    
    // Pre-fetched bits from 'reader'
    // TODO: add conditional compilation to support 32-bit
    prefetch_bits: u64,

    // bit position in 'prefetchBits'
    bit_position: u32,

    // the byte we're reading from
    buf_position: usize,

    // the number of bytes remaining
    available_bytes: usize,

    end_of_stream: bool,

    // Input byte buffer, consist of a ringbuffer and a "slack" region where 
    // bytes from the start of the ringbuffer are copied.
    buf: [u8; IBUF_SIZE]
}

impl<R: Read + Sized> StreamBitReader<R> {

    // *********** These methods are not implementations of the trait *********** //
    pub fn new(reader: R) -> StreamBitReader<R> {
        StreamBitReader {
            reader: reader,
            prefetch_bits: 0,
            bit_position: 64, // TODO: add conditional compilation to support 32-bit
            buf_position: 0,
            available_bytes: 0,
            end_of_stream: false,
            buf: [0; IBUF_SIZE]
        }
    }

    fn read_input( &mut self, finish: bool ) -> bool
    {
        if self.end_of_stream 
        {
            false
        }
        else
        {
            // Shift buffer contents to the front
            if self.buf_position != 0 
            {
                // TODO: Replace with overlapping memcpy
                for i in 0 .. self.available_bytes
                {
                    self.buf[i] = self.buf[i+self.buf_position];
                }
                self.buf_position = 0;
            }

            match self.reader.read( & mut self.buf[0 .. READ_SIZE - self.available_bytes] )
            {
                Ok( bytes_read ) => { 
                    self.available_bytes += bytes_read; 
                    if bytes_read < READ_SIZE
                    {
                        if !finish
                        {
                            return false;
                        }

                        self.end_of_stream = true;
                        let start = self.buf_position+self.available_bytes;

                        // TODO: Use memset or zeromemory, but this is probably optimised away anyway.
                        for byte in &mut self.buf[start .. start + IMPLICIT_ZEROES]
                        {
                            *byte = 0;
                        }
                        self.available_bytes += IMPLICIT_ZEROES;
                    }
                    return true;
                },
                Err(ref e) if e.kind() == ErrorKind::Interrupted => {
                    return true;
                },
                _ => { return false; }
            }
        }
    }

    // Guarantees that there are at least 24 bits in accumulator.
    #[inline(always)]
    fn fill_bit_window( &mut self )
    {
        if self.bit_position >= 32
        {
            self.prefetch_bits >>= 32;
            self.bit_position -= 32;
            // TODO: optimise using mem::transmute
            self.prefetch_bits |= 
                ( self.buf[self.buf_position]   as u64 ) << 56 |
                ( self.buf[self.buf_position+1] as u64 ) << 48 |
                ( self.buf[self.buf_position+2] as u64 ) << 40 |
                ( self.buf[self.buf_position+3] as u64 ) << 32;
            self.available_bytes -= 4;
            self.buf_position += 4;
        }
    }  
}

impl<R: Read + Sized> BitReader for StreamBitReader<R> {
    
    fn warmup( &mut self ) -> bool
    {
        // SAFETY: additional bit_position check, more paranoid comparisons
        if self.bit_position >= 64 && self.available_bytes <= 0 || self.bit_position < 8
        {
            false
        }
        else
        {
            self.bit_position -= 8;
            self.prefetch_bits = self.buf[self.buf_position] as u64;
            self.prefetch_bits <<= self.bit_position;
            self.buf_position += 1;
            self.available_bytes -= 1;

            true
        }
    }

    #[inline(always)]
    fn get_remaining_bytes( &self ) -> usize
    {
        // TODO: add conditional compilation to support 32-bit
        let total = self.available_bytes + 64 - ( self.bit_position >> 3 ) as usize;
        if !self.end_of_stream 
        { 
            total 
        }
        else
        {
            if total <= IMPLICIT_ZEROES
            {
                0
            }
            else
            {
                total - IMPLICIT_ZEROES
            }
        }
    }

    #[inline(always)]
    fn check_input_amount( &self, bytes: usize ) -> bool
    {
        // SAFETY: Additional check (should be compiled out if inlined vs a constant)
        bytes <= IMPLICIT_ZEROES && self.available_bytes >= bytes
    }

    #[inline(always)]
    fn get_bits_unmasked( &mut self ) -> u32
    {
        self.fill_bit_window();
        
        ( self.prefetch_bits >> self.bit_position ) as u32
    }

    #[inline(always)]
    fn get_bits( &mut self, bits: u32 ) -> u32
    {
        debug_assert!( bits <= 32 );
        self.get_bits_unmasked() & !((0xffffffff) << bits )
    }

    #[inline(always)]
    fn drop_bits( &mut self, bits: u32 )
    {
        debug_assert!( self.bit_position + bits <= 64 );
        self.bit_position += bits;
    }

    #[inline(always)]
    fn take_bits( &mut self, bits: u32 ) -> u32 
    { 
        debug_assert!( self.bit_position + bits <= 64 );
        let result = ( self.prefetch_bits >> self.bit_position ) & !((0xffffffff) << bits );
        self.bit_position += bits;
        
        result as u32
    }

    #[inline(always)]
    fn read_bits( &mut self, bits: u32 ) -> u32 
    {
        self.fill_bit_window();
        self.take_bits( bits )
    }
    
    #[inline(always)]
    fn safe_read_bits( &mut self, bits: u32 ) -> Option<u32> 
    {
        loop 
        {
            if self.bit_position + bits <= 64
            {
                break;
            }
            else if self.available_bytes == 0
            {
                return Option::None
            }
            else
            {
                self.prefetch_bits >>= 8;
                self.prefetch_bits |= ( self.buf[self.buf_position] as u64 ) << 56;
                self.bit_position -= 8;
                self.available_bytes -= 1;
                self.buf_position += 1;
            }
        }
        
        Option::Some( self.take_bits( bits ))
    }

    #[inline(always)]
    fn jump_to_byte_boundary(&mut self) -> bool
    {
        let pad_bits_count = ( 64 - self.bit_position ) & 0x7;
        if pad_bits_count != 0 {
            return self.take_bits( pad_bits_count ) == 0
        }
        else
        {
            true
        }     
    }

    #[inline(always)]
    fn peek_byte( &self, offset: usize ) -> Option<u8>
    {
        if self.bit_position & 7 != 0  
        {
            return Option::None
        }

        let bytes_left = ( 8 - (self.bit_position >> 3)) as usize;

        if offset < bytes_left
        {
            return Option::Some( ((self.prefetch_bits >> (self.bit_position as usize + (offset << 3))) & 0xFF ) as u8 );
        }
        
        let offset_sub = offset - bytes_left;

        if offset_sub < self.available_bytes
        {
            return Option::Some( self.buf[self.buf_position+offset_sub] );
        }
        return Option::None
    }

    fn copy_bytes( &mut self, dst: &mut [u8] ) 
    {
        let mut dst_offs = 0;
        while self.bit_position + 8 <= 64 && dst.len() > dst_offs
        {
            dst[dst_offs] = ( self.prefetch_bits >> self.bit_position ) as u8;
            self.bit_position += 8;
            dst_offs += 1;
        }

        let src = &self.buf[self.buf_position .. self.buf_position + dst.len() - dst_offs];
        // UNSTABLE: bytes::copy_memory( src, dst );
        (&mut dst[dst_offs..]).write( src ).unwrap();
        self.available_bytes -= dst.len();
        self.buf_position += dst.len();
    }

    #[inline(always)]
    fn is_reader_okay( &self) -> bool 
    {
        let remaining_bytes = self.available_bytes + 8 - ((self.bit_position >> 3) as usize);
        return !self.end_of_stream || (remaining_bytes >= IMPLICIT_ZEROES);
    }
}