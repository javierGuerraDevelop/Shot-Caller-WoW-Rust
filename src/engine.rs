use std::{cmp::Reverse, collections::BinaryHeap};

use crate::{constants, parser::Event};

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
    pub fn new_shot_call(timestamp_ms: u64, message: String) -> QueueAction {
        QueueAction::ShotCall {
            timestamp_ms,
            message,
        }
    }

    pub fn new_toggle_off_cooldown(timestamp_ms: u64, guid: String, spell_id: i32) -> QueueAction {
        QueueAction::ToggleOffCooldown {
            timestamp_ms,
            guid,
            spell_id,
        }
    }

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

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Spell {
    Player {
        name: String,
        spell_id: i32,
        cooldown: u64,
        is_on_cooldown: bool,
    },
    Enemy {
        enemy_id: u32,
        spell_id: i32,
        first_cast_ms: i64,
        cooldown_ms: i64,
        callout: &'static str,
        is_interruptable: bool,
        is_ccable: bool,
    },
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
        first_cast: u64,
        recast_delay: u64,
        is_alive: bool,
    },
}

impl Entity {
    pub fn new_player(
        name: String,
        guid: String,
        interrupt: Spell,
        crowd_control_vec: Vec<Spell>,
    ) -> Entity {
        Entity::Player {
            name,
            guid,
            interrupt,
            crowd_control_vec,
            is_alive: true,
        }
    }

    pub fn new_enemy(
        name: String,
        guid: String,
        ability_name: String,
        first_cast: u64,
        recast_delay: u64,
    ) -> Entity {
        Entity::Enemy {
            name,
            guid,
            ability_name,
            first_cast,
            recast_delay,
            is_alive: true,
        }
    }
}

pub struct Engine {
    party: Vec<Entity>,
    enemies: Vec<Entity>,
    interrupts: Vec<Spell>,
    crowd_control: Vec<Spell>,
    callout_queue: BinaryHeap<Reverse<QueueAction>>,
    current_time_ms: u64,
}

impl Engine {
    pub fn new() -> Engine {
        Engine {
            party: vec![],
            enemies: vec![],
            interrupts: vec![],
            crowd_control: vec![],
            callout_queue: BinaryHeap::new(),
            current_time_ms: 0,
        }
    }

    pub fn set_current_time(&mut self, timestamp_ms: u64) {
        if timestamp_ms > self.current_time_ms {
            self.current_time_ms = timestamp_ms;
        }
    }

    pub fn handle_interrupt_event(&mut self, event: Event) {
        let Event::Interrupt {
            timestamp_ms,
            source_guid,
        } = event
        else {
            return;
        };

        for member in &mut self.party {
            #[rustfmt::skip]
            let Entity::Player {guid, interrupt, ..} = member else {
                continue;
            };

            if source_guid == *guid {
                if let Spell::Player {
                    is_on_cooldown,
                    spell_id,
                    ..
                } = interrupt
                {
                    *is_on_cooldown = true;
                    let action = QueueAction::new_toggle_off_cooldown(
                        timestamp_ms,
                        source_guid.clone(),
                        *spell_id,
                    );
                    self.callout_queue.push(Reverse(action));
                }
                break;
            }
        }
    }

    pub fn handle_crowd_control_event(&mut self, event: Event) {
        #[rustfmt::skip]
        let Event::CrowdControl { timestamp_ms, source_guid, spell_id } = event else {
            return;
        };

        for member in &mut self.party {
            #[rustfmt::skip]
            let Entity::Player {guid, crowd_control_vec, ..} = member else {
                continue;
            };

            if source_guid == *guid {
                for crowd_control in crowd_control_vec {
                    if let Spell::Player {
                        spell_id: cc_spell_id,
                        is_on_cooldown,
                        cooldown,
                        ..
                    } = crowd_control
                    {
                        if *cc_spell_id == spell_id {
                            *is_on_cooldown = true;
                            let action = QueueAction::new_toggle_off_cooldown(
                                timestamp_ms + *cooldown,
                                source_guid.clone(),
                                *cc_spell_id,
                            );
                            self.callout_queue.push(Reverse(action));
                            break;
                        }
                    }
                }
                break;
            }
        }
    }

    pub fn handle_death_event(&mut self, event: Event) {
        match event {
            Event::Death { target_guid, .. } => {
                if target_guid.starts_with("Player-") {
                    for member in &mut self.party {
                        #[rustfmt::skip]
                        let Entity::Player { is_alive, guid, .. } = member else {
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
                    for enemy in &mut self.enemies {
                        let Entity::Enemy { is_alive, guid, .. } = enemy else {
                            continue;
                        };
                        if target_guid == *guid {
                            *is_alive = false;
                            break;
                        }
                    }
                }
            }
            _ => return,
        }
    }

    pub fn handle_resurrection_event(&mut self, event: Event) {
        match event {
            Event::Resurrection { target_guid, .. } => {
                for member in &mut self.party {
                    #[rustfmt::skip]
                    let Entity::Player { is_alive, guid, .. } = member else {
                        continue;
                    };
                    if target_guid == *guid {
                        *is_alive = true;
                        break;
                    }
                }
            }
            _ => return,
        };
    }

    pub fn handle_other_event(&mut self, event: Event) {
        match &event {
            Event::Other { source_guid, .. } => {
                if source_guid.starts_with("Creature-") || source_guid.starts_with("Vehicle-") {
                    self.identify_enemy(event);
                } else if source_guid.starts_with("Player-") {
                    self.identify_player(event);
                }
            }
            _ => return,
        }
    }

    pub fn identify_player(&mut self, event: Event) {
        match event {
            Event::Other {
                source_guid,
                source_name,
                spell_id,
                ..
            } => {
                let Some(_) = self
                    .party
                    .iter()
                    .find(|e| matches!(e, Entity::Player { guid, .. } if guid == &source_guid))
                else {
                    let identify_class: Option<constants::PlayerClass> =
                        constants::get_class_from_identifying_spell(spell_id);
                    let player = match identify_class {
                        Some(player_class) => {
                            let interrupt = player_class.get_interrupt();
                            let crowd_control = player_class.get_crowd_control();
                            Entity::new_player(
                                String::from(source_name),
                                String::from(source_guid),
                                interrupt,
                                crowd_control,
                            )
                        }
                        None => return,
                    };
                    self.party.push(player);
                    return;
                };
            }
            _ => return,
        }
    }

    pub fn identify_enemy(&mut self, event: Event) {
        let Event::Other { source_guid, .. } = event else {
            return;
        };

        if self
            .enemies
            .iter()
            .any(|e| matches!(e, Entity::Enemy { guid, .. } if guid == &source_guid))
        {
            return;
        }

        let now_ms = self.current_time_ms;
        let five_minutes_ms = 5 * 60 * 1000;
        let first_cast_ms = 0; // Replace with actual value from enemy
        let recast_delay_ms = 0; // Replace with actual value from enemy
        let ability_name = "Unknown"; // Replace with actual value from enemy

        let mut current_cast_time = now_ms + first_cast_ms;
        let end_time = now_ms + five_minutes_ms;

        if recast_delay_ms == 0 {
            return;
        }

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
        let now_ms = self.current_time_ms;

        while let Some(std::cmp::Reverse(action)) = self.callout_queue.peek() {
            if action.timestamp_ms() <= now_ms {
                let ready_action = self.callout_queue.pop().unwrap().0;

                match ready_action {
                    QueueAction::ShotCall { message, .. } => {
                        // Send text-to-speech
                    }
                    QueueAction::ToggleOffCooldown { guid, spell_id, .. } => {
                        // Use guid to search party and toggle the cooldown for the spell
                    }
                }
            } else {
                break;
            }
        }
    }
}
