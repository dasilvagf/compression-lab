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
use std::io;
use std::vec;
use std::fs::File;
use rand::prelude::*;

// Generate a binary blob with just # different 4 bytes symbols
// randomly placed in the file, so it works as a good test bed
// for files in which RLE is a good compressor!
pub fn gen_blob_rle(filepath : &str, filesize_bytes : i32, 
                    symbols_count : i32 /*how many different symbols the file has*/) -> bool
{
    // Gen symbol table

    return false;
}

// Generate a file using random 4 byte symbols
pub fn gen_random_blob(filepath : &str, filesize_bytes : i32) -> bool
{

    return false;
}

pub fn bin_diff(file1 : &str, file2 : &str) -> bool
{
    return false;
}
