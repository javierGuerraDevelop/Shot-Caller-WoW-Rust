pub struct CombatEvent {
    timestamp: String,
    event_type: String,
    source_name: String,
    source_id: String,
    target_id: String,
    source_raid_flag: String,
    spell_name: String,
    spell_id: i32,
}

impl CombatEvent {
    pub fn new(
        timestamp: String,
        event_type: String,
        source_name: String,
        source_id: String,
        target_id: String,
        source_raid_flag: String,
        spell_name: String,
        spell_id: i32,
    ) -> Self {
        Self {
            timestamp,
            event_type,
            source_name,
            source_id,
            target_id,
            source_raid_flag,
            spell_name,
            spell_id,
        }
    }
}

pub fn parse_line(line: &str) -> Option<CombatEvent> {
    let mut parts = line.split(',');

    let prefix = parts.next()?;
    let source_guid = parts.next()?;
    let source_name = parts.next()?;
    let _source_flags = parts.next()?;
    let source_raid_flags = parts.next()?;
    let dest_guid = parts.next()?;
    let _dest_name = parts.next()?;
    let _dest_flags = parts.next()?;
    let _dest_raid_flags = parts.next()?;
    let spell_id = parts.next()?;
    let spell_name = parts.next()?;

    println!("prefix: {}", prefix);
    println!("source_name: {}", source_name);
    println!("source_guid: {}", source_guid);
    println!("dest_guid: {}", dest_guid);
    println!("source_raid_flags: {}", source_raid_flags);
    println!("spell_name: {}", spell_name);
    println!("spell_id: {}", spell_id);

    Some(CombatEvent::new(
        String::from(prefix),
        String::from(source_name),
        String::from(source_guid),
        String::from(dest_guid),
        String::from(source_raid_flags),
        String::from(spell_name),
        String::from("1"),
        spell_id.parse::<i32>().unwrap_or(0),
    ))
}
