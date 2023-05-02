use std::collections::HashMap;
use serenity::utils::Color;

pub fn get_element_color(name: &String) -> Color {
    let mut color_map = HashMap::new();
    color_map.insert("Physical", Color::new(9934743));
    color_map.insert("Fire", Color::new(15615805));
    color_map.insert("Ice", Color::new(2527955));
    color_map.insert("Lightning", Color::new(12999390));
    color_map.insert("Wind", Color::new(6410131));
    color_map.insert("Quantum", Color::new(6313929));
    color_map.insert("Imaginary", Color::new(15982903));
    color_map.get(name.as_str()).unwrap_or(&Color::new(0)).to_owned()
}

#[allow(dead_code)]
pub fn get_path_color(name: &String) -> Color {
    let mut color_map = HashMap::new();
    color_map.insert("Abundance", Color::new(6410131));
    color_map.insert("Destruction", Color::new(9934743));
    color_map.insert("Erudition", Color::new(12999390));
    color_map.insert("Harmony", Color::new(2527955));
    color_map.insert("Hunt", Color::new(15615805));
    color_map.insert("Nihility", Color::new(6313929));
    color_map.insert("Preservation", Color::new(15982903));
    color_map.get(name.as_str()).unwrap_or(&Color::new(0)).to_owned()
}