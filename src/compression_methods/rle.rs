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

pub struct rle_table
{
    byte_table : Vec<u8>,
    runlenght_table : Vec<u32>
}

fn get_u32_from_bytes(index : usize, buffer : &Vec<u8>) -> u32
{
    let raw_bytes = [buffer[index as usize], buffer[(index + 1) as usize], buffer[(index + 2) as usize], buffer[(index + 3) as usize]];
 /*   let mut raw_bytes : [u8; 4] = [0x0, 0x0, 0x0, 0x0];
    raw_bytes[0] = buffer[index as usize];
    raw_bytes[1] = buffer[(index + 1) as usize];
    raw_bytes[2] = buffer[(index + 2) as usize];
    raw_bytes[3] = buffer[(index + 3) as usize]; */

    return u32::from_ne_bytes(raw_bytes);
}

// Burrows–Wheeler transform
fn apply_bwt(mut buffer : &Vec<u8>)
{

}

pub fn compress(buffer : &Vec<u8>) -> rle_table
{
    //
    // Apply BWT to the orignal file
    //
    let raw_data : Vec<u8> = buffer.to_vec();
    apply_bwt(&raw_data);

    // We going to represent our compressed data as two linear vectors.
    // One containing how many occurences each bytes has (run_lengths) and
    // another one with the bytes theyselfs (bytes). Each time a byte has a run length count
    // it will be repeated 2 times in the bytes array. So, if a byte is single at the point
    // in the byte array it means it doesn't repeat at the moment in the original file.
    let mut run_lengths : Vec<u32> = Vec::new();
    let mut bytes : Vec<u8> = Vec::new();
 
    // Run through the entire file and compress ...
    //let mut current_byte : u8 = raw_data[0];
    let mut current_count : u32 = 1;
    let mut current_symbol : u32 = get_u32_from_bytes(0, buffer);

    for i in (4 .. (raw_data.len() - 1)).step_by(4){
        let mut data : u32 = get_u32_from_bytes(i, buffer);

        if current_symbol == data
        {   // increment count, we just found the same byte adjancent to our current
            current_count += 1;
        }
        else
        {   // We found a new symbol. Therefore, we update our data tables
            if current_count >= 2
            {
                bytes.extend(current_symbol.to_be_bytes().to_vec());
                bytes.extend(current_symbol.to_be_bytes().to_vec());
                run_lengths.push(current_count);
            }
            else
            {
                bytes.extend(current_symbol.to_be_bytes().to_vec());
            }

            // Update state
            current_symbol = data;
            current_count = 0;
        }
    }

    // Store compressed data 
    let compressed_data : rle_table = 
        rle_table { byte_table : bytes.to_vec(), runlenght_table : run_lengths.to_vec() };

    //
    // Print statical status
    //
    let original_size_bytes : u32 = raw_data.len().try_into().unwrap();
    let compressed_size_bytes : u32 = (bytes.len() + (2 * run_lengths.len())).try_into().unwrap();
 
    println!("RLE Encoding Statistics");
    println!("Original Size (bytes): {}", original_size_bytes);
    println!("Compressed Size (bytes): {}", compressed_size_bytes);
    println!("Compression Ratio: {}%", 100.0*((compressed_size_bytes as f32)/(original_size_bytes as f32)));
    
    return compressed_data;
}

pub fn decompress(buffer : &rle_table) -> Vec<u8>
{
    return buffer.byte_table.to_vec();
}
