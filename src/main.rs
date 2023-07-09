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
    let current_dir = env::current_dir();
    assert_eq!(current_dir.is_ok(), true);

    // Set filename to random_blob.bin
    let mut filepath : path::PathBuf = current_dir.unwrap();
    filepath.push("random_blob.bin");

    assert_eq!(utils::gen_random_blob(filepath, 1000000 /*1MB*/), true);
}
