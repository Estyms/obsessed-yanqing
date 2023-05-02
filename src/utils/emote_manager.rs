use std::collections::HashMap;

pub fn get_element_emote(name: &String) -> &'static str {
    let mut emote_map = HashMap::new();
    emote_map.insert("Physical", "<:ele_physical:1102553011928186901>");
    emote_map.insert("Fire", "<:ele_fire:1102553084523196470>");
    emote_map.insert("Ice", "<:ele_ice:1102553257009750046>");
    emote_map.insert("Lightning", "<:ele_lightning:1102553255692738640>");
    emote_map.insert("Wind", "<:ele_wind:1102553253733998643>");
    emote_map.insert("Quantum", "<:ele_quantum:1102553252278583326>");
    emote_map.insert("Imaginary", "<:ele_imaginary:1102551584057069649>");
    emote_map.get(name.as_str()).unwrap_or(&"").to_owned()
}

pub fn get_path_emote(name: &String) -> &'static str {
    let mut emote_map = HashMap::new();
    emote_map.insert("Abundance", "<:path_abundance:1102554507986088038>");
    emote_map.insert("Destruction", "<:path_destruction:1102554505511448646>");
    emote_map.insert("Erudition", "<:path_erudition:1102554503527530597>");
    emote_map.insert("Harmony","<:path_harmony:1102554501518471249>");
    emote_map.insert("Hunt", "<:path_hunt:1102554500352458812>");
    emote_map.insert("Nihility", "<:path_nihility:1102554499085778964>");
    emote_map.insert("Preservation", "<:path_preservation:1102554496757927968>");
    emote_map.get(name.as_str()).unwrap_or(&"").to_owned()
}