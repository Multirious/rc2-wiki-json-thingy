use std::{collections::HashSet, fs::File, io::Read, path::PathBuf};

use clap::Parser;

use path_map::PathMap;
use prelude::*;

mod path_map;
mod prelude;
mod test;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// JSON files to tranpose together
    #[arg(short, long, value_delimiter = ' ', num_args = 1..)]
    files: Vec<PathBuf>,
}

// fn tranpose<'a>(input: &'a PathMap, to_not_expanded: &HashSet<String>) -> PathMap<'a> {
//     let mut result = PathMap::new();
//     for (path, value) in input.0.iter() {
//         let mut path = path.clone();
//         let front = path.pop_front().unwrap();

//         let insert_location = 'p: {
//             for (i, path_seg) in path.iter().enumerate() {
//                 if to_not_expanded.contains(*path_seg) {
//                     break 'p i + 1;
//                 }
//             }
//             path.len()
//         };

//         path.insert(insert_location, front);
//         result.0.insert(path, value.clone());
//     }
//     result
// }

fn tranpose<'a>(input: &'a PathMap, do_special_treatment_for: &HashSet<String>) -> PathMap<'a> {
    let mut result = PathMap::new();
    for (path, value) in input.0.iter() {
        let mut path = path.clone();
        println!("before {path:?}");
        let front = path.pop_front().unwrap();

        let insert_location = 'p: {
            if path.len() > 1 {
                let tail_table = path[path.len() - 2];
                if do_special_treatment_for.contains(tail_table) {
                    println!("special");
                    break 'p path.len();
                }
            } else {
                println!("no table");
                break 'p path.len();
            }
            path.len() - 1
        };

        path.insert(insert_location, front);
        println!("after {path:?}");
        println!();
        result.0.insert(path, value.clone());
    }
    result
}

fn main() -> Result<()> {
    let args = Args::parse();
    let files = args.files;
    let mut jsons = vec![];
    let mut input_path_map_merged = PathMap::new();
    for file_path in files.iter() {
        let mut file = File::open(file_path)
            .with_context(|| format!("Trying to open file {}", file_path.display()))?;
        let mut content = String::new();
        file.read_to_string(&mut content)
            .with_context(|| format!("Trying to read file {}", file_path.display()))?;

        let json: Json = serde_json::from_str(&content)
            .with_context(|| format!("Trying to parse JSON from file {}", file_path.display()))?;
        jsons.push(json);
    }
    for json in jsons.iter() {
        let path_map = PathMap::from_json(json);
        input_path_map_merged.0.extend(path_map.0);
    }
    let do_special_treatment_for = HashSet::from_iter(
        [
            "Data",
            "WeaponData",
            "TyreData",
            "ConnectableBlockCategoryPerFace",
        ]
        .iter()
        .map(|s| s.to_string()),
    );
    let input_tranposed_path_map = tranpose(&input_path_map_merged, &do_special_treatment_for);
    let input_tranposed = input_tranposed_path_map.to_json();
    let to_print = serde_json::to_string_pretty(&input_tranposed)
        .context("Trying to convert JSON to string")?;
    println!("{to_print}");
    Ok(())
}
