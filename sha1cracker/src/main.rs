use sha1::Digest;
use std::{env,error::Error,fs::File,io::{BufRead, BufReader},};

const SHA1_HEX_STRING_LENGTH: usize = 40;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        println!("Usage:");
        println!("sha1_cracker: <wordlist.txt> <sha1_hash>");
    return Ok(());
    }

    let hash = args[2].trim();
    if hash.len() != SHA1_HEX_STRING_LENGTH {
        return Err("sha1 hash is not valid".into());
    }

    let wordlist_file = File::open(&args[1])?;
    let reader = BufReader::new(&wordlist_file);
    
    for line in reader.lines() {
        let line = line?;
        let pass = line.trim();
        if hash == &hex::encode(sha1::Sha1::digest(pass.as_bytes())) {
        println!("Password found: {}", &pass);
    return Ok(());
        }
    }

    println!("password not found in wordlist :(");
    Ok(())
}

