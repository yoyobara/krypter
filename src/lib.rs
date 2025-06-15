mod png_serialize;

use std::{
    error::Error, io::{BufRead, Read, Seek, Write}
};

use png_serialize::{data_to_png, png_to_data};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
struct ChunkData {
    uuid: Uuid,
    chunk_index: usize,
    part: Vec<u8>
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

        let chunk_data = ChunkData {uuid, chunk_index, part: buffer[..bytes_read].to_vec()};
        data_to_png(chunk_data, &mut to(chunk_index))?;
        chunk_index += 1;
    }

    Ok(())
}

pub fn krypt_decrypt<R, W>(readers: Vec<R>, mut to: W) -> Result<(), Box<dyn Error>> where R: BufRead + Seek, W: Write {
    let mut chunk_index = 0usize;
    let mut uuid: Option<Uuid> = None;

    for reader in readers {
        let chunk_data: ChunkData = png_to_data(reader)?;

        match uuid {
            Some(expected) => {
                assert_eq!(chunk_data.uuid, expected);
            },
            None => {
                uuid = Some(chunk_data.uuid);
            }
        }
        assert_eq!(chunk_data.chunk_index, chunk_index);

        to.write_all(&chunk_data.part)?;

        chunk_index += 1;
    }

    Ok(())
}
