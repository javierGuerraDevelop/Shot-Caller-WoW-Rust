use std::{cmp::Reverse, collections::BinaryHeap, time::Duration};

use crate::parser::Event;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum QueueAction {
    ShotCall {
        timestamp_ms: u64,
        message: String,
    },
    ToggleOffCooldown {
        timestamp_ms: u64,
        guid: String,
        spell_id: i32,
    },
}

impl QueueAction {
    pub fn timestamp_ms(&self) -> u64 {
        match self {
            QueueAction::ShotCall { timestamp_ms, .. } => *timestamp_ms,
            QueueAction::ToggleOffCooldown { timestamp_ms, .. } => *timestamp_ms,
        }
    }
}

impl PartialOrd for QueueAction {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for QueueAction {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.timestamp_ms().cmp(&other.timestamp_ms())
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
        is_alive: bool,
    },
}

pub struct Engine {
    party: Vec<Entity>,
    enemies: Vec<Entity>,
    interrupts: Vec<Spell>,
    crowd_control: Vec<Spell>,
    callout_queue: BinaryHeap<Reverse<QueueAction>>,
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

    pub fn handle_interrupt_event(&mut self, event: Event) {
        let Event::Interrupt { source_guid, .. } = event else {
            return;
        };

        for member in &mut self.party {
            #[rustfmt::skip]
            let Entity::Player {guid, interrupt, ..} = member else {
                continue;
            };

            if source_guid == *guid {
                interrupt.is_on_cooldown = true;
                // generate event to take it off cooldown in future
                break;
            }
        }
    }

    pub fn handle_crowd_control_event(&mut self, event: Event) {
        #[rustfmt::skip]
        let Event::CrowdControl { source_guid, spell_id, .. } = event else {
            return;
        };

        for member in &mut self.party {
            #[rustfmt::skip]
            let Entity::Player {guid, crowd_control_vec, ..} = member else {
                continue;
            };

            if source_guid == *guid {
                for crowd_control in crowd_control_vec {
                    if crowd_control.spell_id == spell_id {
                        crowd_control.is_on_cooldown = true;
                        // generate event to take it off cooldown in future
                        return;
                    }
                }
                break;
            }
        }
    }

    pub fn handle_death_event(&mut self, event: Event) {
        match event {
            Event::Death {
                timestamp,
                target_guid,
            } => {
                if target_guid.starts_with("Player-") {
                    for member in &mut self.party {
                        #[rustfmt::skip]
                        let Entity::Player { guid, is_alive, .. } = member else {
                            continue;
                        };

                        if target_guid == *guid {
                            *is_alive = false;
                            break;
                        }
                    }
                } else if target_guid.starts_with("Creature-")
                    || target_guid.starts_with("Vehicle-")
                {
                    // enemy death logic
                }
            }
            _ => return,
        }
    }

    pub fn handle_resurrection_event(&mut self, event: Event) {}

    pub fn handle_other_event(&mut self, event: Event) {}

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
            self.callout_queue
                .push(std::cmp::Reverse(QueueAction::ShotCall {
                    timestamp_ms: current_cast_time,
                    message: format!("Enemy casting {}", ability_name),
                }));
            current_cast_time += recast_delay_ms;
        }
    }

    pub fn process_queue(&mut self) {
        let now_ms = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or(std::time::Duration::default())
            .as_millis() as u64;

        while let Some(std::cmp::Reverse(action)) = self.callout_queue.peek() {
            if action.timestamp_ms() <= now_ms {
                let ready_action = self.callout_queue.pop().unwrap().0;

                match ready_action {
                    QueueAction::ShotCall { message, .. } => {
                        // Send text-to-speech
                    }
                    QueueAction::ToggleOffCooldown { guid, spell_id, .. } => {
                        // use guid to search party and toggle the cooldown for the spell
                    }
                }
            } else {
                break;
            }
        }
    }
}
