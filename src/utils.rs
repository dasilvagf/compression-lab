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
use std::io::prelude::*;
use std::vec;
use std::path;
use rand::prelude::*;

// NOTE: Both functions in here ignore error handling for the file writing.

// Generate a binary blob with # different 32bit symbols randomly placed in the file, 
// so it works as a good test bed for RLE-based compressors!
pub fn gen_rle_blob(filepath : path::PathBuf, filesize_bytes : u32, symbols_count : u32) -> bool
{
    // Gen random symbol table
    let mut symbols_table : Vec<u32> = vec![];
    for _s in 0..symbols_count {
        symbols_table.push(rand::random::<u32>());
    }

    //
    // Open file for writing
    //
    let mut random_file = std::fs::File::create(filepath).unwrap();

    // Write random data
    let mut rng_seed = rand::thread_rng();
    let n_symbols : u32 = filesize_bytes / 4;

    for _s in 0..n_symbols {
        // Select a symbol from the table randomly and write
        let rand_int = symbols_table[rng_seed.gen_range(0..symbols_count) as usize];
        random_file.write(&rand_int.to_be_bytes()).ok();
    }

    return true;
}

// Generate a complete random file
pub fn gen_random_blob(filepath : path::PathBuf, filesize_bytes : u32) -> bool
{
    //
    // Open file for writing
    //
    let mut random_file = std::fs::File::create(filepath).unwrap();

    // Write random data
    let n_symbols : u32 = filesize_bytes / 4;
    for _s in 0..n_symbols {
        let rand_int = rand::random::<u32>();
        random_file.write(&rand_int.to_be_bytes()).ok();
    }

    return true;
}

pub fn open_file(filepath : path::PathBuf) -> Result<Vec<u8>, io::Error>
{
    let mut buffer : Vec<u8> = vec![];
    
    // Open file for reading
    let mut file = std::fs::File::open(filepath)?;
    file.read_to_end(&mut buffer)?;
    return Ok(buffer);
}

// Diff two binary buffers
pub fn diff_buffers(buffer_0 : Vec<u8>, buffer_1 : Vec<u8>) -> bool
{
    assert!(buffer_0.len() > 0 && buffer_1.len() > 0, "Ops! This shouldn't happen: One of our buffer is NULL!");

    // Equal size means they are equal, as makes no sense to compress a file to its original size
    if buffer_0.len() == buffer_1.len() 
    {
        return true;
    }



    return false
} 
