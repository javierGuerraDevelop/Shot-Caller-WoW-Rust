use crate::constants;

pub enum Event {
    Interrupt {
        timestamp: String,
        source_guid: String,
    },
    CrowdControl {
        timestamp: String,
        source_guid: String,
        spell_id: i32,
    },
    Death {
        timestamp: String,
        target_guid: String,
    },
    Resurrection {
        timestamp: String,
        target_guid: String,
    },
    Other {
        timestamp: String,
        source_guid: String,
        target_guid: String,
    },
}

pub fn parse_line(line: &str) -> Option<Event> {
    let mut parts = line.split(',');

    let prefix = parts.next()?;
    let (timestamp, event_type) = prefix
        .split_once("  ")
        .filter(|(ts, ev)| !ts.is_empty() && !ev.is_empty())?;

    if !constants::is_valid_event(event_type) {
        return None;
    }

    let source_guid = parts.next().filter(|s| !s.is_empty())?;
    let _source_name = parts.next().filter(|s| !s.is_empty())?;
    let _source_flags = parts.next()?;
    let source_raid_flag = parts.next().filter(|s| !s.is_empty())?;
    let target_guid = parts.next().filter(|s| !s.is_empty())?;
    let _target_name = parts.next()?;
    let _target_flags = parts.next()?;
    let _target_raid_flags = parts.next()?;

    if event_type == "UNIT_DIED" {
        return Some(Event::Death {
            timestamp: timestamp.to_string(),
            target_guid: target_guid.to_string(),
        });
    } else if event_type == "SPELL_RESURRECT" {
        return Some(Event::Resurrection {
            timestamp: timestamp.to_string(),
            target_guid: target_guid.to_string(),
        });
    }

    let spell_id = parts.next()?.parse::<i32>().ok()?;
    if constants::is_interrupt(spell_id) {
        Some(Event::Interrupt {
            timestamp: timestamp.to_string(),
            source_guid: source_guid.to_string(),
        })
    } else if constants::is_crowd_control(spell_id) {
        Some(Event::CrowdControl {
            timestamp: timestamp.to_string(),
            source_guid: source_guid.to_string(),
            spell_id,
        })
    } else {
        Some(Event::Other {
            timestamp: timestamp.to_string(),
            source_guid: source_guid.to_string(),
            target_guid: target_guid.to_string(),
        })
    }
}
