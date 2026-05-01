use std::{cmp::Reverse, collections::BinaryHeap, time::Duration};

use crate::parser::Event;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Callout {
    timestamp_ms: u64,
    message: String,
}

impl Callout {
    pub fn new(timestamp_ms: u64, message: String) -> Callout {
        Callout {
            timestamp_ms,
            message,
        }
    }
}

pub struct Spell {
    name: String,
    spell_id: i32,
    cooldown: Duration,
    is_on_cooldown: bool,
}

pub enum Entity {
    Player {
        name: String,
        guid: String,
        interrupt: Spell,
        crowd_control_vec: Vec<Spell>,
        is_alive: bool,
    },
    Enemy {
        name: String,
        guid: String,
        ability_name: String,
        first_cast: Duration,
        recast_delay: Duration,
    },
}

pub struct Engine {
    party: Vec<Entity>,
    enemies: Vec<Entity>,
    interrupts: Vec<Spell>,
    crowd_control: Vec<Spell>,
    callout_queue: BinaryHeap<Reverse<Callout>>,
}

impl Engine {
    pub fn new() -> Engine {
        Engine {
            party: vec![],
            enemies: vec![],
            interrupts: vec![],
            crowd_control: vec![],
            callout_queue: BinaryHeap::new(),
        }
    }

    pub fn handle_interrupt(&mut self, event: Event) {
        let Event::Interrupt { source_guid, .. } = event else {
            return;
        };

        for member in &mut self.party {
            #[rustfmt::skip]
            let Entity::Player {guid, interrupt, ..} = member else {
                continue;
            };

            if source_guid == guid {
                interrupt.is_on_cooldown = true;
                // future event to take it off cooldown
                break;
            }
        }
    }

    pub fn handle_crowd_control(&mut self, event: Event) {
        #[rustfmt::skip]
        let Event::CrowdControl { source_guid, spell_id, .. } = event else {
            return;
        };

        for member in &mut self.party {
            #[rustfmt::skip]
            let Entity::Player {guid, crowd_control_vec, ..} = member else {
                continue;
            };

            if source_guid == guid {
                for crowd_control in crowd_control_vec {
                    if crowd_control.spell_id == spell_id {
                        crowd_control.is_on_cooldown = true;
                        // future event to take it off cooldown
                        return;
                    }
                }
                break;
            }
        }
    }

    pub fn handle_death(&mut self, event: Event) {}

    pub fn handle_resurrection(&mut self, event: Event) {}

    pub fn handle_other(&mut self, event: Event) {}

    pub fn identify_player(&mut self, event: Event) {}

    pub fn identify_enemy(&mut self, event: Event) {
        let now_ms = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or(std::time::Duration::default())
            .as_millis() as u64;
        let five_minutes_ms = 0;
        let first_cast_ms = 0;
        let recast_delay_ms = 0;
        let ability_name = "Unknown Ability";

        let mut current_cast_time = now_ms + first_cast_ms;
        let end_time = now_ms + five_minutes_ms;

        while current_cast_time < end_time {
            self.callout_queue.push(std::cmp::Reverse(Callout::new(
                current_cast_time,
                format!("Enemy casting {}", ability_name),
            )));
            current_cast_time += recast_delay_ms;
        }
    }

    pub fn process_callouts(&mut self) {
        let now_ms = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or(std::time::Duration::default())
            .as_millis() as u64;

        while let Some(std::cmp::Reverse(callout)) = self.callout_queue.peek() {
            if callout.timestamp_ms <= now_ms {
                let ready_callout = self.callout_queue.pop().unwrap().0;
                println!("SHOTCALL: {}", ready_callout.message);
            } else {
                break;
            }
        }
    }
}
