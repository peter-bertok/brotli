use std::io::Write;
use std::slice::IterMut;

/// Brotli text transformation functions
pub enum WordTransformType {
    /// The word is copied to the output buffer as-is
    Identity,

    /// The first pseudo-UTF8 character (first 1-3 bytes) of the word are transformed using `to_uppercase()`
    UppercaseFirst,

    /// All pseudo-UTF8 characters of the word are transformed using `to_uppercase()`
    UppercaseAll,

    /// Copy the word, except for the last few bytes
    OmitLast(u8),

    /// Copy the word, except for the first few bytes
    OmitFirst(u8)
}

/// A word transform definition used to generate variants of a source text in the output stream
pub struct Transform {
  prefix : &'static [u8],
  transform : WordTransformType,
  suffix : &'static [u8]
}

impl Transform {

    // SAFETY: Instead of 'panic!()', a preferred option might be to return either the bytes copied or an Error
    /// Utility method to copy an array of bytes. 
    fn copy( src: &[u8], dst: &mut [u8] )
    {
        let mut i = dst.iter_mut();
        for byte in src {
            if let Some(d) = i.next() {
                *d = *byte;
            }
            else
            {
                panic!();
            }
        }
    }

    /// The Brotli format specifies a simplified uppercase function for a pseudo-UTF8 interpretation
    /// of a byte array.
    pub fn to_uppercase( word: &mut [u8], first_char_only: bool ) {

        let mut ch = word.into_iter();

        loop {
            match ch.next() {
                Some(p) => {
                    let c = *p;
                    if c < 192
                    {
                        if c >= 97 && c <= 122 
                        {
                            *p ^= 32;
                        }
                    }
                    else if let Some(p1) = ch.next() 
                    {
                        if c < 224
                        {
                            *p1 ^= 32;
                        }
                        else if let Some(p2) = ch.next()
                        {    
                            *p2 ^= 5;
                        }
                        else
                        {
                            break;
                        }
                    }
                    else
                    {
                        break;
                    }
                },
                None => break
            }

            if first_char_only { break; }
        }
    }

    pub fn transform( &self, src: &[u8], dst: &mut [u8] ) -> usize {
                             
        Transform::copy( self.prefix, dst );
        let mut offs = self.prefix.len();

        match self.transform {
            WordTransformType::Identity =>  Transform::copy( src, &mut dst[offs..] ),
            WordTransformType::UppercaseFirst => {
                Transform::copy( src, &mut dst[offs..] );
                Transform::to_uppercase( &mut dst[offs..offs+src.len()], true );
                offs += src.len();
            }
            WordTransformType::UppercaseAll => { 
                Transform::copy( src, &mut dst[offs..] );
                Transform::to_uppercase( &mut dst[offs..offs+src.len()], false );
                offs += src.len();
            }
            WordTransformType::OmitFirst( n ) => {
                Transform::copy( &src[(n as usize)..], &mut dst[offs..] );
                offs += src.len() - (n as usize);
            }
            WordTransformType::OmitLast( n ) => { 
                 Transform::copy( &src[..src.len()-(n as usize)], &mut dst[offs..] );
                offs += src.len() - (n as usize);
            }
        }
         
        Transform::copy( self.suffix, &mut dst[offs..] );        
        offs + self.suffix.len()
    }
}

#[test()]
fn test_transform()
{
    let word = b"hello";
    let mut dst = &mut [0u8;20];

    let bytes = TRANSFORMS[4].transform( word, dst );
    assert_eq!( &dst[..bytes], b"Hello " );
    assert_eq!( bytes, 6 );

    let bytes = TRANSFORMS[85].transform( word, dst );
    assert_eq!( &dst[..bytes], b" HELLO" );
    assert_eq!( bytes, 6 );

    let bytes = TRANSFORMS[11].transform( word, dst );
    assert_eq!( &dst[..bytes], b"llo" );
    assert_eq!( bytes, 3 );

}

/// The Brotli standard defines these 121 standard transforms to use during decompression.
const TRANSFORMS : [Transform;121] = [
	Transform { prefix: b"",		transform: WordTransformType::Identity,			suffix: b""  },
	Transform { prefix: b"",		transform: WordTransformType::Identity,		    suffix: b" "  },
	Transform { prefix: b" ",		transform: WordTransformType::Identity,		    suffix: b" "  },
	Transform { prefix: b"",		transform: WordTransformType::OmitFirst(1),		suffix: b""  },
	Transform { prefix: b"",		transform: WordTransformType::UppercaseFirst,	suffix: b" "  },
	Transform { prefix: b"",		transform: WordTransformType::Identity,		    suffix: b" the "  },
	Transform { prefix: b" ",		transform: WordTransformType::Identity,		    suffix: b""  },
	Transform { prefix: b"s ",		transform: WordTransformType::Identity,		    suffix: b" "  },
	Transform { prefix: b"",		transform: WordTransformType::Identity,		    suffix: b" of "  },
	Transform { prefix: b"",		transform: WordTransformType::UppercaseFirst,	suffix: b""  },
	Transform { prefix: b"",		transform: WordTransformType::Identity,		    suffix: b" and "  },
	Transform { prefix: b"",		transform: WordTransformType::OmitFirst(2),		suffix: b""  },
	Transform { prefix: b"",		transform: WordTransformType::OmitLast(1),		suffix: b""  },
	Transform { prefix: b", ",		transform: WordTransformType::Identity,		    suffix: b" "  },
	Transform { prefix: b"",		transform: WordTransformType::Identity,		    suffix: b", "  },
	Transform { prefix: b" ",		transform: WordTransformType::UppercaseFirst,	suffix: b" "  },
	Transform { prefix: b"",		transform: WordTransformType::Identity,		    suffix: b" in "  },
	Transform { prefix: b"",		transform: WordTransformType::Identity,		    suffix: b" to "  },
	Transform { prefix: b"e ",		transform: WordTransformType::Identity,		    suffix: b" "  },
	Transform { prefix: b"",		transform: WordTransformType::Identity,		    suffix: b"\""  },
	Transform { prefix: b"",		transform: WordTransformType::Identity,		    suffix: b"."  },
	Transform { prefix: b"",		transform: WordTransformType::Identity,		    suffix: b"\">"  },
	Transform { prefix: b"",		transform: WordTransformType::Identity,		    suffix: b"\n"  },
	Transform { prefix: b"",		transform: WordTransformType::OmitLast(3),		suffix: b""  },
	Transform { prefix: b"",		transform: WordTransformType::Identity,		    suffix: b"]"  },
	Transform { prefix: b"",		transform: WordTransformType::Identity,		    suffix: b" for "  },
	Transform { prefix: b"",		transform: WordTransformType::OmitFirst(3),		suffix: b""  },
	Transform { prefix: b"",		transform: WordTransformType::OmitLast(2),		suffix: b""  },
	Transform { prefix: b"",		transform: WordTransformType::Identity,		    suffix: b" a "  },
	Transform { prefix: b"",		transform: WordTransformType::Identity,		    suffix: b" that "  },
	Transform { prefix: b" ",		transform: WordTransformType::UppercaseFirst,	suffix: b""  },
	Transform { prefix: b"",		transform: WordTransformType::Identity,		    suffix: b". "  },
	Transform { prefix: b".",		transform: WordTransformType::Identity,		    suffix: b""  },
	Transform { prefix: b" ",		transform: WordTransformType::Identity,		    suffix: b", "  },
	Transform { prefix: b"",		transform: WordTransformType::OmitFirst(4),		suffix: b""  },
	Transform { prefix: b"",		transform: WordTransformType::Identity,		    suffix: b" with "  },
	Transform { prefix: b"",		transform: WordTransformType::Identity,		    suffix: b"'"  },
	Transform { prefix: b"",		transform: WordTransformType::Identity,		    suffix: b" from "  },
	Transform { prefix: b"",		transform: WordTransformType::Identity,		    suffix: b" by "  },
	Transform { prefix: b"",		transform: WordTransformType::OmitFirst(5),		suffix: b""  },
	Transform { prefix: b"",		transform: WordTransformType::OmitFirst(6),		suffix: b""  },
	Transform { prefix: b" the ",	transform: WordTransformType::Identity,		    suffix: b""  },
	Transform { prefix: b"",		transform: WordTransformType::OmitLast(4),		suffix: b""  },
	Transform { prefix: b"",		transform: WordTransformType::Identity,		    suffix: b". The "  },
	Transform { prefix: b"",		transform: WordTransformType::UppercaseAll,		suffix: b""  },
	Transform { prefix: b"",		transform: WordTransformType::Identity,		    suffix: b" on "  },
	Transform { prefix: b"",		transform: WordTransformType::Identity,		    suffix: b" as "  },
	Transform { prefix: b"",		transform: WordTransformType::Identity,		    suffix: b" is "  },
	Transform { prefix: b"",		transform: WordTransformType::OmitLast(7),		suffix: b""  },
	Transform { prefix: b"",		transform: WordTransformType::OmitLast(1),		suffix: b"ing "  },
	Transform { prefix: b"",		transform: WordTransformType::Identity,		    suffix: b"\n\t"  },
	Transform { prefix: b"",		transform: WordTransformType::Identity,		    suffix: b":"  },
	Transform { prefix: b" ",		transform: WordTransformType::Identity,		    suffix: b". "  },
	Transform { prefix: b"",		transform: WordTransformType::Identity,		    suffix: b"ed "  },
	Transform { prefix: b"",		transform: WordTransformType::OmitFirst(9),		suffix: b""  },
	Transform { prefix: b"",		transform: WordTransformType::OmitFirst(7),		suffix: b""  },
	Transform { prefix: b"",		transform: WordTransformType::OmitLast(6),		suffix: b""  },
	Transform { prefix: b"",		transform: WordTransformType::Identity,		    suffix: b"("  },
	Transform { prefix: b"",		transform: WordTransformType::UppercaseFirst,	suffix: b", "  },
	Transform { prefix: b"",		transform: WordTransformType::OmitLast(8),		suffix: b""  },
	Transform { prefix: b"",		transform: WordTransformType::Identity,		    suffix: b" at "  },
	Transform { prefix: b"",		transform: WordTransformType::Identity,		    suffix: b"ly "  },
	Transform { prefix: b" the ",	transform: WordTransformType::Identity,		    suffix: b" of "  },
	Transform { prefix: b"",		transform: WordTransformType::OmitLast(5),		suffix: b""  },
	Transform { prefix: b"",		transform: WordTransformType::OmitLast(9),		suffix: b""  },
	Transform { prefix: b" ",		transform: WordTransformType::UppercaseFirst,	suffix: b", "  },
	Transform { prefix: b"",		transform: WordTransformType::UppercaseFirst,	suffix: b"\""  },
	Transform { prefix: b".",		transform: WordTransformType::Identity,		    suffix: b"("  },
	Transform { prefix: b"",		transform: WordTransformType::UppercaseAll,		suffix: b" "  },
	Transform { prefix: b"",		transform: WordTransformType::UppercaseFirst,	suffix: b"\">"  },
	Transform { prefix: b"",		transform: WordTransformType::Identity,		suffix: b"=\""  },
	Transform { prefix: b" ",		transform: WordTransformType::Identity,		suffix: b"."  },
	Transform { prefix: b".com/",	transform: WordTransformType::Identity,		suffix: b""  },
	Transform { prefix: b" the ",	transform: WordTransformType::Identity,		suffix: b" of the "  },
	Transform { prefix: b"",		transform: WordTransformType::UppercaseFirst,		suffix: b"'"  },
	Transform { prefix: b"",		transform: WordTransformType::Identity,		suffix: b". This "  },
	Transform { prefix: b"",		transform: WordTransformType::Identity,		suffix: b","  },
	Transform { prefix: b".",		transform: WordTransformType::Identity,		suffix: b" "  },
	Transform { prefix: b"",		transform: WordTransformType::UppercaseFirst,		suffix: b"("  },
	Transform { prefix: b"",		transform: WordTransformType::UppercaseFirst,		suffix: b"."  },
	Transform { prefix: b"",		transform: WordTransformType::Identity,		suffix: b" not "  },
	Transform { prefix: b" ",		transform: WordTransformType::Identity,		suffix: b"=\""  },
	Transform { prefix: b"",		transform: WordTransformType::Identity,		suffix: b"er "  },
	Transform { prefix: b" ",		transform: WordTransformType::UppercaseAll,		suffix: b" "  },
	Transform { prefix: b"",		transform: WordTransformType::Identity,		suffix: b"al "  },
	Transform { prefix: b" ",		transform: WordTransformType::UppercaseAll,		suffix: b""  },
	Transform { prefix: b"",		transform: WordTransformType::Identity,		suffix: b"='"  },
	Transform { prefix: b"",		transform: WordTransformType::UppercaseAll,		suffix: b"\""  },
	Transform { prefix: b"",		transform: WordTransformType::UppercaseFirst,		suffix: b". "  },
	Transform { prefix: b" ",		transform: WordTransformType::Identity,		suffix: b"("  },
	Transform { prefix: b"",		transform: WordTransformType::Identity,		suffix: b"ful "  },
	Transform { prefix: b" ",		transform: WordTransformType::UppercaseFirst,		suffix: b". "  },
	Transform { prefix: b"",		transform: WordTransformType::Identity,		suffix: b"ive "  },
	Transform { prefix: b"",		transform: WordTransformType::Identity,		suffix: b"less "  },
	Transform { prefix: b"",		transform: WordTransformType::UppercaseAll,		suffix: b"'"  },
	Transform { prefix: b"",		transform: WordTransformType::Identity,		suffix: b"est "  },
	Transform { prefix: b" ",		transform: WordTransformType::UppercaseFirst,		suffix: b"."  },
	Transform { prefix: b"",		transform: WordTransformType::UppercaseAll,		suffix: b"\">"  },
	Transform { prefix: b" ",		transform: WordTransformType::Identity,		suffix: b"='"  },
	Transform { prefix: b"",		transform: WordTransformType::UppercaseFirst,		suffix: b","  },
	Transform { prefix: b"",		transform: WordTransformType::Identity,		suffix: b"ize "  },
	Transform { prefix: b"",		transform: WordTransformType::UppercaseAll,		suffix: b"."  },
	Transform { prefix: b"\xc2\xa0",	transform: WordTransformType::Identity,		suffix: b""  },
	Transform { prefix: b" ",		transform: WordTransformType::Identity,		suffix: b","  },
	Transform { prefix: b"",		transform: WordTransformType::UppercaseFirst,		suffix: b"=\""  },
	Transform { prefix: b"",		transform: WordTransformType::UppercaseAll,		suffix: b"=\""  },
	Transform { prefix: b"",		transform: WordTransformType::Identity,		suffix: b"ous "  },
	Transform { prefix: b"",		transform: WordTransformType::UppercaseAll,		suffix: b", "  },
	Transform { prefix: b"",		transform: WordTransformType::UppercaseFirst,		suffix: b"='"  },
	Transform { prefix: b" ",		transform: WordTransformType::UppercaseFirst,		suffix: b","  },
	Transform { prefix: b" ",		transform: WordTransformType::UppercaseAll,		suffix: b"=\""  },
	Transform { prefix: b" ",		transform: WordTransformType::UppercaseAll,		suffix: b", "  },
	Transform { prefix: b"",		transform: WordTransformType::UppercaseAll,		suffix: b","  },
	Transform { prefix: b"",		transform: WordTransformType::UppercaseAll,		suffix: b"("  },
	Transform { prefix: b"",		transform: WordTransformType::UppercaseAll,		suffix: b". "  },
	Transform { prefix: b" ",		transform: WordTransformType::UppercaseAll,		suffix: b"."  },
	Transform { prefix: b"",		transform: WordTransformType::UppercaseAll,		suffix: b"='"  },
	Transform { prefix: b" ",		transform: WordTransformType::UppercaseAll,		suffix: b". "  },
	Transform { prefix: b" ",		transform: WordTransformType::UppercaseFirst,		suffix: b"=\""  },
	Transform { prefix: b" ",		transform: WordTransformType::UppercaseAll,		suffix: b"='"  },
	Transform { prefix: b" ",		transform: WordTransformType::UppercaseFirst,		suffix: b"='"  }
];
