use std::io::Write;
use std::slice::IterMut;

#[repr(C)]
enum WordTransformType {
    Identity,
    UppercaseFirst,
    UppercaseAll,
    OmitLast(u8 ),
    OmitFirst(u8 )
}

struct Transform {
  prefix : &'static [u8],
  transform : WordTransformType,
  suffix : &'static [u8]
}

fn to_uppercase( word: &mut [u8], first_char_only: bool ) {

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

impl Transform {

    pub fn TransformDictionaryWord<W:Write> ( &self, word: &[u8], mut dst: W ) -> usize {
       
        dst.write_all( self.prefix ).unwrap();

        let mut bytes: usize = word.len();
        match self.transform {
            WordTransformType::Identity => dst.write_all( self.suffix ).unwrap(); 
            WordTransformType::UppercaseFirst => {
                 dst.write_all( self.suffix ).unwrap(); 
                /* TODO to_uppercase( dst, true ); */ 
            }
            WordTransformType::UppercaseAll => { 
                dst.write_all( self.suffix ).unwrap();
                /* TODO to_uppercase( dst, false ); */ 
            }
            WordTransformType::OmitFirst( n ) => { 
                dst.write_all( &word[(n as usize) .. word.len()] ).unwrap(); 
                bytes -= n as usize; 
            }
            WordTransformType::OmitLast( n ) => { 
                dst.write_all( &word[0 .. word.len()-( n as usize)] ).unwrap(); 
                bytes -= n as usize; 
            }
        }
         
        dst.write_all( self.suffix ).unwrap();

        return self.prefix.len() + bytes + self.suffix.len();
    }
}

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
