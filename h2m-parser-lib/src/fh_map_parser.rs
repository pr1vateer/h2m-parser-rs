use std::fs::read;
use std::{fs::File};
use std::io::{self, BufReader, Read};
use byteorder::{BigEndian, ReadBytesExt};
use num_enum::TryFromPrimitive;
use strum_macros::Display;

#[derive(Display, TryFromPrimitive)]
#[repr(u8)]
pub enum Difficulty {
    Easy = 0,
    Normal = 1,
    Hard = 2,
    Expert = 3
}

#[derive(Display, TryFromPrimitive, Clone)]
#[repr(u8)]
pub enum Race {
    None = 0,
    Knight = 1,
    Barbarian = 2,
    Sorceress = 4,
    Warlock = 8,
    Wizard = 16,
    Necromancer = 32,
    Multiple = 64,
    Random = 128,
    // Knight | Barbarian | Sorceress | Warlock | Wizard | Necromancer
    All = 63
}

#[derive(Display, TryFromPrimitive)]
#[repr(u8)]
pub enum VictoryCondition {
    DefeatEveryone = 0,
    CaptureTown = 1,
    KillHero = 2,
    ObtainArtifact = 3,
    DefeatOtherSide = 4,
    CollectEnoughGold = 5
}

#[derive(Display, TryFromPrimitive)]
#[repr(u8)]
pub enum LossCondition {
    LoseEverything = 0,
    LoseTown = 1,
    LoseHero = 2,
    OutOfTime = 3
}

pub struct RessurectionMapInfo {
    pub width: u32,
    pub height: u32,
    pub version: u16,
    pub is_campaign: bool,
    pub difficulty: Difficulty,
    available_player_colors: u8,
    pub players: Vec<Race>,
    pub victory_condition: VictoryCondition,
    pub is_victory_condition_applicable_for_ai: bool,
    pub allow_normal_victory: bool,
    pub victory_conditions_metadata: Vec<u32>,
    pub loss_condition: LossCondition,
    pub loss_conditions_metadata: Vec<u32>,
    pub map_language: u8,
    pub name: String,
}

impl RessurectionMapInfo {
    const MIN_MAP_SIZE_BYTES: u64 = 512;

    pub fn load_from_file(file_path: &str) -> io::Result<Self> {
        let file = File::open(file_path)?;
        let len = file.metadata()?.len();
        let mut reader = BufReader::new(file);
        let mut header_buf = [0u8; 6];

        if len < Self::MIN_MAP_SIZE_BYTES {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "File is too small"));
        }

        reader.read_exact(&mut header_buf)?;
        if header_buf != *b"h2map\0" {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "Not a valid fh2m map file"));
        }

        let version = reader.read_u16::<BigEndian>()?;
        let is_campaign = reader.read_u8()? != 0;
        let difficulty = reader.read_u8()?;
        let available_player_colors = reader.read_u8()?;
        let human_player_colors = reader.read_u8()?;
        let computer_player_colors = reader.read_u8()?;
        let alliances_count = reader.read_u32::<BigEndian>()?;
        println!("alliances: {}", alliances_count);
        let mut alliances: Vec<u8> = [].to_vec();
        for _ in 0..alliances_count {
            let a = reader.read_u8()?;
            alliances.push(a);
        }
        let players_count = reader.read_u32::<BigEndian>()?;

        let mut players : Vec<Race> = [].to_vec();
        for _ in 0..players_count {
            let p = reader.read_u8()?;
            players.push(Race::try_from(p).unwrap());
        }
        let victory_condition = VictoryCondition::try_from(reader.read_u8()?).unwrap();
        let victory_condition_also_ai = reader.read_u8()? != 0;
        let allow_normal = reader.read_u8()? != 0;
        let victory_meta_count = reader.read_u32::<BigEndian>()?;
        let mut vic_meta_arr :Vec<u32> = [].to_vec();
        for _ in 0..victory_meta_count {
            vic_meta_arr.push(reader.read_u32::<BigEndian>()?);
        }
        let loss_condition = LossCondition::try_from(reader.read_u8()?).unwrap();
        let loss_meta_count = reader.read_u32::<BigEndian>()?;
        let mut loss_meta_arr :Vec<u32> = [].to_vec();
        for _ in 0..loss_meta_count {
            loss_meta_arr.push(reader.read_u32::<BigEndian>()?);
        }
        let width = reader.read_u32::<BigEndian>()?;
        if width < 1 {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid map witdth!"));
        }
        
        let main_language = reader.read_u8()?;
        let name_len = reader.read_u32::<BigEndian>()?;

        let mut name_buf = vec![0u8; name_len as usize];
        //TODO: handle errors
        _ = reader.read_exact(&mut name_buf);
        let n = String::from_utf8(name_buf).unwrap();

        return Ok(RessurectionMapInfo{ 
            width: width, 
            height: width, //all maps are square
            version: version,
            is_campaign: is_campaign,
            difficulty: Difficulty::try_from(difficulty).unwrap(),
            available_player_colors: available_player_colors,
            players: players,
            victory_condition: victory_condition,
            is_victory_condition_applicable_for_ai: victory_condition_also_ai,
            allow_normal_victory: allow_normal,
            victory_conditions_metadata: vic_meta_arr,
            loss_condition: loss_condition,
            loss_conditions_metadata: loss_meta_arr,
            map_language: main_language,
            name: n
        });
    }
}