use io::Write;
use item::process::process_item_json;
use log::info;
use reqwest::blocking::Client;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

extern crate log;
extern crate pretty_env_logger;

mod item;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {
    pretty_env_logger::init();

    let mut count: usize = 0;
    let mut seen_count: usize = 0;

    const IN_FILENAME: &str = "./processed_test.json";
    const OUT_FILENAME: &str = "items_test2.json";

    // File hosts must exist in current path before this produces output
    let lines = read_lines(IN_FILENAME).unwrap();

    let client = Client::builder().build().unwrap();
    let mut id_label_map: HashMap<String, String> = HashMap::new();

    let total: usize = 47711555;

    let path = Path::new(OUT_FILENAME);
    let display = path.display();

    // Open a file in write-only mode, returns `io::Result<File>`
    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", display, why),
        Ok(file) => file,
    };

    for line in lines {
        seen_count += 1;
        println!("{}", seen_count);
        if let Ok(item_json) = line {
            if let Some(item) = process_item_json(&item_json, &mut id_label_map, &client) {
                count += 1;
                info!(
                    "Count={}  Seen={}  Total={}  Percent={}  ID Map={}",
                    count,
                    seen_count,
                    total,
                    seen_count / total * 100,
                    id_label_map.len(),
                );
                info!("");
                info!("{}", &item.id);
                info!("{}", &item.label);
                info!("{}", &item.description);
                info!("https://commons.wikimedia.org/w/index.php?title=Special:Redirect/file/{}&width=300", urlencoding::encode(&item.image));
                info!(
                    "https://en.wikipedia.org/wiki/{}",
                    item.wikipedia_title.replace(" ", "_")
                );
                info!("instance_of: {}", &item.instance_of.join(","));
                info!("");

                let json = serde_json::to_string(&item).unwrap();

                match file.write(format!("{}\n", json).as_bytes()) {
                    Err(why) => panic!("couldn't write to {}: {}", display, why),
                    Ok(_) => (),
                }
            }
        }
    }

    info!("Total: Count={} Seen={}", count, seen_count);
}
