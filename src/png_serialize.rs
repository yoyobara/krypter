use bincode::config;
use image::{codecs::png::{PngDecoder, PngEncoder}, ColorType, ImageDecoder, ImageEncoder};
use serde::{de::DeserializeOwned, Serialize};
use std::{
    error::Error,
    io::{BufRead, Seek, Write},
};

fn calculate_image_dimensions(data_size: usize) -> (u32, u32) {
    let pixel_count = data_size as f64 / 4.0;

    let mut w = pixel_count.sqrt().floor();
    let mut h = (pixel_count / w).ceil();

    if w * h < pixel_count {
        w += 1.0;
        h = (pixel_count / w).ceil();
    }

    (w as u32, h as u32)
}

pub fn data_to_png<W, S>(obj: S, output: W) -> Result<(), Box<dyn Error>>
where
    W: Write + Seek,
    S: Serialize,
{
    let mut data = bincode::serde::encode_to_vec(obj, config::standard())?;
    let (width, height) = calculate_image_dimensions(data.len());

    data.resize((width * height * 4) as usize, 0);

    let encoder = PngEncoder::new(output);
    encoder.write_image(&data, width, height, image::ExtendedColorType::Rgba8).map_err(Into::into)
}

pub fn png_to_data<D, R>(input: R) -> Result<D, Box<dyn Error>>
where
    D: DeserializeOwned,
    R: BufRead + Seek,
{
    let decoder = PngDecoder::new(input)?;
    if decoder.color_type() != ColorType::Rgba8 {
        return Err("Rgba8 format is expected".into());
    }

    let mut raw = vec![0u8; decoder.total_bytes() as usize];
    decoder.read_image(&mut raw)?;

    bincode::serde::decode_from_slice(&raw, config::standard()).map(|(d, _)| d).map_err(Into::into)
}

