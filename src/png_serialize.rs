use bincode::config;
use image::{ImageBuffer, ImageReader, RgbaImage};
use serde::{de::DeserializeOwned, Serialize};
use std::{
    error::Error,
    io::{BufRead, Seek, Write},
};

fn calculate_image_dimensions(data_size: usize) -> (usize, usize) {
    let pixel_count = data_size as f64 / 4.0;

    let mut w = pixel_count.sqrt().floor();
    let mut h = (pixel_count / w).ceil();

    if w * h < pixel_count {
        w += 1.0;
        h = (pixel_count / w).ceil();
    }

    (w as usize, h as usize)
}

pub fn data_to_png<W, S>(obj: S, output: &mut W) -> Result<(), Box<dyn Error>>
where
    W: Write + Seek,
    S: Serialize,
{
    let mut data = bincode::serde::encode_to_vec(obj, config::standard())?;
    let (width, height) = calculate_image_dimensions(data.len());

    data.resize(width * height * 4, 0);

    let img: RgbaImage = ImageBuffer::from_raw(width as u32, height as u32, data).ok_or("what")?;

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

#[cfg(test)]
mod tests {
    use std::io::Cursor;
    use rand::Rng;

    use super::*;

    macro_rules! generate_test {
        ($num:expr) => {
            #[test]
            fn roundtrip() {
                let mut rng = rand::rng();
                let original = (0..$num).map(|_| rng.random_range(0..=255)).collect::<Vec<u8>>();

                let mut buffer = Cursor::new(Vec::new());

                data_to_png(&original, &mut buffer).unwrap();
                buffer.rewind().unwrap();
                let restored: Vec<u8> = png_to_data(buffer).unwrap();

                assert_eq!(original, restored);
                println!("ok")
            }
        };
    }

    generate_test!(2000 * 2000 * 4);

}
