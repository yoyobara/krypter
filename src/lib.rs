mod png_serialize;

use std::{
    error::Error, io::{Read, Seek, Write}
};

use png_serialize::data_to_png;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
struct ChunkData<'a> {
    uuid: Uuid,
    chunk_index: usize,
    part: &'a [u8],
}

pub fn krypt_encrypt<R, W, F>(mut data: R, buffer_size: usize, mut to: F) -> Result<(), Box<dyn Error>>
where
    R: Read,
    W: Write + Seek,
    F: FnMut(usize) -> W,
{
    let uuid = Uuid::new_v4();
    let mut chunk_index = 0;
    let mut buffer = vec![0u8; buffer_size];

    loop {
        let bytes_read = data.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }

        let chunk_data = ChunkData {uuid, chunk_index, part: &buffer[..bytes_read]};
        data_to_png(chunk_data, &mut to(chunk_index))?;
        chunk_index += 1;
    }

    Ok(())
}
