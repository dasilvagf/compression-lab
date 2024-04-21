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

// Handle warnings I don't care about
#![allow(unused)] 
#![allow(non_camel_case_types)] 

use std::env;
use std::path;
use std::error::Error;

mod compression_methods;
mod utils;

fn main () -> Result<(), Box<dyn Error>>
{
    //
    //              - - Generate Test Files -- 
    //
 
    /*
    // Get filepath for current directory
    let mut filepath_random : path::PathBuf = env::current_dir()?;
    let mut filepath_rle : path::PathBuf = env::current_dir()?;

    filepath_random.push("random_blob.bin");
    assert_eq!(utils::gen_random_blob(filepath_random, 1000000 /*1MB*/), true);
    filepath_rle.push("rle_blob.bin");
    assert_eq!(utils::gen_rle_blob(filepath_rle, 1000000 /*1MB*/, 10), true);
    */

    //
    //              0 - Open original file and compress it 
    // 
    let mut filepath_rle : path::PathBuf = env::current_dir()?;
    filepath_rle.push("render_test_24bits.bmp");

    let original_data : Vec<u8> = utils::open_bmp(filepath_rle)?;
    let compressed_data : compression_methods::rle::rle_table = 
        compression_methods::rle::compress(&original_data);

    //
    //              1 - Uncompress file and test for equality (loss-less)
    // 
    let uncompressed_data : Vec<u8> = compression_methods::rle::decompress(&compressed_data);
    //assert_eq!(utils::diff_buffers(uncompressed_data, original_data), true);
    
    // Yep, we've done it!
    Ok(())
}
