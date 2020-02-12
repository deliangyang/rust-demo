use std::fs;
use std::io::Read;

struct Header {
    magic: u32,
    cpu_type: u32,
    cpu_sub_type: u32,
    file_type: u32,
    ncmds: u32,
    size_of_cmds: u32,
    flags: u32,
}

fn main() {
    let mut file_in = std::fs::File::open("./return").unwrap();
    let mut buffer = [0u8; 4];

    let nbytes = file_in.read(&mut buffer).unwrap();
    println!("{}, {:#?}", nbytes, buffer);
    let item = (buffer[3] as u32) << 0
        | (buffer[2] as u32) << 8
        | (buffer[1] as u32) << 16
        | (buffer[0] as u32) << 24;
    println!("0x{:x}", item);

}
