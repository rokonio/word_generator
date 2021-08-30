//! This module contains a list of lists of word of every word of a
//! language. See the Licences section of the main module for more information
//! about the licence of each lists

/// Every word of the french language with one word per line
pub const FR_TXT: &'static [u8] = _FR_TXT;

const _FR_TXT: &'static [u8] = include_bytes!("Fr.txt");
