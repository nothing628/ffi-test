pub struct RIFFContainer<'a> {
    frame_id: &'a str,
    subchunks: Vec<Box<dyn Chunk>>
}

pub struct RegularChunk<'a> {
    chunk_id: &'a str,
    chunk_data: Vec<u8>,
}

pub trait Chunk {
    fn get_chunk_id(&self) -> &str;
    fn get_chunk_size(&self) -> usize;
    fn get_chunk_data(&self) -> Option<&Vec<Box<dyn Chunk>>>;
    fn get_chunk_bytes(&self) -> Option<&Vec<u8>>;
    fn to_bytes(&self) -> Vec<u8>;
}

pub trait RiffChunk {
    fn get_chunk_frame_id(&self) -> &str;
}

impl RiffChunk for RIFFContainer<'_> {
    fn get_chunk_frame_id(&self) -> &str {
        self.frame_id
    }
}

impl Chunk for RIFFContainer<'_> {
    fn get_chunk_id(&self) -> &str {
        "RIFF"
    }

    fn get_chunk_data(&self) -> Option<&Vec<Box<dyn Chunk>>> {
        Some(&self.subchunks)
    }

    fn get_chunk_bytes(&self) -> Option<&Vec<u8>> {
        todo!("To be implemented")
    }

    fn get_chunk_size(&self) -> usize {
        todo!("Tobe implemented")
    }

    fn to_bytes(&self) -> Vec<u8> {
        todo!("Tobe implemented")
    }
}

impl Chunk for RegularChunk<'_> {
    fn get_chunk_data(&self) -> Option<&Vec<Box<dyn Chunk>>> {
        None
    }

    fn get_chunk_bytes(&self) -> Option<&Vec<u8>> {
        Some(&self.chunk_data)
    }

    fn get_chunk_id(&self) -> &str {
        &self.chunk_id
    }

    fn get_chunk_size(&self) -> usize {
        self.chunk_data.len()
    }

    fn to_bytes(&self) -> Vec<u8> {
        todo!("To be implemented")
    }
}
