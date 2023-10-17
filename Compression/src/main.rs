// like importing a 
extern crate flate2;

use flate2::write::GzEncoder;
use flate2::Compression;
use std::fs::File;
use std::io::copy;
use std::io::BufReader;
use std::io::stdin;
use std::time::Instant; // used to record the time taken to compress/decompress file

fn main(){
    // parse arguments to input path and output path
    println!("Please enter the input file: ");
    let mut input_path=String::new();
    stdin().read_line(&mut input_path).unwrap();

    let mut input_file=BufReader::new(File::open(input_path).unwrap());

    println!("Please enter the output file: ");
    let mut output_path=String::new();
    stdin().read_line(&mut output_path).unwrap();

    let output_file=File::create(output_path).unwrap();
    // create a new encoder
    let mut encoder = GzEncoder::new(&output_file, Compression::default());
    // timing
    let start=Instant::now();
    // pass input file to encoder
    copy(&mut input_file, &mut encoder).unwrap();
    // formatted print
    println!(
        "Source len: {:?}",
        input_file.get_ref().metadata().unwrap().len()
    );
    println!("Target len: {:?}", (&output_file).metadata().unwrap().len());
    // print the time elapsed
    println!("Elapsed: {:?}", start.elapsed());

}

