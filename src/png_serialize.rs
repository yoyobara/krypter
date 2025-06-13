use bincode::config;
use image::{ImageBuffer, ImageReader, RgbaImage};
use serde::{de::DeserializeOwned, Serialize};
use std::{
    error::Error,
    io::{BufRead, Seek, Write},
};

fn calculate_image_dimensions(data_size: usize) -> (usize, usize) {
    let pixel_count = data_size / 4;

    let mut w = (pixel_count as f64).sqrt() as usize;
    let mut h = pixel_count.div_ceil(w);

    if w * h < pixel_count {
        w += 1;
        h = pixel_count.div_ceil(w);
    }

    (w, h)
}

fn pad_with_random_values(mut data: Vec<u8>, desired_length: usize) -> Vec<u8> {
    let bytes_to_pad = desired_length.saturating_sub(data.len());
    data.extend(std::iter::repeat(0).take(bytes_to_pad));

    data
}

pub fn data_to_png<W, S>(obj: S, output: &mut W) -> Result<(), Box<dyn Error>>
where
    W: Write + Seek,
    S: Serialize,
{
    let data = bincode::serde::encode_to_vec(obj, config::standard())?;
    let (width, height) = calculate_image_dimensions(data.len());

    let padded_data = pad_with_random_values(data, width * height);
    let img: RgbaImage =
        ImageBuffer::from_raw(width as u32, height as u32, padded_data).ok_or("what")?;

    img.write_to(output, image::ImageFormat::Png)?;

    Ok(())
}

pub fn png_to_data<'a, D, R>(input: R) -> Result<D, Box<dyn Error>>
where
    D: DeserializeOwned,
    R: 'a + BufRead + Seek,
{
    let image = ImageReader::with_format(input, image::ImageFormat::Png)
        .decode()?
        .into_rgba8();
    let raw = image.into_raw();

    let (obj, _): (D, _) = bincode::serde::decode_from_slice(&raw, config::standard())?;
    Ok(obj)
}
