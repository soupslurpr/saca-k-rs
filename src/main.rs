use std::io::Write;
use std::io::Read;
use saca_k_rs::saca_k;
use std::env;
use std::fs::File;
use std::io::{self, BufReader, BufWriter};
use std::process;
use std::time::Instant;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        eprintln!("Usage: {} <input file> <output file>", args[0]);
        process::exit(1);
    }

    let input_path = &args[1];
    let output_path = &args[2];

    // Open the input file and create a buffered reader
    let input_file = File::open(input_path)?;
    let metadata = input_file.metadata()?;
    let size = metadata.len();
    println!("Input file size {}", size);

    let mut reader = BufReader::new(input_file);
    let mut T = vec![0u8; (size + 1).try_into().unwrap()];
    reader.read_exact(&mut T[..size.try_into().unwrap()])?;

    // Initialize suffix array
    let mut sa = vec![0u32; (size + 1).try_into().unwrap()];

    // Measure time for constructing suffix array
    let start = Instant::now();
    saca_k(
        &mut T,
        &mut sa,
        (size + 1).try_into().unwrap(),
        128,
        (size + 1).try_into().unwrap(),
        0,
    );
    let duration = start.elapsed();
    println!("Time needed for constructing suffix array: {:?}", duration);

    // Writing suffix array to output file
    println!("Writing suffix array to output file...");
    let output_file = File::create(output_path)?;
    let mut writer = BufWriter::new(output_file);
    for &elem in &sa[1..] {
        writer.write_all(&elem.to_le_bytes())?;
    }

    Ok(())
}
