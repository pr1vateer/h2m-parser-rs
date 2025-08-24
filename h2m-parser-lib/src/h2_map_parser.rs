use std::{fs::File};
use std::io::{self, BufReader, Read, Seek};
use byteorder::{BigEndian, LittleEndian, ReadBytesExt};

//Legacy Heroes2 format (original game)
pub struct OriginalMapInfo {
    pub width: u32,
    pub height: u32,
}

impl OriginalMapInfo {
    const MP2_MAP_INFO_SIZE: u64 = 428;

    pub fn load_from_file(file_path: &str) -> io::Result<Self> {
        let file = File::open(file_path)?;
        let len = file.metadata()?.len();
        let mut reader = BufReader::new(file);

        let signature = reader.read_u32::<BigEndian>()?;
        if signature != 0x5C000000 {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "Not a valid h2m map file"));
        }
        
        println!("{}", len);

        if len < Self::MP2_MAP_INFO_SIZE {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "File is too small"));
        }

        reader.seek(io::SeekFrom::Start(Self::MP2_MAP_INFO_SIZE - 2*4))?;
        let width = reader.read_u32::<LittleEndian>()?;
        let height = reader.read_u32::<LittleEndian>()?;

        return Ok(OriginalMapInfo { width: width, height: height });
    }
}