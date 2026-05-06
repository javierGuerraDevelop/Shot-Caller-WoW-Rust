use crate::constants;
use chrono::NaiveDateTime;

pub enum Event {
    Interrupt {
        timestamp_ms: u64,
        source_guid: String,
    },
    CrowdControl {
        timestamp_ms: u64,
        source_guid: String,
        spell_id: i32,
    },
    Death {
        timestamp_ms: u64,
        target_guid: String,
    },
    Resurrection {
        timestamp_ms: u64,
        target_guid: String,
    },
    Other {
        timestamp_ms: u64,
        source_guid: String,
        target_guid: String,
        spell_id: i32,
    },
}

impl Event {
    pub fn timestamp_ms(&self) -> u64 {
        match self {
            Event::Interrupt { timestamp_ms, .. } => *timestamp_ms,
            Event::CrowdControl { timestamp_ms, .. } => *timestamp_ms,
            Event::Death { timestamp_ms, .. } => *timestamp_ms,
            Event::Resurrection { timestamp_ms, .. } => *timestamp_ms,
            Event::Other { timestamp_ms, .. } => *timestamp_ms,
        }
    }
}

fn parse_timestamp(ts: &str) -> u64 {
    let ts = ts.split('-').next().unwrap_or(ts).trim();
    if let Ok(dt) = NaiveDateTime::parse_from_str(ts, "%m/%d/%Y %H:%M:%S%.f") {
        dt.and_utc().timestamp_millis() as u64
    } else {
        0
    }
}

pub fn parse_line(line: &str) -> Option<Event> {
    let mut parts = line.split(',');

    let prefix = parts.next()?;
    let (timestamp_str, event_type) = prefix
        .split_once("  ")
        .filter(|(ts, ev)| !ts.is_empty() && !ev.is_empty())?;

    let timestamp_ms = parse_timestamp(timestamp_str);

    if !constants::is_valid_event(event_type) {
        return None;
    }

    let source_guid = parts.next().filter(|s| !s.is_empty())?;
    let _source_name = parts.next().filter(|s| !s.is_empty())?;
    let _source_flags = parts.next()?;
    let _source_raid_flag = parts.next().filter(|s| !s.is_empty())?;
    let target_guid = parts.next().filter(|s| !s.is_empty())?;
    let _target_name = parts.next()?;
    let _target_flags = parts.next()?;
    let _target_raid_flags = parts.next()?;

    if event_type == "UNIT_DIED" {
        return Some(Event::Death {
            timestamp_ms,
            target_guid: target_guid.to_string(),
        });
    } else if event_type == "SPELL_RESURRECT" {
        return Some(Event::Resurrection {
            timestamp_ms,
            target_guid: target_guid.to_string(),
        });
    }

    let spell_id = parts.next()?.parse::<i32>().ok()?;
    if constants::is_interrupt(spell_id) {
        Some(Event::Interrupt {
            timestamp_ms,
            source_guid: source_guid.to_string(),
        })
    } else if constants::is_crowd_control(spell_id) {
        Some(Event::CrowdControl {
            timestamp_ms,
            source_guid: source_guid.to_string(),
            spell_id,
        })
    } else {
        Some(Event::Other {
            timestamp_ms,
            source_guid: source_guid.to_string(),
            target_guid: target_guid.to_string(),
            spell_id,
        })
    }
}
