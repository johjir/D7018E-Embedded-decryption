#![no_std]

extern crate cortex_m_semihosting;
extern crate stm32f40x; // heads up! use `stm32f40x` for the NUCLEO-401RE
extern crate cortex_m;
use core::fmt::Write;

use cortex_m::peripheral::DWT;
use cortex_m_semihosting::hio;

//use std::u32;
//use std::char;
use core::u32;
use core::char;

fn main() {
    
    unsafe {
        (*DWT.get()).enable_cycle_counter();
        (*DWT.get()).cyccnt.write(0);
    }

    // get a handle to the *host* standard output
    let mut stdout = hio::hstdout().unwrap();

    let mut seed: u32 = 0x0e0657c1;     // Decode key

    // Arrays with coded chars	
    let wordarr: [u32; 4] = [0x9fdd9158, 0x85715808, 0xac73323a, 0];  // "abc" when using the example key
    
    let mut bytearr: [u32; 4] = [0; 4];       // Destination for the decoded chars

    decode(&wordarr, &mut bytearr, &mut seed);  // Decodes the array

    for index in bytearr.iter() {               // Prints the decoded chars
        write!(stdout, "{}", char::from_u32(*index).unwrap()).unwrap();
    }
    write!(stdout, "\n").unwrap();
    
    writeln!(stdout, "{}", unsafe{(*DWT.get()).cyccnt.read()}).unwrap();
}


fn codgen(seed: &mut u32) -> u32 {
    let mut seed_mod = *seed;   // Sets up variables for binary zero counting
    let mut n: u32 = 32;

    while seed_mod > 0 {    // Counts the number of zeros in seed. .count_zeros() could be used instead but where is the fun in that!
        n -= 1;
        seed_mod = seed_mod & (seed_mod - 1);
    }

    let x: u32 = (*seed).rotate_right(2);       // Does stuff with the seed
    let y: u32 = (*seed as i32 >> 6) as u32; 

    *seed = x ^ y ^ n;          // Generates the next seed
    *seed ^ 0x464b713e          // Returns a part of a decode key
}

fn decode(wordarr: &[u32], bytearr: &mut [u32], seed: &mut u32) -> u32 {
    
    let x = !codgen(seed);      // Gets a part of a decode key
    let mut r: u32;

    if wordarr[0] == 0 {        // Sets the last bytearr char to null when the wordarr array terminates
        bytearr[0] = 0;
        r = x;                  // And returns a part of the key to the previous char
    }
    else {                      // Calculates the current char depending on the following chars
        let y = decode(&wordarr[1..], &mut bytearr[1..], seed);
        let m: u32 = x.wrapping_sub(y).wrapping_sub(wordarr[0]);
        bytearr[0] = (m & 0x000ff000).wrapping_shr(13);
        r = (!codgen(seed)) + 1;
        r = x.wrapping_add(y).wrapping_add(m).wrapping_add(r).wrapping_add(5); // Returns a part of the decode key for the previous char
    }
    r
}




