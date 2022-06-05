use std::{
    env,
    error::Error,
    fs::File,
    io::{BufRead, BufReader}};
use sha1::Digest;

const SHA1_HEX_STRING_LENGTH: usize = 40;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        println!();
        Err(format!("args.len() should equal 3: 'sha1_cracker' <word_list.txt> <sha1>. Current {:?}", args).into())
    } else {
        println!("Args are valid: {:?}", args);
        let hash_to_crack = args[2].trim();

        if hash_to_crack.len() != SHA1_HEX_STRING_LENGTH {
            Err(format!("sha1 '{}' is not valid", hash_to_crack).into())
        } else {
            let word_list = File::open(&args[1].trim())?;
            let reader = BufReader::new(&word_list);
            reader.lines()
                .for_each(|l| {
                    let password_to_test = l.unwrap().trim().to_string();
                    let sha1_to_test = &hex::encode(sha1::Sha1::digest(&password_to_test.as_bytes()));
                    if sha1_to_test == hash_to_crack {
                        println!("AMAZING !!! Found password: '{}'", &password_to_test);
                    }
                });
            Ok(())
        }
    }
}
