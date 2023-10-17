// like importing a 
extern crate flate2;

use flate2::write::GzEncoder;
use flate2::Compression;
use std::env::args;
use std::fs::File;
use std::io::copy;
use std::io::BufReader;
use std::time::Instant; // used to record the time taken to compress/decompress file

fn main(){
    // check if the program has received enough arguments
    if args().len() !=3 {
        eprintln!("Usage: 'source' 'target'");
        return;
    }
    // parse arguments to input path and output path
    let mut input=BufReader::new(File::open(args().nth(1).unwrap()).unwrap());
    let output=File::create(args().nth(2).unwrap()).unwrap();
    // create a new encoder
    let mut encoder = GzEncoder::new(output, Compression::default());
    // timing
    let start=Instant::now();
    // pass input file to encoder
    copy(&mut input, &mut encoder).unwrap();
    // formatted print
    println!(
        "Source len: {:?}",
        input.get_ref().metadata().unwrap().len()
    );
    println!("Target len: {:?}", output.metadata().unwrap().len());
    // print the time elapsed
    println!("Elapsed: {:?}", start.elapsed());

}

