pub mod capture;

use byteorder::{LittleEndian, WriteBytesExt};
use std::io::Write;

pub fn convert_to_wav(samples: Vec<f32>, sample_rate: u32) -> Vec<u8> {
    let spec_channels = 1u16;
    let spec_bits_per_sample = 16u16;

    let mut buf = Vec::with_capacity(44 + samples.len() * 2);

    buf.write_all(b"RIFF").unwrap();
    buf.write_u32::<LittleEndian>(36 + (samples.len() * 2) as u32)
        .unwrap();
    buf.write_all(b"WAVEfmt ").unwrap();
    buf.write_u32::<LittleEndian>(16).unwrap();
    buf.write_u16::<LittleEndian>(1).unwrap();
    buf.write_u16::<LittleEndian>(spec_channels).unwrap();
    buf.write_u32::<LittleEndian>(sample_rate).unwrap();
    buf.write_u32::<LittleEndian>(sample_rate * 2).unwrap(); // Byte rate
    buf.write_u16::<LittleEndian>(2).unwrap(); // Block align
    buf.write_u16::<LittleEndian>(spec_bits_per_sample).unwrap();
    buf.write_all(b"data").unwrap();
    buf.write_u32::<LittleEndian>((samples.len() * 2) as u32)
        .unwrap();

    for sample in samples {
        let scaled = (sample.clamp(-1.0, 1.0) * i16::MAX as f32) as i16;
        buf.write_i16::<LittleEndian>(scaled).unwrap();
    }
    buf
}
