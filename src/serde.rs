use std::io::{Read, Write};
use std::io;
use std::fs::File;

// u8
pub fn encode_u8(output: &mut File, d: u8) -> io::Result<()> {
    output.write(&[d])?;
    Ok(())
}

pub fn decode_u8(input: &mut File) -> io::Result<u8> {
    let mut buf = [0; 1];
    input.read(&mut buf)?;
    Ok(buf[0])
}

// u16
pub fn encode_u16(output: &mut File, d: u16) -> io::Result<()> {
    output.write(&d.to_le_bytes())?;
    Ok(())
}

pub fn decode_u16(input: &mut File) -> io::Result<u16> {
    let mut buf = [0; 2];
    input.read(&mut buf)?;
    Ok(u16::from_le_bytes(buf))
}

// u32
pub fn encode_u32(output: &mut File, d: u32) -> io::Result<()> {
    output.write(&d.to_le_bytes())?;
    Ok(())
}

pub fn decode_u32(input: &mut File) -> io::Result<u32> {
    let mut buf = [0; 4];
    input.read(&mut buf)?;
    Ok(u32::from_le_bytes(buf))
}

// u64
pub fn encode_u64(output: &mut File, d: u64) -> io::Result<()> {
    output.write(&d.to_le_bytes())?;
    Ok(())
}

pub fn decode_u64(input: &mut File) -> io::Result<u64> {
    let mut buf = [0; 8];
    input.read(&mut buf)?;
    Ok(u64::from_le_bytes(buf))
}

// usize
pub fn encode_usize(output: &mut File, d: usize) -> io::Result<()> {
    // TODO usize isn't fixed size
    output.write(&d.to_le_bytes())?;
    Ok(())
}

pub fn decode_usize(input: &mut File) -> io::Result<usize> {
    // TODO usize isn't fixed size
    let mut buf = [0; 8];
    input.read(&mut buf)?;
    Ok(usize::from_le_bytes(buf))
}

// Vec<u8>
pub fn encode_vec(output: &mut File, d: &Vec<u8>) -> io::Result<()> {
    encode_usize(output, d.len())?;

    for x in 0 .. d.len() {
        encode_u8(output, d[x])?;
    }

    Ok(())
}

pub fn decode_vec(input: &mut File) -> io::Result<Vec<u8>> {
    let mut v = vec![0; decode_usize(input)?];

    for x in 0 .. v.len() {
        v[x] = decode_u8(input)?;
    }

    Ok(v)
}
