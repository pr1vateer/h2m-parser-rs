use std::io;

use h2_map_parser::*;
use fh_map_parser::*;

mod h2_map_parser;
mod fh_map_parser;

pub fn parse_map(file_path: &str) -> io::Result<RessurectionMapInfo> {
    return RessurectionMapInfo::load_from_file(file_path);
}

pub fn parse_original_map(file_path: &str) -> io::Result<OriginalMapInfo> {
    return OriginalMapInfo::load_from_file(file_path);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(true, true);
    }
}
