struct CombatEvent {
    timestamp: String,
    event_type: String,
    name: String,
    source_id: String,
    target_id: String,
    source_raid_flag: String,
    spell_name: String,
    npc_id: String,
    spell_id: i32,
}

pub fn parse_line(line: &str) {
    let parsed: Vec<&str> = line.split(',').collect();
}
