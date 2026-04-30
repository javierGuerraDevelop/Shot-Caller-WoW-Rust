use crate::constants;

pub enum Event<'a> {
    Interrupt {
        timestamp: &'a str,
        source_guid: &'a str,
    },
    CrowdControl {
        timestamp: &'a str,
        source_guid: &'a str,
    },
    Death {
        timestamp: &'a str,
        target_guid: &'a str,
        source_raid_flag: &'a str,
    },
    Resurrection {
        timestamp: &'a str,
        target_guid: &'a str,
    },
    Other {
        timestamp: &'a str,
        source_guid: &'a str,
        target_guid: &'a str,
    },
}

pub fn parse_line<'a>(line: &'a str) -> Option<Event<'a>> {
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
            timestamp,
            target_guid,
            source_raid_flag,
        });
    } else if event_type == "SPELL_RESURRECT" {
        return Some(Event::Resurrection {
            timestamp,
            target_guid,
        });
    }

    let spell_id = parts.next()?.parse::<i32>().ok()?;
    if constants::is_interrupt(spell_id) {
        Some(Event::Interrupt {
            timestamp,
            source_guid,
        })
    } else if constants::is_crowd_control(spell_id) {
        Some(Event::CrowdControl {
            timestamp,
            source_guid,
        })
    } else {
        Some(Event::Other {
            timestamp,
            source_guid,
            target_guid,
        })
    }
}
