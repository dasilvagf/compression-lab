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
use std::env;
use std::path;

mod compression_methods;
mod utils;

fn main ()
{
    // Get filepath for current directory
    //
    // NOTE: This is not the right way to use unwrap, I should be checking in case of Result
    // returning an error (what Rust calls panic!), but I'm not going to do this as this is just
    // a test-bed/research project.
    let mut filepath_random : path::PathBuf = env::current_dir().unwrap();
    let mut filepath_rle : path::PathBuf = env::current_dir().unwrap();

    //
    // Test
    //
    filepath_random.push("random_blob.bin");
    assert_eq!(utils::gen_random_blob(filepath_random, 1000000 /*1MB*/), true);
    filepath_rle.push("rle_blob.bin");
    assert_eq!(utils::gen_rle_blob(filepath_rle, 1000000 /*1MB*/, 10), true);
}
