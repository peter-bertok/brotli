// Represents the range of values belonging to a prefix code: [offset, offset + 2^nbits)
#[derive(Debug, Copy, Clone)]
pub struct Range {
    offset: u16,
    nbits: u8
}

pub const PREFIX_CODE_RANGE: [Range;26] = [
    Range { offset:    1,  nbits:  2}, Range { offset:     5,  nbits: 2},  Range { offset:    9,  nbits: 2},  Range { offset:   13,  nbits: 2},
    Range { offset:   17,  nbits:  3}, Range { offset:    25,  nbits: 3},  Range { offset:   33,  nbits: 3},  Range { offset:   41,  nbits: 3},
    Range { offset:   49,  nbits:  4}, Range { offset:    65,  nbits: 4},  Range { offset:   81,  nbits: 4},  Range { offset:   97,  nbits: 4},
    Range { offset:  113,  nbits:  5}, Range { offset:   145,  nbits: 5},  Range { offset:  177,  nbits: 5},  Range { offset:  209,  nbits: 5},
    Range { offset:  241,  nbits:  6}, Range { offset:   305,  nbits: 6},  Range { offset:  369,  nbits: 7},  Range { offset:  497,  nbits: 8},
    Range { offset:  753,  nbits:  9}, Range { offset:  1265,  nbits: 10}, Range { offset: 2289,  nbits: 11}, Range { offset: 4337,  nbits: 12},
    Range { offset: 8433,  nbits: 13}, Range { offset: 16625,  nbits: 24}
];

#[derive(Debug, Copy, Clone)]
pub struct CmdLutElement {
    insert_len_extra_bits: u8,
    copy_len_extra_bits: u8,
    distance_code: bool,
    context: u8,
    insert_len_offset: u16,
    copy_len_offset: u16
}

// Convenience macro to reduce the amount of boilerplate for the constant lookup table below
macro_rules! lut {
    ($insert_len_extra_bits: expr,
    $copy_len_extra_bits: expr,
    $distance_code: expr,
    $context: expr,
    $insert_len_offset: expr,
    $copy_len_offset: expr ) => ( CmdLutElement {
        insert_len_extra_bits: $insert_len_extra_bits,
        copy_len_extra_bits: $copy_len_extra_bits,
        distance_code: $distance_code,
        context: $context,
        insert_len_offset: $insert_len_offset,
        copy_len_offset: $copy_len_offset
    } )
}

pub const CMD_LUT: [CmdLutElement;704] = [
    lut!( 0x00, 0x00, false, 0x00, 0x0000, 0x0002 ),
    lut!( 0x00, 0x00, false, 0x01, 0x0000, 0x0003 ),
    lut!( 0x00, 0x00, false, 0x02, 0x0000, 0x0004 ),
    lut!( 0x00, 0x00, false, 0x03, 0x0000, 0x0005 ),
    lut!( 0x00, 0x00, false, 0x03, 0x0000, 0x0006 ),
    lut!( 0x00, 0x00, false, 0x03, 0x0000, 0x0007 ),
    lut!( 0x00, 0x00, false, 0x03, 0x0000, 0x0008 ),
    lut!( 0x00, 0x00, false, 0x03, 0x0000, 0x0009 ),
    lut!( 0x00, 0x00, false, 0x00, 0x0001, 0x0002 ),
    lut!( 0x00, 0x00, false, 0x01, 0x0001, 0x0003 ),
    lut!( 0x00, 0x00, false, 0x02, 0x0001, 0x0004 ),
    lut!( 0x00, 0x00, false, 0x03, 0x0001, 0x0005 ),
    lut!( 0x00, 0x00, false, 0x03, 0x0001, 0x0006 ),
    lut!( 0x00, 0x00, false, 0x03, 0x0001, 0x0007 ),
    lut!( 0x00, 0x00, false, 0x03, 0x0001, 0x0008 ),
    lut!( 0x00, 0x00, false, 0x03, 0x0001, 0x0009 ),
    lut!( 0x00, 0x00, false, 0x00, 0x0002, 0x0002 ),
    lut!( 0x00, 0x00, false, 0x01, 0x0002, 0x0003 ),
    lut!( 0x00, 0x00, false, 0x02, 0x0002, 0x0004 ),
    lut!( 0x00, 0x00, false, 0x03, 0x0002, 0x0005 ),
    lut!( 0x00, 0x00, false, 0x03, 0x0002, 0x0006 ),
    lut!( 0x00, 0x00, false, 0x03, 0x0002, 0x0007 ),
    lut!( 0x00, 0x00, false, 0x03, 0x0002, 0x0008 ),
    lut!( 0x00, 0x00, false, 0x03, 0x0002, 0x0009 ),
    lut!( 0x00, 0x00, false, 0x00, 0x0003, 0x0002 ),
    lut!( 0x00, 0x00, false, 0x01, 0x0003, 0x0003 ),
    lut!( 0x00, 0x00, false, 0x02, 0x0003, 0x0004 ),
    lut!( 0x00, 0x00, false, 0x03, 0x0003, 0x0005 ),
    lut!( 0x00, 0x00, false, 0x03, 0x0003, 0x0006 ),
    lut!( 0x00, 0x00, false, 0x03, 0x0003, 0x0007 ),
    lut!( 0x00, 0x00, false, 0x03, 0x0003, 0x0008 ),
    lut!( 0x00, 0x00, false, 0x03, 0x0003, 0x0009 ),
    lut!( 0x00, 0x00, false, 0x00, 0x0004, 0x0002 ),
    lut!( 0x00, 0x00, false, 0x01, 0x0004, 0x0003 ),
    lut!( 0x00, 0x00, false, 0x02, 0x0004, 0x0004 ),
    lut!( 0x00, 0x00, false, 0x03, 0x0004, 0x0005 ),
    lut!( 0x00, 0x00, false, 0x03, 0x0004, 0x0006 ),
    lut!( 0x00, 0x00, false, 0x03, 0x0004, 0x0007 ),
    lut!( 0x00, 0x00, false, 0x03, 0x0004, 0x0008 ),
    lut!( 0x00, 0x00, false, 0x03, 0x0004, 0x0009 ),
    lut!( 0x00, 0x00, false, 0x00, 0x0005, 0x0002 ),
    lut!( 0x00, 0x00, false, 0x01, 0x0005, 0x0003 ),
    lut!( 0x00, 0x00, false, 0x02, 0x0005, 0x0004 ),
    lut!( 0x00, 0x00, false, 0x03, 0x0005, 0x0005 ),
    lut!( 0x00, 0x00, false, 0x03, 0x0005, 0x0006 ),
    lut!( 0x00, 0x00, false, 0x03, 0x0005, 0x0007 ),
    lut!( 0x00, 0x00, false, 0x03, 0x0005, 0x0008 ),
    lut!( 0x00, 0x00, false, 0x03, 0x0005, 0x0009 ),
    lut!( 0x01, 0x00, false, 0x00, 0x0006, 0x0002 ),
    lut!( 0x01, 0x00, false, 0x01, 0x0006, 0x0003 ),
    lut!( 0x01, 0x00, false, 0x02, 0x0006, 0x0004 ),
    lut!( 0x01, 0x00, false, 0x03, 0x0006, 0x0005 ),
    lut!( 0x01, 0x00, false, 0x03, 0x0006, 0x0006 ),
    lut!( 0x01, 0x00, false, 0x03, 0x0006, 0x0007 ),
    lut!( 0x01, 0x00, false, 0x03, 0x0006, 0x0008 ),
    lut!( 0x01, 0x00, false, 0x03, 0x0006, 0x0009 ),
    lut!( 0x01, 0x00, false, 0x00, 0x0008, 0x0002 ),
    lut!( 0x01, 0x00, false, 0x01, 0x0008, 0x0003 ),
    lut!( 0x01, 0x00, false, 0x02, 0x0008, 0x0004 ),
    lut!( 0x01, 0x00, false, 0x03, 0x0008, 0x0005 ),
    lut!( 0x01, 0x00, false, 0x03, 0x0008, 0x0006 ),
    lut!( 0x01, 0x00, false, 0x03, 0x0008, 0x0007 ),
    lut!( 0x01, 0x00, false, 0x03, 0x0008, 0x0008 ),
    lut!( 0x01, 0x00, false, 0x03, 0x0008, 0x0009 ),
    lut!( 0x00, 0x01, false, 0x03, 0x0000, 0x000a ),
    lut!( 0x00, 0x01, false, 0x03, 0x0000, 0x000c ),
    lut!( 0x00, 0x02, false, 0x03, 0x0000, 0x000e ),
    lut!( 0x00, 0x02, false, 0x03, 0x0000, 0x0012 ),
    lut!( 0x00, 0x03, false, 0x03, 0x0000, 0x0016 ),
    lut!( 0x00, 0x03, false, 0x03, 0x0000, 0x001e ),
    lut!( 0x00, 0x04, false, 0x03, 0x0000, 0x0026 ),
    lut!( 0x00, 0x04, false, 0x03, 0x0000, 0x0036 ),
    lut!( 0x00, 0x01, false, 0x03, 0x0001, 0x000a ),
    lut!( 0x00, 0x01, false, 0x03, 0x0001, 0x000c ),
    lut!( 0x00, 0x02, false, 0x03, 0x0001, 0x000e ),
    lut!( 0x00, 0x02, false, 0x03, 0x0001, 0x0012 ),
    lut!( 0x00, 0x03, false, 0x03, 0x0001, 0x0016 ),
    lut!( 0x00, 0x03, false, 0x03, 0x0001, 0x001e ),
    lut!( 0x00, 0x04, false, 0x03, 0x0001, 0x0026 ),
    lut!( 0x00, 0x04, false, 0x03, 0x0001, 0x0036 ),
    lut!( 0x00, 0x01, false, 0x03, 0x0002, 0x000a ),
    lut!( 0x00, 0x01, false, 0x03, 0x0002, 0x000c ),
    lut!( 0x00, 0x02, false, 0x03, 0x0002, 0x000e ),
    lut!( 0x00, 0x02, false, 0x03, 0x0002, 0x0012 ),
    lut!( 0x00, 0x03, false, 0x03, 0x0002, 0x0016 ),
    lut!( 0x00, 0x03, false, 0x03, 0x0002, 0x001e ),
    lut!( 0x00, 0x04, false, 0x03, 0x0002, 0x0026 ),
    lut!( 0x00, 0x04, false, 0x03, 0x0002, 0x0036 ),
    lut!( 0x00, 0x01, false, 0x03, 0x0003, 0x000a ),
    lut!( 0x00, 0x01, false, 0x03, 0x0003, 0x000c ),
    lut!( 0x00, 0x02, false, 0x03, 0x0003, 0x000e ),
    lut!( 0x00, 0x02, false, 0x03, 0x0003, 0x0012 ),
    lut!( 0x00, 0x03, false, 0x03, 0x0003, 0x0016 ),
    lut!( 0x00, 0x03, false, 0x03, 0x0003, 0x001e ),
    lut!( 0x00, 0x04, false, 0x03, 0x0003, 0x0026 ),
    lut!( 0x00, 0x04, false, 0x03, 0x0003, 0x0036 ),
    lut!( 0x00, 0x01, false, 0x03, 0x0004, 0x000a ),
    lut!( 0x00, 0x01, false, 0x03, 0x0004, 0x000c ),
    lut!( 0x00, 0x02, false, 0x03, 0x0004, 0x000e ),
    lut!( 0x00, 0x02, false, 0x03, 0x0004, 0x0012 ),
    lut!( 0x00, 0x03, false, 0x03, 0x0004, 0x0016 ),
    lut!( 0x00, 0x03, false, 0x03, 0x0004, 0x001e ),
    lut!( 0x00, 0x04, false, 0x03, 0x0004, 0x0026 ),
    lut!( 0x00, 0x04, false, 0x03, 0x0004, 0x0036 ),
    lut!( 0x00, 0x01, false, 0x03, 0x0005, 0x000a ),
    lut!( 0x00, 0x01, false, 0x03, 0x0005, 0x000c ),
    lut!( 0x00, 0x02, false, 0x03, 0x0005, 0x000e ),
    lut!( 0x00, 0x02, false, 0x03, 0x0005, 0x0012 ),
    lut!( 0x00, 0x03, false, 0x03, 0x0005, 0x0016 ),
    lut!( 0x00, 0x03, false, 0x03, 0x0005, 0x001e ),
    lut!( 0x00, 0x04, false, 0x03, 0x0005, 0x0026 ),
    lut!( 0x00, 0x04, false, 0x03, 0x0005, 0x0036 ),
    lut!( 0x01, 0x01, false, 0x03, 0x0006, 0x000a ),
    lut!( 0x01, 0x01, false, 0x03, 0x0006, 0x000c ),
    lut!( 0x01, 0x02, false, 0x03, 0x0006, 0x000e ),
    lut!( 0x01, 0x02, false, 0x03, 0x0006, 0x0012 ),
    lut!( 0x01, 0x03, false, 0x03, 0x0006, 0x0016 ),
    lut!( 0x01, 0x03, false, 0x03, 0x0006, 0x001e ),
    lut!( 0x01, 0x04, false, 0x03, 0x0006, 0x0026 ),
    lut!( 0x01, 0x04, false, 0x03, 0x0006, 0x0036 ),
    lut!( 0x01, 0x01, false, 0x03, 0x0008, 0x000a ),
    lut!( 0x01, 0x01, false, 0x03, 0x0008, 0x000c ),
    lut!( 0x01, 0x02, false, 0x03, 0x0008, 0x000e ),
    lut!( 0x01, 0x02, false, 0x03, 0x0008, 0x0012 ),
    lut!( 0x01, 0x03, false, 0x03, 0x0008, 0x0016 ),
    lut!( 0x01, 0x03, false, 0x03, 0x0008, 0x001e ),
    lut!( 0x01, 0x04, false, 0x03, 0x0008, 0x0026 ),
    lut!( 0x01, 0x04, false, 0x03, 0x0008, 0x0036 ),
    lut!( 0x00, 0x00, true, 0x00, 0x0000, 0x0002 ),
    lut!( 0x00, 0x00, true, 0x01, 0x0000, 0x0003 ),
    lut!( 0x00, 0x00, true, 0x02, 0x0000, 0x0004 ),
    lut!( 0x00, 0x00, true, 0x03, 0x0000, 0x0005 ),
    lut!( 0x00, 0x00, true, 0x03, 0x0000, 0x0006 ),
    lut!( 0x00, 0x00, true, 0x03, 0x0000, 0x0007 ),
    lut!( 0x00, 0x00, true, 0x03, 0x0000, 0x0008 ),
    lut!( 0x00, 0x00, true, 0x03, 0x0000, 0x0009 ),
    lut!( 0x00, 0x00, true, 0x00, 0x0001, 0x0002 ),
    lut!( 0x00, 0x00, true, 0x01, 0x0001, 0x0003 ),
    lut!( 0x00, 0x00, true, 0x02, 0x0001, 0x0004 ),
    lut!( 0x00, 0x00, true, 0x03, 0x0001, 0x0005 ),
    lut!( 0x00, 0x00, true, 0x03, 0x0001, 0x0006 ),
    lut!( 0x00, 0x00, true, 0x03, 0x0001, 0x0007 ),
    lut!( 0x00, 0x00, true, 0x03, 0x0001, 0x0008 ),
    lut!( 0x00, 0x00, true, 0x03, 0x0001, 0x0009 ),
    lut!( 0x00, 0x00, true, 0x00, 0x0002, 0x0002 ),
    lut!( 0x00, 0x00, true, 0x01, 0x0002, 0x0003 ),
    lut!( 0x00, 0x00, true, 0x02, 0x0002, 0x0004 ),
    lut!( 0x00, 0x00, true, 0x03, 0x0002, 0x0005 ),
    lut!( 0x00, 0x00, true, 0x03, 0x0002, 0x0006 ),
    lut!( 0x00, 0x00, true, 0x03, 0x0002, 0x0007 ),
    lut!( 0x00, 0x00, true, 0x03, 0x0002, 0x0008 ),
    lut!( 0x00, 0x00, true, 0x03, 0x0002, 0x0009 ),
    lut!( 0x00, 0x00, true, 0x00, 0x0003, 0x0002 ),
    lut!( 0x00, 0x00, true, 0x01, 0x0003, 0x0003 ),
    lut!( 0x00, 0x00, true, 0x02, 0x0003, 0x0004 ),
    lut!( 0x00, 0x00, true, 0x03, 0x0003, 0x0005 ),
    lut!( 0x00, 0x00, true, 0x03, 0x0003, 0x0006 ),
    lut!( 0x00, 0x00, true, 0x03, 0x0003, 0x0007 ),
    lut!( 0x00, 0x00, true, 0x03, 0x0003, 0x0008 ),
    lut!( 0x00, 0x00, true, 0x03, 0x0003, 0x0009 ),
    lut!( 0x00, 0x00, true, 0x00, 0x0004, 0x0002 ),
    lut!( 0x00, 0x00, true, 0x01, 0x0004, 0x0003 ),
    lut!( 0x00, 0x00, true, 0x02, 0x0004, 0x0004 ),
    lut!( 0x00, 0x00, true, 0x03, 0x0004, 0x0005 ),
    lut!( 0x00, 0x00, true, 0x03, 0x0004, 0x0006 ),
    lut!( 0x00, 0x00, true, 0x03, 0x0004, 0x0007 ),
    lut!( 0x00, 0x00, true, 0x03, 0x0004, 0x0008 ),
    lut!( 0x00, 0x00, true, 0x03, 0x0004, 0x0009 ),
    lut!( 0x00, 0x00, true, 0x00, 0x0005, 0x0002 ),
    lut!( 0x00, 0x00, true, 0x01, 0x0005, 0x0003 ),
    lut!( 0x00, 0x00, true, 0x02, 0x0005, 0x0004 ),
    lut!( 0x00, 0x00, true, 0x03, 0x0005, 0x0005 ),
    lut!( 0x00, 0x00, true, 0x03, 0x0005, 0x0006 ),
    lut!( 0x00, 0x00, true, 0x03, 0x0005, 0x0007 ),
    lut!( 0x00, 0x00, true, 0x03, 0x0005, 0x0008 ),
    lut!( 0x00, 0x00, true, 0x03, 0x0005, 0x0009 ),
    lut!( 0x01, 0x00, true, 0x00, 0x0006, 0x0002 ),
    lut!( 0x01, 0x00, true, 0x01, 0x0006, 0x0003 ),
    lut!( 0x01, 0x00, true, 0x02, 0x0006, 0x0004 ),
    lut!( 0x01, 0x00, true, 0x03, 0x0006, 0x0005 ),
    lut!( 0x01, 0x00, true, 0x03, 0x0006, 0x0006 ),
    lut!( 0x01, 0x00, true, 0x03, 0x0006, 0x0007 ),
    lut!( 0x01, 0x00, true, 0x03, 0x0006, 0x0008 ),
    lut!( 0x01, 0x00, true, 0x03, 0x0006, 0x0009 ),
    lut!( 0x01, 0x00, true, 0x00, 0x0008, 0x0002 ),
    lut!( 0x01, 0x00, true, 0x01, 0x0008, 0x0003 ),
    lut!( 0x01, 0x00, true, 0x02, 0x0008, 0x0004 ),
    lut!( 0x01, 0x00, true, 0x03, 0x0008, 0x0005 ),
    lut!( 0x01, 0x00, true, 0x03, 0x0008, 0x0006 ),
    lut!( 0x01, 0x00, true, 0x03, 0x0008, 0x0007 ),
    lut!( 0x01, 0x00, true, 0x03, 0x0008, 0x0008 ),
    lut!( 0x01, 0x00, true, 0x03, 0x0008, 0x0009 ),
    lut!( 0x00, 0x01, true, 0x03, 0x0000, 0x000a ),
    lut!( 0x00, 0x01, true, 0x03, 0x0000, 0x000c ),
    lut!( 0x00, 0x02, true, 0x03, 0x0000, 0x000e ),
    lut!( 0x00, 0x02, true, 0x03, 0x0000, 0x0012 ),
    lut!( 0x00, 0x03, true, 0x03, 0x0000, 0x0016 ),
    lut!( 0x00, 0x03, true, 0x03, 0x0000, 0x001e ),
    lut!( 0x00, 0x04, true, 0x03, 0x0000, 0x0026 ),
    lut!( 0x00, 0x04, true, 0x03, 0x0000, 0x0036 ),
    lut!( 0x00, 0x01, true, 0x03, 0x0001, 0x000a ),
    lut!( 0x00, 0x01, true, 0x03, 0x0001, 0x000c ),
    lut!( 0x00, 0x02, true, 0x03, 0x0001, 0x000e ),
    lut!( 0x00, 0x02, true, 0x03, 0x0001, 0x0012 ),
    lut!( 0x00, 0x03, true, 0x03, 0x0001, 0x0016 ),
    lut!( 0x00, 0x03, true, 0x03, 0x0001, 0x001e ),
    lut!( 0x00, 0x04, true, 0x03, 0x0001, 0x0026 ),
    lut!( 0x00, 0x04, true, 0x03, 0x0001, 0x0036 ),
    lut!( 0x00, 0x01, true, 0x03, 0x0002, 0x000a ),
    lut!( 0x00, 0x01, true, 0x03, 0x0002, 0x000c ),
    lut!( 0x00, 0x02, true, 0x03, 0x0002, 0x000e ),
    lut!( 0x00, 0x02, true, 0x03, 0x0002, 0x0012 ),
    lut!( 0x00, 0x03, true, 0x03, 0x0002, 0x0016 ),
    lut!( 0x00, 0x03, true, 0x03, 0x0002, 0x001e ),
    lut!( 0x00, 0x04, true, 0x03, 0x0002, 0x0026 ),
    lut!( 0x00, 0x04, true, 0x03, 0x0002, 0x0036 ),
    lut!( 0x00, 0x01, true, 0x03, 0x0003, 0x000a ),
    lut!( 0x00, 0x01, true, 0x03, 0x0003, 0x000c ),
    lut!( 0x00, 0x02, true, 0x03, 0x0003, 0x000e ),
    lut!( 0x00, 0x02, true, 0x03, 0x0003, 0x0012 ),
    lut!( 0x00, 0x03, true, 0x03, 0x0003, 0x0016 ),
    lut!( 0x00, 0x03, true, 0x03, 0x0003, 0x001e ),
    lut!( 0x00, 0x04, true, 0x03, 0x0003, 0x0026 ),
    lut!( 0x00, 0x04, true, 0x03, 0x0003, 0x0036 ),
    lut!( 0x00, 0x01, true, 0x03, 0x0004, 0x000a ),
    lut!( 0x00, 0x01, true, 0x03, 0x0004, 0x000c ),
    lut!( 0x00, 0x02, true, 0x03, 0x0004, 0x000e ),
    lut!( 0x00, 0x02, true, 0x03, 0x0004, 0x0012 ),
    lut!( 0x00, 0x03, true, 0x03, 0x0004, 0x0016 ),
    lut!( 0x00, 0x03, true, 0x03, 0x0004, 0x001e ),
    lut!( 0x00, 0x04, true, 0x03, 0x0004, 0x0026 ),
    lut!( 0x00, 0x04, true, 0x03, 0x0004, 0x0036 ),
    lut!( 0x00, 0x01, true, 0x03, 0x0005, 0x000a ),
    lut!( 0x00, 0x01, true, 0x03, 0x0005, 0x000c ),
    lut!( 0x00, 0x02, true, 0x03, 0x0005, 0x000e ),
    lut!( 0x00, 0x02, true, 0x03, 0x0005, 0x0012 ),
    lut!( 0x00, 0x03, true, 0x03, 0x0005, 0x0016 ),
    lut!( 0x00, 0x03, true, 0x03, 0x0005, 0x001e ),
    lut!( 0x00, 0x04, true, 0x03, 0x0005, 0x0026 ),
    lut!( 0x00, 0x04, true, 0x03, 0x0005, 0x0036 ),
    lut!( 0x01, 0x01, true, 0x03, 0x0006, 0x000a ),
    lut!( 0x01, 0x01, true, 0x03, 0x0006, 0x000c ),
    lut!( 0x01, 0x02, true, 0x03, 0x0006, 0x000e ),
    lut!( 0x01, 0x02, true, 0x03, 0x0006, 0x0012 ),
    lut!( 0x01, 0x03, true, 0x03, 0x0006, 0x0016 ),
    lut!( 0x01, 0x03, true, 0x03, 0x0006, 0x001e ),
    lut!( 0x01, 0x04, true, 0x03, 0x0006, 0x0026 ),
    lut!( 0x01, 0x04, true, 0x03, 0x0006, 0x0036 ),
    lut!( 0x01, 0x01, true, 0x03, 0x0008, 0x000a ),
    lut!( 0x01, 0x01, true, 0x03, 0x0008, 0x000c ),
    lut!( 0x01, 0x02, true, 0x03, 0x0008, 0x000e ),
    lut!( 0x01, 0x02, true, 0x03, 0x0008, 0x0012 ),
    lut!( 0x01, 0x03, true, 0x03, 0x0008, 0x0016 ),
    lut!( 0x01, 0x03, true, 0x03, 0x0008, 0x001e ),
    lut!( 0x01, 0x04, true, 0x03, 0x0008, 0x0026 ),
    lut!( 0x01, 0x04, true, 0x03, 0x0008, 0x0036 ),
    lut!( 0x02, 0x00, true, 0x00, 0x000a, 0x0002 ),
    lut!( 0x02, 0x00, true, 0x01, 0x000a, 0x0003 ),
    lut!( 0x02, 0x00, true, 0x02, 0x000a, 0x0004 ),
    lut!( 0x02, 0x00, true, 0x03, 0x000a, 0x0005 ),
    lut!( 0x02, 0x00, true, 0x03, 0x000a, 0x0006 ),
    lut!( 0x02, 0x00, true, 0x03, 0x000a, 0x0007 ),
    lut!( 0x02, 0x00, true, 0x03, 0x000a, 0x0008 ),
    lut!( 0x02, 0x00, true, 0x03, 0x000a, 0x0009 ),
    lut!( 0x02, 0x00, true, 0x00, 0x000e, 0x0002 ),
    lut!( 0x02, 0x00, true, 0x01, 0x000e, 0x0003 ),
    lut!( 0x02, 0x00, true, 0x02, 0x000e, 0x0004 ),
    lut!( 0x02, 0x00, true, 0x03, 0x000e, 0x0005 ),
    lut!( 0x02, 0x00, true, 0x03, 0x000e, 0x0006 ),
    lut!( 0x02, 0x00, true, 0x03, 0x000e, 0x0007 ),
    lut!( 0x02, 0x00, true, 0x03, 0x000e, 0x0008 ),
    lut!( 0x02, 0x00, true, 0x03, 0x000e, 0x0009 ),
    lut!( 0x03, 0x00, true, 0x00, 0x0012, 0x0002 ),
    lut!( 0x03, 0x00, true, 0x01, 0x0012, 0x0003 ),
    lut!( 0x03, 0x00, true, 0x02, 0x0012, 0x0004 ),
    lut!( 0x03, 0x00, true, 0x03, 0x0012, 0x0005 ),
    lut!( 0x03, 0x00, true, 0x03, 0x0012, 0x0006 ),
    lut!( 0x03, 0x00, true, 0x03, 0x0012, 0x0007 ),
    lut!( 0x03, 0x00, true, 0x03, 0x0012, 0x0008 ),
    lut!( 0x03, 0x00, true, 0x03, 0x0012, 0x0009 ),
    lut!( 0x03, 0x00, true, 0x00, 0x001a, 0x0002 ),
    lut!( 0x03, 0x00, true, 0x01, 0x001a, 0x0003 ),
    lut!( 0x03, 0x00, true, 0x02, 0x001a, 0x0004 ),
    lut!( 0x03, 0x00, true, 0x03, 0x001a, 0x0005 ),
    lut!( 0x03, 0x00, true, 0x03, 0x001a, 0x0006 ),
    lut!( 0x03, 0x00, true, 0x03, 0x001a, 0x0007 ),
    lut!( 0x03, 0x00, true, 0x03, 0x001a, 0x0008 ),
    lut!( 0x03, 0x00, true, 0x03, 0x001a, 0x0009 ),
    lut!( 0x04, 0x00, true, 0x00, 0x0022, 0x0002 ),
    lut!( 0x04, 0x00, true, 0x01, 0x0022, 0x0003 ),
    lut!( 0x04, 0x00, true, 0x02, 0x0022, 0x0004 ),
    lut!( 0x04, 0x00, true, 0x03, 0x0022, 0x0005 ),
    lut!( 0x04, 0x00, true, 0x03, 0x0022, 0x0006 ),
    lut!( 0x04, 0x00, true, 0x03, 0x0022, 0x0007 ),
    lut!( 0x04, 0x00, true, 0x03, 0x0022, 0x0008 ),
    lut!( 0x04, 0x00, true, 0x03, 0x0022, 0x0009 ),
    lut!( 0x04, 0x00, true, 0x00, 0x0032, 0x0002 ),
    lut!( 0x04, 0x00, true, 0x01, 0x0032, 0x0003 ),
    lut!( 0x04, 0x00, true, 0x02, 0x0032, 0x0004 ),
    lut!( 0x04, 0x00, true, 0x03, 0x0032, 0x0005 ),
    lut!( 0x04, 0x00, true, 0x03, 0x0032, 0x0006 ),
    lut!( 0x04, 0x00, true, 0x03, 0x0032, 0x0007 ),
    lut!( 0x04, 0x00, true, 0x03, 0x0032, 0x0008 ),
    lut!( 0x04, 0x00, true, 0x03, 0x0032, 0x0009 ),
    lut!( 0x05, 0x00, true, 0x00, 0x0042, 0x0002 ),
    lut!( 0x05, 0x00, true, 0x01, 0x0042, 0x0003 ),
    lut!( 0x05, 0x00, true, 0x02, 0x0042, 0x0004 ),
    lut!( 0x05, 0x00, true, 0x03, 0x0042, 0x0005 ),
    lut!( 0x05, 0x00, true, 0x03, 0x0042, 0x0006 ),
    lut!( 0x05, 0x00, true, 0x03, 0x0042, 0x0007 ),
    lut!( 0x05, 0x00, true, 0x03, 0x0042, 0x0008 ),
    lut!( 0x05, 0x00, true, 0x03, 0x0042, 0x0009 ),
    lut!( 0x05, 0x00, true, 0x00, 0x0062, 0x0002 ),
    lut!( 0x05, 0x00, true, 0x01, 0x0062, 0x0003 ),
    lut!( 0x05, 0x00, true, 0x02, 0x0062, 0x0004 ),
    lut!( 0x05, 0x00, true, 0x03, 0x0062, 0x0005 ),
    lut!( 0x05, 0x00, true, 0x03, 0x0062, 0x0006 ),
    lut!( 0x05, 0x00, true, 0x03, 0x0062, 0x0007 ),
    lut!( 0x05, 0x00, true, 0x03, 0x0062, 0x0008 ),
    lut!( 0x05, 0x00, true, 0x03, 0x0062, 0x0009 ),
    lut!( 0x02, 0x01, true, 0x03, 0x000a, 0x000a ),
    lut!( 0x02, 0x01, true, 0x03, 0x000a, 0x000c ),
    lut!( 0x02, 0x02, true, 0x03, 0x000a, 0x000e ),
    lut!( 0x02, 0x02, true, 0x03, 0x000a, 0x0012 ),
    lut!( 0x02, 0x03, true, 0x03, 0x000a, 0x0016 ),
    lut!( 0x02, 0x03, true, 0x03, 0x000a, 0x001e ),
    lut!( 0x02, 0x04, true, 0x03, 0x000a, 0x0026 ),
    lut!( 0x02, 0x04, true, 0x03, 0x000a, 0x0036 ),
    lut!( 0x02, 0x01, true, 0x03, 0x000e, 0x000a ),
    lut!( 0x02, 0x01, true, 0x03, 0x000e, 0x000c ),
    lut!( 0x02, 0x02, true, 0x03, 0x000e, 0x000e ),
    lut!( 0x02, 0x02, true, 0x03, 0x000e, 0x0012 ),
    lut!( 0x02, 0x03, true, 0x03, 0x000e, 0x0016 ),
    lut!( 0x02, 0x03, true, 0x03, 0x000e, 0x001e ),
    lut!( 0x02, 0x04, true, 0x03, 0x000e, 0x0026 ),
    lut!( 0x02, 0x04, true, 0x03, 0x000e, 0x0036 ),
    lut!( 0x03, 0x01, true, 0x03, 0x0012, 0x000a ),
    lut!( 0x03, 0x01, true, 0x03, 0x0012, 0x000c ),
    lut!( 0x03, 0x02, true, 0x03, 0x0012, 0x000e ),
    lut!( 0x03, 0x02, true, 0x03, 0x0012, 0x0012 ),
    lut!( 0x03, 0x03, true, 0x03, 0x0012, 0x0016 ),
    lut!( 0x03, 0x03, true, 0x03, 0x0012, 0x001e ),
    lut!( 0x03, 0x04, true, 0x03, 0x0012, 0x0026 ),
    lut!( 0x03, 0x04, true, 0x03, 0x0012, 0x0036 ),
    lut!( 0x03, 0x01, true, 0x03, 0x001a, 0x000a ),
    lut!( 0x03, 0x01, true, 0x03, 0x001a, 0x000c ),
    lut!( 0x03, 0x02, true, 0x03, 0x001a, 0x000e ),
    lut!( 0x03, 0x02, true, 0x03, 0x001a, 0x0012 ),
    lut!( 0x03, 0x03, true, 0x03, 0x001a, 0x0016 ),
    lut!( 0x03, 0x03, true, 0x03, 0x001a, 0x001e ),
    lut!( 0x03, 0x04, true, 0x03, 0x001a, 0x0026 ),
    lut!( 0x03, 0x04, true, 0x03, 0x001a, 0x0036 ),
    lut!( 0x04, 0x01, true, 0x03, 0x0022, 0x000a ),
    lut!( 0x04, 0x01, true, 0x03, 0x0022, 0x000c ),
    lut!( 0x04, 0x02, true, 0x03, 0x0022, 0x000e ),
    lut!( 0x04, 0x02, true, 0x03, 0x0022, 0x0012 ),
    lut!( 0x04, 0x03, true, 0x03, 0x0022, 0x0016 ),
    lut!( 0x04, 0x03, true, 0x03, 0x0022, 0x001e ),
    lut!( 0x04, 0x04, true, 0x03, 0x0022, 0x0026 ),
    lut!( 0x04, 0x04, true, 0x03, 0x0022, 0x0036 ),
    lut!( 0x04, 0x01, true, 0x03, 0x0032, 0x000a ),
    lut!( 0x04, 0x01, true, 0x03, 0x0032, 0x000c ),
    lut!( 0x04, 0x02, true, 0x03, 0x0032, 0x000e ),
    lut!( 0x04, 0x02, true, 0x03, 0x0032, 0x0012 ),
    lut!( 0x04, 0x03, true, 0x03, 0x0032, 0x0016 ),
    lut!( 0x04, 0x03, true, 0x03, 0x0032, 0x001e ),
    lut!( 0x04, 0x04, true, 0x03, 0x0032, 0x0026 ),
    lut!( 0x04, 0x04, true, 0x03, 0x0032, 0x0036 ),
    lut!( 0x05, 0x01, true, 0x03, 0x0042, 0x000a ),
    lut!( 0x05, 0x01, true, 0x03, 0x0042, 0x000c ),
    lut!( 0x05, 0x02, true, 0x03, 0x0042, 0x000e ),
    lut!( 0x05, 0x02, true, 0x03, 0x0042, 0x0012 ),
    lut!( 0x05, 0x03, true, 0x03, 0x0042, 0x0016 ),
    lut!( 0x05, 0x03, true, 0x03, 0x0042, 0x001e ),
    lut!( 0x05, 0x04, true, 0x03, 0x0042, 0x0026 ),
    lut!( 0x05, 0x04, true, 0x03, 0x0042, 0x0036 ),
    lut!( 0x05, 0x01, true, 0x03, 0x0062, 0x000a ),
    lut!( 0x05, 0x01, true, 0x03, 0x0062, 0x000c ),
    lut!( 0x05, 0x02, true, 0x03, 0x0062, 0x000e ),
    lut!( 0x05, 0x02, true, 0x03, 0x0062, 0x0012 ),
    lut!( 0x05, 0x03, true, 0x03, 0x0062, 0x0016 ),
    lut!( 0x05, 0x03, true, 0x03, 0x0062, 0x001e ),
    lut!( 0x05, 0x04, true, 0x03, 0x0062, 0x0026 ),
    lut!( 0x05, 0x04, true, 0x03, 0x0062, 0x0036 ),
    lut!( 0x00, 0x05, true, 0x03, 0x0000, 0x0046 ),
    lut!( 0x00, 0x05, true, 0x03, 0x0000, 0x0066 ),
    lut!( 0x00, 0x06, true, 0x03, 0x0000, 0x0086 ),
    lut!( 0x00, 0x07, true, 0x03, 0x0000, 0x00c6 ),
    lut!( 0x00, 0x08, true, 0x03, 0x0000, 0x0146 ),
    lut!( 0x00, 0x09, true, 0x03, 0x0000, 0x0246 ),
    lut!( 0x00, 0x0a, true, 0x03, 0x0000, 0x0446 ),
    lut!( 0x00, 0x18, true, 0x03, 0x0000, 0x0846 ),
    lut!( 0x00, 0x05, true, 0x03, 0x0001, 0x0046 ),
    lut!( 0x00, 0x05, true, 0x03, 0x0001, 0x0066 ),
    lut!( 0x00, 0x06, true, 0x03, 0x0001, 0x0086 ),
    lut!( 0x00, 0x07, true, 0x03, 0x0001, 0x00c6 ),
    lut!( 0x00, 0x08, true, 0x03, 0x0001, 0x0146 ),
    lut!( 0x00, 0x09, true, 0x03, 0x0001, 0x0246 ),
    lut!( 0x00, 0x0a, true, 0x03, 0x0001, 0x0446 ),
    lut!( 0x00, 0x18, true, 0x03, 0x0001, 0x0846 ),
    lut!( 0x00, 0x05, true, 0x03, 0x0002, 0x0046 ),
    lut!( 0x00, 0x05, true, 0x03, 0x0002, 0x0066 ),
    lut!( 0x00, 0x06, true, 0x03, 0x0002, 0x0086 ),
    lut!( 0x00, 0x07, true, 0x03, 0x0002, 0x00c6 ),
    lut!( 0x00, 0x08, true, 0x03, 0x0002, 0x0146 ),
    lut!( 0x00, 0x09, true, 0x03, 0x0002, 0x0246 ),
    lut!( 0x00, 0x0a, true, 0x03, 0x0002, 0x0446 ),
    lut!( 0x00, 0x18, true, 0x03, 0x0002, 0x0846 ),
    lut!( 0x00, 0x05, true, 0x03, 0x0003, 0x0046 ),
    lut!( 0x00, 0x05, true, 0x03, 0x0003, 0x0066 ),
    lut!( 0x00, 0x06, true, 0x03, 0x0003, 0x0086 ),
    lut!( 0x00, 0x07, true, 0x03, 0x0003, 0x00c6 ),
    lut!( 0x00, 0x08, true, 0x03, 0x0003, 0x0146 ),
    lut!( 0x00, 0x09, true, 0x03, 0x0003, 0x0246 ),
    lut!( 0x00, 0x0a, true, 0x03, 0x0003, 0x0446 ),
    lut!( 0x00, 0x18, true, 0x03, 0x0003, 0x0846 ),
    lut!( 0x00, 0x05, true, 0x03, 0x0004, 0x0046 ),
    lut!( 0x00, 0x05, true, 0x03, 0x0004, 0x0066 ),
    lut!( 0x00, 0x06, true, 0x03, 0x0004, 0x0086 ),
    lut!( 0x00, 0x07, true, 0x03, 0x0004, 0x00c6 ),
    lut!( 0x00, 0x08, true, 0x03, 0x0004, 0x0146 ),
    lut!( 0x00, 0x09, true, 0x03, 0x0004, 0x0246 ),
    lut!( 0x00, 0x0a, true, 0x03, 0x0004, 0x0446 ),
    lut!( 0x00, 0x18, true, 0x03, 0x0004, 0x0846 ),
    lut!( 0x00, 0x05, true, 0x03, 0x0005, 0x0046 ),
    lut!( 0x00, 0x05, true, 0x03, 0x0005, 0x0066 ),
    lut!( 0x00, 0x06, true, 0x03, 0x0005, 0x0086 ),
    lut!( 0x00, 0x07, true, 0x03, 0x0005, 0x00c6 ),
    lut!( 0x00, 0x08, true, 0x03, 0x0005, 0x0146 ),
    lut!( 0x00, 0x09, true, 0x03, 0x0005, 0x0246 ),
    lut!( 0x00, 0x0a, true, 0x03, 0x0005, 0x0446 ),
    lut!( 0x00, 0x18, true, 0x03, 0x0005, 0x0846 ),
    lut!( 0x01, 0x05, true, 0x03, 0x0006, 0x0046 ),
    lut!( 0x01, 0x05, true, 0x03, 0x0006, 0x0066 ),
    lut!( 0x01, 0x06, true, 0x03, 0x0006, 0x0086 ),
    lut!( 0x01, 0x07, true, 0x03, 0x0006, 0x00c6 ),
    lut!( 0x01, 0x08, true, 0x03, 0x0006, 0x0146 ),
    lut!( 0x01, 0x09, true, 0x03, 0x0006, 0x0246 ),
    lut!( 0x01, 0x0a, true, 0x03, 0x0006, 0x0446 ),
    lut!( 0x01, 0x18, true, 0x03, 0x0006, 0x0846 ),
    lut!( 0x01, 0x05, true, 0x03, 0x0008, 0x0046 ),
    lut!( 0x01, 0x05, true, 0x03, 0x0008, 0x0066 ),
    lut!( 0x01, 0x06, true, 0x03, 0x0008, 0x0086 ),
    lut!( 0x01, 0x07, true, 0x03, 0x0008, 0x00c6 ),
    lut!( 0x01, 0x08, true, 0x03, 0x0008, 0x0146 ),
    lut!( 0x01, 0x09, true, 0x03, 0x0008, 0x0246 ),
    lut!( 0x01, 0x0a, true, 0x03, 0x0008, 0x0446 ),
    lut!( 0x01, 0x18, true, 0x03, 0x0008, 0x0846 ),
    lut!( 0x06, 0x00, true, 0x00, 0x0082, 0x0002 ),
    lut!( 0x06, 0x00, true, 0x01, 0x0082, 0x0003 ),
    lut!( 0x06, 0x00, true, 0x02, 0x0082, 0x0004 ),
    lut!( 0x06, 0x00, true, 0x03, 0x0082, 0x0005 ),
    lut!( 0x06, 0x00, true, 0x03, 0x0082, 0x0006 ),
    lut!( 0x06, 0x00, true, 0x03, 0x0082, 0x0007 ),
    lut!( 0x06, 0x00, true, 0x03, 0x0082, 0x0008 ),
    lut!( 0x06, 0x00, true, 0x03, 0x0082, 0x0009 ),
    lut!( 0x07, 0x00, true, 0x00, 0x00c2, 0x0002 ),
    lut!( 0x07, 0x00, true, 0x01, 0x00c2, 0x0003 ),
    lut!( 0x07, 0x00, true, 0x02, 0x00c2, 0x0004 ),
    lut!( 0x07, 0x00, true, 0x03, 0x00c2, 0x0005 ),
    lut!( 0x07, 0x00, true, 0x03, 0x00c2, 0x0006 ),
    lut!( 0x07, 0x00, true, 0x03, 0x00c2, 0x0007 ),
    lut!( 0x07, 0x00, true, 0x03, 0x00c2, 0x0008 ),
    lut!( 0x07, 0x00, true, 0x03, 0x00c2, 0x0009 ),
    lut!( 0x08, 0x00, true, 0x00, 0x0142, 0x0002 ),
    lut!( 0x08, 0x00, true, 0x01, 0x0142, 0x0003 ),
    lut!( 0x08, 0x00, true, 0x02, 0x0142, 0x0004 ),
    lut!( 0x08, 0x00, true, 0x03, 0x0142, 0x0005 ),
    lut!( 0x08, 0x00, true, 0x03, 0x0142, 0x0006 ),
    lut!( 0x08, 0x00, true, 0x03, 0x0142, 0x0007 ),
    lut!( 0x08, 0x00, true, 0x03, 0x0142, 0x0008 ),
    lut!( 0x08, 0x00, true, 0x03, 0x0142, 0x0009 ),
    lut!( 0x09, 0x00, true, 0x00, 0x0242, 0x0002 ),
    lut!( 0x09, 0x00, true, 0x01, 0x0242, 0x0003 ),
    lut!( 0x09, 0x00, true, 0x02, 0x0242, 0x0004 ),
    lut!( 0x09, 0x00, true, 0x03, 0x0242, 0x0005 ),
    lut!( 0x09, 0x00, true, 0x03, 0x0242, 0x0006 ),
    lut!( 0x09, 0x00, true, 0x03, 0x0242, 0x0007 ),
    lut!( 0x09, 0x00, true, 0x03, 0x0242, 0x0008 ),
    lut!( 0x09, 0x00, true, 0x03, 0x0242, 0x0009 ),
    lut!( 0x0a, 0x00, true, 0x00, 0x0442, 0x0002 ),
    lut!( 0x0a, 0x00, true, 0x01, 0x0442, 0x0003 ),
    lut!( 0x0a, 0x00, true, 0x02, 0x0442, 0x0004 ),
    lut!( 0x0a, 0x00, true, 0x03, 0x0442, 0x0005 ),
    lut!( 0x0a, 0x00, true, 0x03, 0x0442, 0x0006 ),
    lut!( 0x0a, 0x00, true, 0x03, 0x0442, 0x0007 ),
    lut!( 0x0a, 0x00, true, 0x03, 0x0442, 0x0008 ),
    lut!( 0x0a, 0x00, true, 0x03, 0x0442, 0x0009 ),
    lut!( 0x0c, 0x00, true, 0x00, 0x0842, 0x0002 ),
    lut!( 0x0c, 0x00, true, 0x01, 0x0842, 0x0003 ),
    lut!( 0x0c, 0x00, true, 0x02, 0x0842, 0x0004 ),
    lut!( 0x0c, 0x00, true, 0x03, 0x0842, 0x0005 ),
    lut!( 0x0c, 0x00, true, 0x03, 0x0842, 0x0006 ),
    lut!( 0x0c, 0x00, true, 0x03, 0x0842, 0x0007 ),
    lut!( 0x0c, 0x00, true, 0x03, 0x0842, 0x0008 ),
    lut!( 0x0c, 0x00, true, 0x03, 0x0842, 0x0009 ),
    lut!( 0x0e, 0x00, true, 0x00, 0x1842, 0x0002 ),
    lut!( 0x0e, 0x00, true, 0x01, 0x1842, 0x0003 ),
    lut!( 0x0e, 0x00, true, 0x02, 0x1842, 0x0004 ),
    lut!( 0x0e, 0x00, true, 0x03, 0x1842, 0x0005 ),
    lut!( 0x0e, 0x00, true, 0x03, 0x1842, 0x0006 ),
    lut!( 0x0e, 0x00, true, 0x03, 0x1842, 0x0007 ),
    lut!( 0x0e, 0x00, true, 0x03, 0x1842, 0x0008 ),
    lut!( 0x0e, 0x00, true, 0x03, 0x1842, 0x0009 ),
    lut!( 0x18, 0x00, true, 0x00, 0x5842, 0x0002 ),
    lut!( 0x18, 0x00, true, 0x01, 0x5842, 0x0003 ),
    lut!( 0x18, 0x00, true, 0x02, 0x5842, 0x0004 ),
    lut!( 0x18, 0x00, true, 0x03, 0x5842, 0x0005 ),
    lut!( 0x18, 0x00, true, 0x03, 0x5842, 0x0006 ),
    lut!( 0x18, 0x00, true, 0x03, 0x5842, 0x0007 ),
    lut!( 0x18, 0x00, true, 0x03, 0x5842, 0x0008 ),
    lut!( 0x18, 0x00, true, 0x03, 0x5842, 0x0009 ),
    lut!( 0x02, 0x05, true, 0x03, 0x000a, 0x0046 ),
    lut!( 0x02, 0x05, true, 0x03, 0x000a, 0x0066 ),
    lut!( 0x02, 0x06, true, 0x03, 0x000a, 0x0086 ),
    lut!( 0x02, 0x07, true, 0x03, 0x000a, 0x00c6 ),
    lut!( 0x02, 0x08, true, 0x03, 0x000a, 0x0146 ),
    lut!( 0x02, 0x09, true, 0x03, 0x000a, 0x0246 ),
    lut!( 0x02, 0x0a, true, 0x03, 0x000a, 0x0446 ),
    lut!( 0x02, 0x18, true, 0x03, 0x000a, 0x0846 ),
    lut!( 0x02, 0x05, true, 0x03, 0x000e, 0x0046 ),
    lut!( 0x02, 0x05, true, 0x03, 0x000e, 0x0066 ),
    lut!( 0x02, 0x06, true, 0x03, 0x000e, 0x0086 ),
    lut!( 0x02, 0x07, true, 0x03, 0x000e, 0x00c6 ),
    lut!( 0x02, 0x08, true, 0x03, 0x000e, 0x0146 ),
    lut!( 0x02, 0x09, true, 0x03, 0x000e, 0x0246 ),
    lut!( 0x02, 0x0a, true, 0x03, 0x000e, 0x0446 ),
    lut!( 0x02, 0x18, true, 0x03, 0x000e, 0x0846 ),
    lut!( 0x03, 0x05, true, 0x03, 0x0012, 0x0046 ),
    lut!( 0x03, 0x05, true, 0x03, 0x0012, 0x0066 ),
    lut!( 0x03, 0x06, true, 0x03, 0x0012, 0x0086 ),
    lut!( 0x03, 0x07, true, 0x03, 0x0012, 0x00c6 ),
    lut!( 0x03, 0x08, true, 0x03, 0x0012, 0x0146 ),
    lut!( 0x03, 0x09, true, 0x03, 0x0012, 0x0246 ),
    lut!( 0x03, 0x0a, true, 0x03, 0x0012, 0x0446 ),
    lut!( 0x03, 0x18, true, 0x03, 0x0012, 0x0846 ),
    lut!( 0x03, 0x05, true, 0x03, 0x001a, 0x0046 ),
    lut!( 0x03, 0x05, true, 0x03, 0x001a, 0x0066 ),
    lut!( 0x03, 0x06, true, 0x03, 0x001a, 0x0086 ),
    lut!( 0x03, 0x07, true, 0x03, 0x001a, 0x00c6 ),
    lut!( 0x03, 0x08, true, 0x03, 0x001a, 0x0146 ),
    lut!( 0x03, 0x09, true, 0x03, 0x001a, 0x0246 ),
    lut!( 0x03, 0x0a, true, 0x03, 0x001a, 0x0446 ),
    lut!( 0x03, 0x18, true, 0x03, 0x001a, 0x0846 ),
    lut!( 0x04, 0x05, true, 0x03, 0x0022, 0x0046 ),
    lut!( 0x04, 0x05, true, 0x03, 0x0022, 0x0066 ),
    lut!( 0x04, 0x06, true, 0x03, 0x0022, 0x0086 ),
    lut!( 0x04, 0x07, true, 0x03, 0x0022, 0x00c6 ),
    lut!( 0x04, 0x08, true, 0x03, 0x0022, 0x0146 ),
    lut!( 0x04, 0x09, true, 0x03, 0x0022, 0x0246 ),
    lut!( 0x04, 0x0a, true, 0x03, 0x0022, 0x0446 ),
    lut!( 0x04, 0x18, true, 0x03, 0x0022, 0x0846 ),
    lut!( 0x04, 0x05, true, 0x03, 0x0032, 0x0046 ),
    lut!( 0x04, 0x05, true, 0x03, 0x0032, 0x0066 ),
    lut!( 0x04, 0x06, true, 0x03, 0x0032, 0x0086 ),
    lut!( 0x04, 0x07, true, 0x03, 0x0032, 0x00c6 ),
    lut!( 0x04, 0x08, true, 0x03, 0x0032, 0x0146 ),
    lut!( 0x04, 0x09, true, 0x03, 0x0032, 0x0246 ),
    lut!( 0x04, 0x0a, true, 0x03, 0x0032, 0x0446 ),
    lut!( 0x04, 0x18, true, 0x03, 0x0032, 0x0846 ),
    lut!( 0x05, 0x05, true, 0x03, 0x0042, 0x0046 ),
    lut!( 0x05, 0x05, true, 0x03, 0x0042, 0x0066 ),
    lut!( 0x05, 0x06, true, 0x03, 0x0042, 0x0086 ),
    lut!( 0x05, 0x07, true, 0x03, 0x0042, 0x00c6 ),
    lut!( 0x05, 0x08, true, 0x03, 0x0042, 0x0146 ),
    lut!( 0x05, 0x09, true, 0x03, 0x0042, 0x0246 ),
    lut!( 0x05, 0x0a, true, 0x03, 0x0042, 0x0446 ),
    lut!( 0x05, 0x18, true, 0x03, 0x0042, 0x0846 ),
    lut!( 0x05, 0x05, true, 0x03, 0x0062, 0x0046 ),
    lut!( 0x05, 0x05, true, 0x03, 0x0062, 0x0066 ),
    lut!( 0x05, 0x06, true, 0x03, 0x0062, 0x0086 ),
    lut!( 0x05, 0x07, true, 0x03, 0x0062, 0x00c6 ),
    lut!( 0x05, 0x08, true, 0x03, 0x0062, 0x0146 ),
    lut!( 0x05, 0x09, true, 0x03, 0x0062, 0x0246 ),
    lut!( 0x05, 0x0a, true, 0x03, 0x0062, 0x0446 ),
    lut!( 0x05, 0x18, true, 0x03, 0x0062, 0x0846 ),
    lut!( 0x06, 0x01, true, 0x03, 0x0082, 0x000a ),
    lut!( 0x06, 0x01, true, 0x03, 0x0082, 0x000c ),
    lut!( 0x06, 0x02, true, 0x03, 0x0082, 0x000e ),
    lut!( 0x06, 0x02, true, 0x03, 0x0082, 0x0012 ),
    lut!( 0x06, 0x03, true, 0x03, 0x0082, 0x0016 ),
    lut!( 0x06, 0x03, true, 0x03, 0x0082, 0x001e ),
    lut!( 0x06, 0x04, true, 0x03, 0x0082, 0x0026 ),
    lut!( 0x06, 0x04, true, 0x03, 0x0082, 0x0036 ),
    lut!( 0x07, 0x01, true, 0x03, 0x00c2, 0x000a ),
    lut!( 0x07, 0x01, true, 0x03, 0x00c2, 0x000c ),
    lut!( 0x07, 0x02, true, 0x03, 0x00c2, 0x000e ),
    lut!( 0x07, 0x02, true, 0x03, 0x00c2, 0x0012 ),
    lut!( 0x07, 0x03, true, 0x03, 0x00c2, 0x0016 ),
    lut!( 0x07, 0x03, true, 0x03, 0x00c2, 0x001e ),
    lut!( 0x07, 0x04, true, 0x03, 0x00c2, 0x0026 ),
    lut!( 0x07, 0x04, true, 0x03, 0x00c2, 0x0036 ),
    lut!( 0x08, 0x01, true, 0x03, 0x0142, 0x000a ),
    lut!( 0x08, 0x01, true, 0x03, 0x0142, 0x000c ),
    lut!( 0x08, 0x02, true, 0x03, 0x0142, 0x000e ),
    lut!( 0x08, 0x02, true, 0x03, 0x0142, 0x0012 ),
    lut!( 0x08, 0x03, true, 0x03, 0x0142, 0x0016 ),
    lut!( 0x08, 0x03, true, 0x03, 0x0142, 0x001e ),
    lut!( 0x08, 0x04, true, 0x03, 0x0142, 0x0026 ),
    lut!( 0x08, 0x04, true, 0x03, 0x0142, 0x0036 ),
    lut!( 0x09, 0x01, true, 0x03, 0x0242, 0x000a ),
    lut!( 0x09, 0x01, true, 0x03, 0x0242, 0x000c ),
    lut!( 0x09, 0x02, true, 0x03, 0x0242, 0x000e ),
    lut!( 0x09, 0x02, true, 0x03, 0x0242, 0x0012 ),
    lut!( 0x09, 0x03, true, 0x03, 0x0242, 0x0016 ),
    lut!( 0x09, 0x03, true, 0x03, 0x0242, 0x001e ),
    lut!( 0x09, 0x04, true, 0x03, 0x0242, 0x0026 ),
    lut!( 0x09, 0x04, true, 0x03, 0x0242, 0x0036 ),
    lut!( 0x0a, 0x01, true, 0x03, 0x0442, 0x000a ),
    lut!( 0x0a, 0x01, true, 0x03, 0x0442, 0x000c ),
    lut!( 0x0a, 0x02, true, 0x03, 0x0442, 0x000e ),
    lut!( 0x0a, 0x02, true, 0x03, 0x0442, 0x0012 ),
    lut!( 0x0a, 0x03, true, 0x03, 0x0442, 0x0016 ),
    lut!( 0x0a, 0x03, true, 0x03, 0x0442, 0x001e ),
    lut!( 0x0a, 0x04, true, 0x03, 0x0442, 0x0026 ),
    lut!( 0x0a, 0x04, true, 0x03, 0x0442, 0x0036 ),
    lut!( 0x0c, 0x01, true, 0x03, 0x0842, 0x000a ),
    lut!( 0x0c, 0x01, true, 0x03, 0x0842, 0x000c ),
    lut!( 0x0c, 0x02, true, 0x03, 0x0842, 0x000e ),
    lut!( 0x0c, 0x02, true, 0x03, 0x0842, 0x0012 ),
    lut!( 0x0c, 0x03, true, 0x03, 0x0842, 0x0016 ),
    lut!( 0x0c, 0x03, true, 0x03, 0x0842, 0x001e ),
    lut!( 0x0c, 0x04, true, 0x03, 0x0842, 0x0026 ),
    lut!( 0x0c, 0x04, true, 0x03, 0x0842, 0x0036 ),
    lut!( 0x0e, 0x01, true, 0x03, 0x1842, 0x000a ),
    lut!( 0x0e, 0x01, true, 0x03, 0x1842, 0x000c ),
    lut!( 0x0e, 0x02, true, 0x03, 0x1842, 0x000e ),
    lut!( 0x0e, 0x02, true, 0x03, 0x1842, 0x0012 ),
    lut!( 0x0e, 0x03, true, 0x03, 0x1842, 0x0016 ),
    lut!( 0x0e, 0x03, true, 0x03, 0x1842, 0x001e ),
    lut!( 0x0e, 0x04, true, 0x03, 0x1842, 0x0026 ),
    lut!( 0x0e, 0x04, true, 0x03, 0x1842, 0x0036 ),
    lut!( 0x18, 0x01, true, 0x03, 0x5842, 0x000a ),
    lut!( 0x18, 0x01, true, 0x03, 0x5842, 0x000c ),
    lut!( 0x18, 0x02, true, 0x03, 0x5842, 0x000e ),
    lut!( 0x18, 0x02, true, 0x03, 0x5842, 0x0012 ),
    lut!( 0x18, 0x03, true, 0x03, 0x5842, 0x0016 ),
    lut!( 0x18, 0x03, true, 0x03, 0x5842, 0x001e ),
    lut!( 0x18, 0x04, true, 0x03, 0x5842, 0x0026 ),
    lut!( 0x18, 0x04, true, 0x03, 0x5842, 0x0036 ),
    lut!( 0x06, 0x05, true, 0x03, 0x0082, 0x0046 ),
    lut!( 0x06, 0x05, true, 0x03, 0x0082, 0x0066 ),
    lut!( 0x06, 0x06, true, 0x03, 0x0082, 0x0086 ),
    lut!( 0x06, 0x07, true, 0x03, 0x0082, 0x00c6 ),
    lut!( 0x06, 0x08, true, 0x03, 0x0082, 0x0146 ),
    lut!( 0x06, 0x09, true, 0x03, 0x0082, 0x0246 ),
    lut!( 0x06, 0x0a, true, 0x03, 0x0082, 0x0446 ),
    lut!( 0x06, 0x18, true, 0x03, 0x0082, 0x0846 ),
    lut!( 0x07, 0x05, true, 0x03, 0x00c2, 0x0046 ),
    lut!( 0x07, 0x05, true, 0x03, 0x00c2, 0x0066 ),
    lut!( 0x07, 0x06, true, 0x03, 0x00c2, 0x0086 ),
    lut!( 0x07, 0x07, true, 0x03, 0x00c2, 0x00c6 ),
    lut!( 0x07, 0x08, true, 0x03, 0x00c2, 0x0146 ),
    lut!( 0x07, 0x09, true, 0x03, 0x00c2, 0x0246 ),
    lut!( 0x07, 0x0a, true, 0x03, 0x00c2, 0x0446 ),
    lut!( 0x07, 0x18, true, 0x03, 0x00c2, 0x0846 ),
    lut!( 0x08, 0x05, true, 0x03, 0x0142, 0x0046 ),
    lut!( 0x08, 0x05, true, 0x03, 0x0142, 0x0066 ),
    lut!( 0x08, 0x06, true, 0x03, 0x0142, 0x0086 ),
    lut!( 0x08, 0x07, true, 0x03, 0x0142, 0x00c6 ),
    lut!( 0x08, 0x08, true, 0x03, 0x0142, 0x0146 ),
    lut!( 0x08, 0x09, true, 0x03, 0x0142, 0x0246 ),
    lut!( 0x08, 0x0a, true, 0x03, 0x0142, 0x0446 ),
    lut!( 0x08, 0x18, true, 0x03, 0x0142, 0x0846 ),
    lut!( 0x09, 0x05, true, 0x03, 0x0242, 0x0046 ),
    lut!( 0x09, 0x05, true, 0x03, 0x0242, 0x0066 ),
    lut!( 0x09, 0x06, true, 0x03, 0x0242, 0x0086 ),
    lut!( 0x09, 0x07, true, 0x03, 0x0242, 0x00c6 ),
    lut!( 0x09, 0x08, true, 0x03, 0x0242, 0x0146 ),
    lut!( 0x09, 0x09, true, 0x03, 0x0242, 0x0246 ),
    lut!( 0x09, 0x0a, true, 0x03, 0x0242, 0x0446 ),
    lut!( 0x09, 0x18, true, 0x03, 0x0242, 0x0846 ),
    lut!( 0x0a, 0x05, true, 0x03, 0x0442, 0x0046 ),
    lut!( 0x0a, 0x05, true, 0x03, 0x0442, 0x0066 ),
    lut!( 0x0a, 0x06, true, 0x03, 0x0442, 0x0086 ),
    lut!( 0x0a, 0x07, true, 0x03, 0x0442, 0x00c6 ),
    lut!( 0x0a, 0x08, true, 0x03, 0x0442, 0x0146 ),
    lut!( 0x0a, 0x09, true, 0x03, 0x0442, 0x0246 ),
    lut!( 0x0a, 0x0a, true, 0x03, 0x0442, 0x0446 ),
    lut!( 0x0a, 0x18, true, 0x03, 0x0442, 0x0846 ),
    lut!( 0x0c, 0x05, true, 0x03, 0x0842, 0x0046 ),
    lut!( 0x0c, 0x05, true, 0x03, 0x0842, 0x0066 ),
    lut!( 0x0c, 0x06, true, 0x03, 0x0842, 0x0086 ),
    lut!( 0x0c, 0x07, true, 0x03, 0x0842, 0x00c6 ),
    lut!( 0x0c, 0x08, true, 0x03, 0x0842, 0x0146 ),
    lut!( 0x0c, 0x09, true, 0x03, 0x0842, 0x0246 ),
    lut!( 0x0c, 0x0a, true, 0x03, 0x0842, 0x0446 ),
    lut!( 0x0c, 0x18, true, 0x03, 0x0842, 0x0846 ),
    lut!( 0x0e, 0x05, true, 0x03, 0x1842, 0x0046 ),
    lut!( 0x0e, 0x05, true, 0x03, 0x1842, 0x0066 ),
    lut!( 0x0e, 0x06, true, 0x03, 0x1842, 0x0086 ),
    lut!( 0x0e, 0x07, true, 0x03, 0x1842, 0x00c6 ),
    lut!( 0x0e, 0x08, true, 0x03, 0x1842, 0x0146 ),
    lut!( 0x0e, 0x09, true, 0x03, 0x1842, 0x0246 ),
    lut!( 0x0e, 0x0a, true, 0x03, 0x1842, 0x0446 ),
    lut!( 0x0e, 0x18, true, 0x03, 0x1842, 0x0846 ),
    lut!( 0x18, 0x05, true, 0x03, 0x5842, 0x0046 ),
    lut!( 0x18, 0x05, true, 0x03, 0x5842, 0x0066 ),
    lut!( 0x18, 0x06, true, 0x03, 0x5842, 0x0086 ),
    lut!( 0x18, 0x07, true, 0x03, 0x5842, 0x00c6 ),
    lut!( 0x18, 0x08, true, 0x03, 0x5842, 0x0146 ),
    lut!( 0x18, 0x09, true, 0x03, 0x5842, 0x0246 ),
    lut!( 0x18, 0x0a, true, 0x03, 0x5842, 0x0446 ),
    lut!( 0x18, 0x18, true, 0x03, 0x5842, 0x0846 )
];