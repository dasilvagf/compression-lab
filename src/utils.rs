/*
    ======================================================================
                                COMPRESSION LAB
    
    Compilation of loss-less compression algorithms written in Rust for 
    fun and learning propourses

    Copyright (C) <2023>  <Gabriel da Silva>

    This program is free software: you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with this program.  If not, see <https://www.gnu.org/licenses/>.
    =======================================================================
 */
use std::io::prelude::*;
use std::vec;
use std::path;
use std::fs::File;
use rand::prelude::*;
use byteorder::{ByteOrder, LittleEndian};

// Generate a binary blob with just # different 4 bytes symbols
// randomly placed in the file, so it works as a good test bed
// for files in which RLE is a good compressor!
pub fn gen_blob_rle(filepath : path::PathBuf, filesize_bytes : u32, 
                    symbols_count : u32 /*how many different symbols the file has*/) -> bool
{
    // Gen symbol table

    return false;
}

// Generate a file using random 4 byte symbols
pub fn gen_random_blob(filepath : path::PathBuf, filesize_bytes : u32) -> bool
{
    // open random file for writing
    let mut random_file = std::fs::File::create(filepath).unwrap();

    // write random data
    let n_symbols : u32 = filesize_bytes / 4;
    for s in 0..n_symbols {
        // generate random 32bit integer

        // write 32bit intiger into 4 8 bit integer array
        //https://zetok.github.io/tox/byteorder/trait.ByteOrder.html#tymethod.write_u32_into
        let test_data : [u8; 4] = [0x0, 0x0, 0x0, 0x0];

        // write to file
        random_file.write(&test_data);
    }

    return true;
}

pub fn bin_diff(file1 : path::PathBuf, file2 : path::PathBuf) -> bool
{
    return false;
}
