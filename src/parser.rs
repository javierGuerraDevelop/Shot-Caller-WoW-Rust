pub struct CombatEvent {
    timestamp: String,
    event_type: String,
    source_name: String,
    source_guid: String,
    target_guid: String,
    source_raid_flag: String,
    spell_name: String,
    spell_id: i32,
}

impl CombatEvent {
    pub fn new(
        timestamp: String,
        event_type: String,
        source_name: String,
        source_guid: String,
        target_guid: String,
        source_raid_flag: String,
        spell_name: String,
        spell_id: i32,
    ) -> Self {
        Self {
            timestamp,
            event_type,
            source_name,
            source_guid,
            target_guid,
            source_raid_flag,
            spell_name,
            spell_id,
        }
    }
}

pub fn parse_line(line: &str) -> Option<CombatEvent> {
    let mut parts = line.split(',');

    let prefix = parts.next()?;
    let (timestamp, event_type) = prefix.split_once("  ")?;
    if timestamp.is_empty() || event_type == "" {
        return None;
    }

    let source_guid = parts.next()?;
    if source_guid.is_empty() {
        return None;
    }

    let source_name = parts.next()?;
    if source_name.is_empty() {
        return None;
    }

    let _source_flags = parts.next()?;

    let source_raid_flag = parts.next()?;
    if source_raid_flag.is_empty() {
        return None;
    }

    let target_guid = parts.next()?;
    if target_guid.is_empty() {
        return None;
    }

    let _target_name = parts.next()?;
    let _target_flags = parts.next()?;
    let _target_raid_flags = parts.next()?;

    let spell_id = parts.next()?.parse::<i32>().ok()?;

    let spell_name = parts.next()?;
    if spell_name.is_empty() {
        return None;
    }

    Some(CombatEvent::new(
        String::from(timestamp),
        String::from(event_type),
        String::from(source_name),
        String::from(source_guid),
        String::from(target_guid),
        String::from(source_raid_flag),
        String::from(spell_name),
        spell_id,
    ))
}
