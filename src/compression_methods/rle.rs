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

// Burrows–Wheeler transform
fn apply_bwt(mut buffer : &Vec<u8>)
{

}

pub fn compress(buffer : &Vec<u8>) -> Vec<u8>
{
    // Apply BWT to the orignal file
    let raw_data : Vec<u8> = buffer.to_vec();
    apply_bwt(&raw_data);

    // We going to represent our compressed data as two linear vectors.
    // One containing how many occurences each bytes has (run_lengths) and
    // another one with the bytes theyselfs (bytes). Each time a byte has a run length count
    // it will be repeated 2 times in the bytes array. So, if a byte is single at the point
    // in the byte array it means it doesn't repeat at the moment in the original file.
    let mut run_lengths : Vec<u32>;
    let mut bytes : Vec<u8>;
    
    // Run through the entire file and compress ...
    let mut current_byte : u8 = raw_data[0];
    let mut current_count : u32 = 0;
    for i in 1 .. raw_data.len(){
        current_byte = raw_data[i];
    }

    // Store compressed data as an array of bytes
    let mut compressed_data : Vec<u8> = Vec::new();

    // THIS IS JUST SO I DONT' GET AN ERROR
    compressed_data.push(0);
    return  compressed_data;
}

pub fn decompress(buffer : &Vec<u8>) -> Vec<u8>
{
    return buffer.to_vec();
}
