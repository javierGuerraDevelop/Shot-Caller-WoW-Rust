use std::time::Duration;

use crate::parser::Event;

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
}

impl Engine {
    pub fn new() -> Engine {
        Engine {
            party: vec![],
            enemies: vec![],
            interrupts: vec![],
            crowd_control: vec![],
        }
    }

    pub fn handle_interrupt(&mut self, event: Event) {
        if let Event::Interrupt { source_guid, .. } = event {
            for member in &mut self.party {
                if let Entity::Player {
                    guid, interrupt, ..
                } = member
                {
                    if source_guid == guid {
                        interrupt.is_on_cooldown = true;
                    }
                }
            }
        }
    }

    pub fn handle_crowd_control(&self, event: Event) {}

    pub fn handle_death(&self, event: Event) {}

    pub fn handle_resurrection(&self, event: Event) {}

    pub fn handle_other(&self, event: Event) {}

    pub fn identify_player(&self, event: Event) {}

    pub fn identify_enemy(&self, event: Event) {}
}
