use log::debug;
// use regex::Regex;
use reqwest::blocking::Client;
use serde_json::Value;
use std::collections::HashMap;

use crate::item::Item;

mod item_label;

fn first_letter_to_uppper_case(s1: String) -> String {
    let mut c = s1.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().to_string() + c.as_str(),
    }
}

fn get_id(item_json: &Value) -> Option<String> {
    let id = &item_json["id"].as_str()?;
    Some(id.to_string())
}

fn get_wikipedia_title(item_json: &Value) -> Option<String> {
    let wikipedia_title = &item_json["sitelinks"]["enwiki"].as_str()?;
    Some(wikipedia_title.to_string())
}

fn get_population(item_json: &Value) -> Option<u64> {
    let population = &item_json["claims"]["P1082"][0];
    population.as_u64()
}

fn get_description(item_json: &Value) -> Option<String> {
    let description = item_json["descriptions"].get("en")?;
    let description = first_letter_to_uppper_case(description.as_str().unwrap().to_string());
    Some(description)
}

// fn ok_description(description: &str) -> bool {
//     let description_blocklist = [
//         // Space objects
//         r"galaxy",
//         r"constellation",
//         r"star",
//         r"planet",
//         r"nebula",
//         r"moon",
//         r"supernova",
//         r"asteroid",
//         r"cluster",
//         r"natural satellite",
//         // Chemicals
//         r"compound",
//         r"element",
//         // Locations
//         r"region",
//         r"state",
//         // r"country",
//         r"capital",
//         r"borough",
//         r"community",
//         r"department",
//         r"province",
//         r"county",
//         r"city",
//         r"town",
//         r"commune",
//         r"federal subject",
//         // Niches
//         r"football",
//         r"basketball",
//         r"baseball",
//         r"esportiva",
//         r"sport",
//         r"team",
//         // Datetimes
//         r"decade",
//         r"domain",
//         // Animals
//         r"species",
//     ];

//     for re in description_blocklist.iter() {
//         if Regex::new(re)
//             .unwrap()
//             .is_match(&description.to_lowercase())
//         {
//             debug!("Is in description blocklist");
//             return false;
//         }
//     }

//     return true;
// }

fn get_instance_of(
    item_json: &Value,
    id_label_map: &mut HashMap<String, String>,
    client: &Client,
) -> Option<Vec<String>> {
    return item_json["claims"]["P31"].as_array().map(|ids| ids.iter().filter_map(|id| return item_label::get(id.as_str().unwrap(), id_label_map, client))
                .collect());
}

fn ok_instance_of(instance_of: &[String]) -> bool {
    if instance_of.to_owned().contains(&String::from("taxon")) {
        debug!("Ignore taxon instances");
        return false;
    }

    true
}

// fn get_num_sitelinks(item_json: &Value) -> Option<usize> {
//     let num_sitelinks = item_json["sitelinks"].as_object()?;
//     return Some(num_sitelinks.keys().len());
// }

// fn enough_sitelinks(num_sitelinks: usize) -> bool {
//     if num_sitelinks < 15 {
//         debug!("Not enough sitelinks");
//         return false;
//     }

//     return true;
// }

pub fn get_image(item_json: &Value) -> Option<String> {
    let image = &item_json["claims"]["P18"][0];
    return Some(image.as_str()?.to_string());
}

pub fn get_label(item_json: &Value) -> Option<String> {
    let label = &item_json["labels"]["en"];
    return Some(label.as_str()?.to_string());
}

pub fn process_item_json(
    item_json: &str,
    // date_props: &HashMap<&str, &str>,
    id_label_map: &mut HashMap<String, String>,
    client: &Client,
) -> Option<Item> {
    let item_json: Value = serde_json::from_str(item_json).unwrap();

    let population = get_population(&item_json).unwrap_or_default();

    let description = get_description(&item_json)?;

    // if !ok_description(&description) {
    //     println!("bad description");
    //     return None;
    // }

    let id = get_id(&item_json)?;
    let wikipedia_title = get_wikipedia_title(&item_json)?;
    let instance_of = get_instance_of(&item_json, id_label_map, client)?;

    if !ok_instance_of(&instance_of) {
        println!("bad instance of");
        return None;
    }
    // let num_sitelinks = get_num_sitelinks(&item_json)?;

    // if !enough_sitelinks(num_sitelinks) {
    //     println!("not enough sitelinks");
    //     return None;
    // }

    // let page_views = page_views::get(&wikipedia_title, client)?;

    // let Wikipedia { image, label } = wikipedia::get(&wikipedia_title, client)?;
    let image = get_image(&item_json)?;

    let label = get_label(&item_json)?;

    Some(Item {
        description,
        id,
        image,
        instance_of,
        label,
        // page_views,
        wikipedia_title,
        population,
    })
}
