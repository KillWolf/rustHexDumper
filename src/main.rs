use std::env;
use std::fs::read;

fn main() {
    let read_file: Vec<u8> = read(env::args().nth(1).expect("Please provide a file path"))
        .expect("Failed to read the file");

    for (i, chunk) in read_file.chunks(16).enumerate() {
        print!("{:08x}  ", i * 16);

        for byte in chunk {
            print!("{:02x} ", byte);
        }

        for _ in 0..(16 - chunk.len()) {
            print!("   ");
        }

        print!(" |");
        for byte in chunk {
            if (0x20..=0x7e).contains(byte) {
                print!("{}", *byte as char);
            } else {
                print!(".");
            }
        }
        print!("|");

        println!();
    }
}
