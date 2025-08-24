use std::env;

use h2m_parser_lib;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let res = h2m_parser_lib::parse_map(&file_path);

    match res {
        Ok(map_info) => {
            println!(
                "Map loaded successfully: {}x{}", map_info.width, map_info.height
            );
            println!("Map name: {}", map_info.name);
            println!(
                "Version: {}\nIs campaign: {}", map_info.version, map_info.is_campaign
            );
            println!("Difficulty: {}", map_info.difficulty);
            for i in 0..map_info.players.len() {
                println!("Player {} : {}", i+1, map_info.players[i]);
            }
            println!("Victory condition: {}, applicable for AI: {}", map_info.victory_condition, map_info.is_victory_condition_applicable_for_ai);
            println!("Normal victory allowed: {}", map_info.allow_normal_victory);
            println!("Loss condition: {}", map_info.loss_condition);
        }
        Err(e) => {
            println!("Error loading map: {}", e);
        }
    }
}
