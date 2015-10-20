//! Copyright 2015 Peter Bertok. All Rights Reserved.
//! 
//! Licensed under the Apache License, Version 2.0 (the "License");
//! you may not use this file except in compliance with the License.
//! You may obtain a copy of the License at
//! 
//! http://www.apache.org/licenses/LICENSE-2.0
//! 
//! Unless required by applicable law or agreed to in writing, software
//! distributed under the License is distributed on an "AS IS" BASIS,
//! WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//! See the License for the specific language governing permissions and
//! limitations under the License.

mod context;            // complete
pub mod transforms;     // complete & tested
pub mod bitreader;      // complete
mod dictionary;         // complete
mod prefix;             // complete
mod huffman;
pub mod decoder;

use std::error::Error;
use std::io::Error as IoError;
use std::fmt::{ Formatter, Display };
use std::result;

#[derive(Debug)]
pub enum BrotliError {
    /// Returned if the input is not a valid Brotli data stream.
    InvalidEncoding,

    /// Returned if the stream appears to be truncated.
    InsufficientData,

    /// Input/Output error during read
    Io( IoError )
}

pub type Result<T> = result::Result<T, BrotliError>;

impl Error for BrotliError {
    /// A short description of the error.
    fn description(&self) -> &str {
        unimplemented!();
    }

    /// The lower level cause of this error, if any.
    fn cause(&self) -> Option<&Error> { 
        if let BrotliError::Io( ref e ) = *self { Some(e) } else { None }
    }
}

impl Display for BrotliError {
    fn fmt(&self, _f: &mut Formatter) -> std::fmt::Result { 
        unimplemented!();
    }
}

impl From<IoError> for BrotliError {
    fn from(err: IoError) -> BrotliError {
        BrotliError::Io( err )
    }
}
