use crate::engine;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum PlayerClass {
    DeathKnight,
    DemonHunter,
    Druid,
    Evoker,
    Hunter,
    Mage,
    Monk,
    Paladin,
    Priest,
    Rogue,
    Shaman,
    Warlock,
    Warrior,
}

impl PlayerClass {
    pub fn get_interrupt(&self) -> engine::Spell {
        match self {
            Self::DeathKnight => engine::Spell::Player { name: String::from("Mind Freeze"), spell_id: 47528, cooldown: 15000, is_on_cooldown: false },
            Self::DemonHunter => engine::Spell::Player { name: String::from("Disrupt"), spell_id: 183752, cooldown: 15000, is_on_cooldown: false },
            Self::Druid => engine::Spell::Player { name: String::from("Skull Bash"), spell_id: 106839, cooldown: 15000, is_on_cooldown: false },
            Self::Evoker => engine::Spell::Player { name: String::from("Quell"), spell_id: 351338, cooldown: 20000, is_on_cooldown: false },
            Self::Hunter => engine::Spell::Player { name: String::from("Muzzle"), spell_id: 187707, cooldown: 15000, is_on_cooldown: false },
            Self::Mage => engine::Spell::Player { name: String::from("Counterspell"), spell_id: 2139, cooldown: 24000, is_on_cooldown: false },
            Self::Monk => engine::Spell::Player { name: String::from("Spear Hand Strike"), spell_id: 116705, cooldown: 15000, is_on_cooldown: false },
            Self::Paladin => engine::Spell::Player { name: String::from("Rebuke"), spell_id: 96231, cooldown: 15000, is_on_cooldown: false },
            Self::Priest => engine::Spell::Player { name: String::from("Silence"), spell_id: 15487, cooldown: 45000, is_on_cooldown: false },
            Self::Rogue => engine::Spell::Player { name: String::from("Kick"), spell_id: 1766, cooldown: 15000, is_on_cooldown: false },
            Self::Shaman => engine::Spell::Player { name: String::from("Wind Shear"), spell_id: 57994, cooldown: 12000, is_on_cooldown: false },
            Self::Warlock => engine::Spell::Player { name: String::from("Spell Lock"), spell_id: 19647, cooldown: 24000, is_on_cooldown: false },
            Self::Warrior => engine::Spell::Player { name: String::from("Pummel"), spell_id: 6552, cooldown: 15000, is_on_cooldown: false },
        }
    }

    pub fn get_crowd_control(&self) -> Vec<engine::Spell> {
        match self {
            Self::DeathKnight => {
                vec![
                    engine::Spell::Player { name: String::from("Blinding Sleet"), spell_id: 207127, cooldown: 60000, is_on_cooldown: false },
                    engine::Spell::Player { name: String::from("Gorefiend's Grasp"), spell_id: 207167, cooldown: 90000, is_on_cooldown: false },
                ]
            }
            Self::DemonHunter => {
                vec![engine::Spell::Player { name: 
                    String::from("Chaos Nova"), spell_id: 179057, cooldown: 60000, is_on_cooldown: false }]
            }
            Self::Druid => {
                vec![
                    engine::Spell::Player { name: String::from("Ursol's Vortex"), spell_id: 102793, cooldown: 60000, is_on_cooldown: false },
                    engine::Spell::Player { name: String::from("Typhoon"), spell_id: 132469, cooldown: 30000, is_on_cooldown: false },
                ]
            }
            Self::Evoker => {
                vec![engine::Spell::Player { name: 
                    String::from("Tail Swipe"), spell_id: 368725, cooldown: 90000, is_on_cooldown: false }]
            }
            Self::Hunter => {
                vec![engine::Spell::Player { name: 
                    String::from("Binding Shot"), spell_id: 109248, cooldown: 45000, is_on_cooldown: false }]
            }
            Self::Mage => {
                vec![
                    engine::Spell::Player { name: String::from("Frost Nova"), spell_id: 122, cooldown: 30000, is_on_cooldown: false },
                    engine::Spell::Player { name: String::from("Dragon's Breath"), spell_id: 31661, cooldown: 45000, is_on_cooldown: false },
                    engine::Spell::Player { name: String::from("Ring of Frost"), spell_id: 113724, cooldown: 45000, is_on_cooldown: false },
                    engine::Spell::Player { name: String::from("Blast Wave"), spell_id: 157981, cooldown: 30000, is_on_cooldown: false },
                ]
            }
            Self::Monk => {
                vec![
                    engine::Spell::Player { name: String::from("Leg Sweep"), spell_id: 119381, cooldown: 60000, is_on_cooldown: false },
                    engine::Spell::Player { name: String::from("Ring of Peace"), spell_id: 116844, cooldown: 60000, is_on_cooldown: false },
                ]
            }
            Self::Paladin => {
                vec![engine::Spell::Player { name: 
                    String::from("Blinding Light"), spell_id: 105421, cooldown: 90000, is_on_cooldown: false }]
            }
            Self::Priest => {
                vec![engine::Spell::Player { name: 
                    String::from("Psychic Scream"), spell_id: 8122, cooldown: 60000, is_on_cooldown: false }]
            }
            Self::Rogue => {
                vec![]
            }
            Self::Shaman => {
                vec![
                    engine::Spell::Player { name: String::from("Capacitor Totem"), spell_id: 192058, cooldown: 60000, is_on_cooldown: false },
                    engine::Spell::Player { name: String::from("Earthgrab Totem"), spell_id: 51485, cooldown: 30000, is_on_cooldown: false },
                    engine::Spell::Player { name: String::from("Thunderstorm"), spell_id: 51490, cooldown: 45000, is_on_cooldown: false },
                    engine::Spell::Player { name: String::from("Sundering"), spell_id: 197214, cooldown: 40000, is_on_cooldown: false },
                ]
            }
            Self::Warlock => {
                vec![
                    engine::Spell::Player { name: String::from("Howl of Terror"), spell_id: 5484, cooldown: 45000, is_on_cooldown: false },
                    engine::Spell::Player { name: String::from("Shadowfury"), spell_id: 30283, cooldown: 60000, is_on_cooldown: false },
                ]
            }
            Self::Warrior => {
                vec![
                    engine::Spell::Player { name: String::from("Intimidating Shout"), spell_id: 5246, cooldown: 90000, is_on_cooldown: false },
                    engine::Spell::Player { name: String::from("Shockwave"), spell_id: 46968, cooldown: 40000, is_on_cooldown: false },
                ]
            }
        }
    }
}

const IDENTIFYING_SPELLS: [(i32, PlayerClass); 27] = [
    // Death Knight
    (48743, PlayerClass::DeathKnight), // Death's Advance
    (48707, PlayerClass::DeathKnight), // Anti-Magic Shell
    // Demon Hunter
    (258920, PlayerClass::DemonHunter), // Immolation Aura
    (198793, PlayerClass::DemonHunter), // Vengeful Retreat
    // Druid
    (1126, PlayerClass::Druid), // Mark of the Wild
    (8936, PlayerClass::Druid), // Regrowth
    // Evoker
    (364342, PlayerClass::Evoker), // Blessing of the Bronze
    (361469, PlayerClass::Evoker), // Living Flame
    // Hunter
    (5384, PlayerClass::Hunter), // Feign Death
    // Mage
    (1459, PlayerClass::Mage),   // Arcane Intellect
    (1953, PlayerClass::Mage),   // Blink
    (212653, PlayerClass::Mage), // Shimmer
    // Monk
    (116670, PlayerClass::Monk), // Vivify
    (109132, PlayerClass::Monk), // Roll
    (115008, PlayerClass::Monk), // Chi Torpedo
    // Paladin
    (19750, PlayerClass::Paladin),  // Flash of Light
    (190784, PlayerClass::Paladin), // Divine Steed
    // Priest
    (21562, PlayerClass::Priest), // Power Word: Fortitude
    (2061, PlayerClass::Priest),  // Flash Heal
    // Rogue
    (1784, PlayerClass::Rogue),  // Stealth
    (36554, PlayerClass::Rogue), // Shadowstep
    // Shaman
    (8004, PlayerClass::Shaman),   // Healing Surge
    (462854, PlayerClass::Shaman), // Skyfury
    // Warlock
    (111771, PlayerClass::Warlock), // Demonic Gateway
    (29893, PlayerClass::Warlock),  // Create Soulwell
    // Warrior
    (6673, PlayerClass::Warrior), // Battle Shout
    (6544, PlayerClass::Warrior), // Heroic Leap
];

pub fn get_class_from_identifying_spell(spell_id: i32) -> Option<PlayerClass> {
    IDENTIFYING_SPELLS
        .iter()
        .find(|&&(id, _)| id == spell_id)
        .map(|&(_, class)| class)
}

pub fn is_valid_event(event_type: &str) -> bool {
    matches!(
        event_type,
        "SPELL_CAST_START"
            | "SPELL_CAST_SUCCESS"
            | "SPELL_INTERRUPT"
            | "SPELL_AURA_APPLIED"
            | "SPELL_AURA_REMOVED"
            | "SPELL_AURA_REFRESH"
            | "UNIT_DIED"
            | "UNIT_DESTROYED"
            | "SPELL_RESURRECT"
            | "SWING_DAMAGE"
            | "SPELL_DAMAGE"
            | "RANGE_DAMAGE"
            | "SPELL_PERIODIC_DAMAGE"
    )
}

pub fn is_interrupt(spell_id: i32) -> bool {
    let interrupts = [
        47528,  // Mind Freeze
        183752, // Disrupt
        106839, // Skull Bash
        78675,  // Solar Beam (Moonkin)
        351338, // Quell
        187707, // Muzzle
        2139,   // Counterspell
        116705, // Spear Hand Strike
        96231,  // Rebuke
        15487,  // Silence
        1766,   // Kick
        57994,  // Wind Shear
        19647,  // Spell Lock
        6552,   // Pummel
    ];

    interrupts.contains(&spell_id)
}

pub fn is_crowd_control(spell_id: i32) -> bool {
    let crowd_controls = [
        207127, // Blinding Sleet
        207167, // Gorefiend's Grasp
        179057, // Chaos Nova
        202138, // Sigil of Silence
        207684, // Sigil of Misery
        204598, // Sigil of Chains
        102359, // Mass Entanglement
        102793, // Ursol's Vortex
        132469, // Typhoon
        371900, // Landslide
        358269, // Deep Breath
        368725, // Tail Swipe
        109248, // Binding Shot
        122,    // Frost Nova
        31661,  // Dragon's Breath
        113724, // Ring of Frost
        157981, // Blast Wave
        119381, // Leg Sweep
        116844, // Ring of Peace
        105421, // Blinding Light
        8122,   // Psychic Scream
        192058, // Capacitor Totem
        51485,  // Earthgrab Totem
        51490,  // Thunderstorm
        197214, // Sundering
        5484,   // Howl of Terror
        30283,  // Shadowfury
        5246,   // Intimidating Shout
        46968,  // Shockwave
    ];

    crowd_controls.contains(&spell_id)
}

pub fn is_battle_rez(spell_id: i32) -> bool {
    let battle_rez_arr = [
        10609,  // Rebirth (Druid)
        376999, // Interpose / Rebirth (Evoker)
        20707,  // Soulstone (Warlock)
        61999,  // Raise Allied Dead (Death Knight)
        407133, // Abyssal Gaze (Paladin - Intercession)
    ];

    battle_rez_arr.contains(&spell_id)
}


const ENEMY_DATA: [engine::Spell; 54] = [
    // Eco-dome
    engine::Spell::Enemy { enemy_id: 245092, spell_id: 1215850, first_cast_ms: 20000, cooldown_ms: 37000, callout: "AoE", is_interruptable: false, is_ccable: true },
    engine::Spell::Enemy { enemy_id: 234883, spell_id: 1221152, first_cast_ms: 6500, cooldown_ms: 18200, callout: "AoE", is_interruptable: false, is_ccable: true },
    engine::Spell::Enemy { enemy_id: 242631, spell_id: 1235368, first_cast_ms: 6900, cooldown_ms: 15800, callout: "Tank Frontal", is_interruptable: false, is_ccable: true },
    engine::Spell::Enemy { enemy_id: 236995, spell_id: 1226111, first_cast_ms: 15000, cooldown_ms: 20600, callout: "Ejection", is_interruptable: false, is_ccable: true },
    engine::Spell::Enemy { enemy_id: 234957, spell_id: 1221483, first_cast_ms: 15000, cooldown_ms: 20600, callout: "Dispel", is_interruptable: false, is_ccable: true },
    engine::Spell::Enemy { enemy_id: 234962, spell_id: 1221679, first_cast_ms: 6000, cooldown_ms: 13300, callout: "Leap", is_interruptable: false, is_ccable: true },
    // Tazavesh
    engine::Spell::Enemy { enemy_id: 180567, spell_id: 357827, first_cast_ms: 5000, cooldown_ms: 17000, callout: "Leap", is_interruptable: false, is_ccable: true },
    engine::Spell::Enemy { enemy_id: 246285, spell_id: 1240912, first_cast_ms: 14300, cooldown_ms: 23000, callout: "Buster", is_interruptable: false, is_ccable: true },
    engine::Spell::Enemy { enemy_id: 246285, spell_id: 1240821, first_cast_ms: 8000, cooldown_ms: 23000, callout: "Spread", is_interruptable: false, is_ccable: true },
    engine::Spell::Enemy { enemy_id: 178165, spell_id: 355429, first_cast_ms: 11300, cooldown_ms: 23000, callout: "AOE", is_interruptable: false, is_ccable: true },
    engine::Spell::Enemy { enemy_id: 178141, spell_id: 355132, first_cast_ms: 9700, cooldown_ms: 27900, callout: "Fish sticks", is_interruptable: false, is_ccable: true },
    engine::Spell::Enemy { enemy_id: 180429, spell_id: 357238, first_cast_ms: 13600, cooldown_ms: 26700, callout: "Pulsar", is_interruptable: false, is_ccable: true },
    engine::Spell::Enemy { enemy_id: 179386, spell_id: 368661, first_cast_ms: 8300, cooldown_ms: 14500, callout: "Toss", is_interruptable: false, is_ccable: true },
    engine::Spell::Enemy { enemy_id: 177716, spell_id: 351119, first_cast_ms: 8000, cooldown_ms: 18200, callout: "Tee Pee", is_interruptable: true, is_ccable: true },
    engine::Spell::Enemy { enemy_id: 177816, spell_id: 355915, first_cast_ms: 7300, cooldown_ms: 17000, callout: "Dispel", is_interruptable: false, is_ccable: true },
    engine::Spell::Enemy { enemy_id: 180431, spell_id: 357260, first_cast_ms: 13300, cooldown_ms: 21800, callout: "Unstable Rift", is_interruptable: true, is_ccable: true },
    // Halls of Atonement
    engine::Spell::Enemy { enemy_id: 164557, spell_id: 326409, first_cast_ms: 8900, cooldown_ms: 23000, callout: "AOE", is_interruptable: false, is_ccable: true },
    engine::Spell::Enemy { enemy_id: 167607, spell_id: 1235326, first_cast_ms: 15900, cooldown_ms: 32800, callout: "Stop casting", is_interruptable: false, is_ccable: true },
    engine::Spell::Enemy { enemy_id: 164562, spell_id: 326450, first_cast_ms: 15300, cooldown_ms: 24200, callout: "Loyal Beast", is_interruptable: true, is_ccable: true },
    engine::Spell::Enemy { enemy_id: 165414, spell_id: 325876, first_cast_ms: 9700, cooldown_ms: 24200, callout: "Dispel", is_interruptable: false, is_ccable: true },
    // Floodgate
    engine::Spell::Enemy { enemy_id: 230748, spell_id: 465827, first_cast_ms: 6800, cooldown_ms: 19400, callout: "Warp blood", is_interruptable: false, is_ccable: true },
    engine::Spell::Enemy { enemy_id: 
        231014, spell_id: 465120, first_cast_ms: 8300, cooldown_ms: 17000, callout: "Loaderbots spinning", is_interruptable: false, is_ccable: true,
     },
    // Dawnbreaker
    engine::Spell::Enemy { enemy_id: 214761, spell_id: 432448, first_cast_ms: 8300, cooldown_ms: 23000, callout: "Seed", is_interruptable: false, is_ccable: true },
    engine::Spell::Enemy { enemy_id: 214761, spell_id: 431364, first_cast_ms: 3300, cooldown_ms: 10900, callout: "Ray", is_interruptable: false, is_ccable: true },
    engine::Spell::Enemy { enemy_id: 210966, spell_id: 451107, first_cast_ms: 4900, cooldown_ms: 20600, callout: "Cocoon", is_interruptable: false, is_ccable: true },
    engine::Spell::Enemy { enemy_id: 228540, spell_id: 431309, first_cast_ms: 12400, cooldown_ms: 23000, callout: "Curse", is_interruptable: false, is_ccable: true },
    engine::Spell::Enemy { enemy_id: 213892, spell_id: 431309, first_cast_ms: 12400, cooldown_ms: 23000, callout: "Curse", is_interruptable: false, is_ccable: true },
    engine::Spell::Enemy { enemy_id: 211261, spell_id: 451102, first_cast_ms: 14300, cooldown_ms: 27800, callout: "Aoe", is_interruptable: false, is_ccable: true },
    engine::Spell::Enemy { enemy_id: 211261, spell_id: 451119, first_cast_ms: 8300, cooldown_ms: 12100, callout: "Dot", is_interruptable: false, is_ccable: true },
    engine::Spell::Enemy { enemy_id: 211262, spell_id: 451119, first_cast_ms: 3900, cooldown_ms: 12100, callout: "Dot", is_interruptable: false, is_ccable: true },
    engine::Spell::Enemy { enemy_id: 211263, spell_id: 451119, first_cast_ms: 4900, cooldown_ms: 12100, callout: "Dot", is_interruptable: false, is_ccable: true },
    engine::Spell::Enemy { enemy_id: 211263, spell_id: 450854, first_cast_ms: 12100, cooldown_ms: 24300, callout: "Orb", is_interruptable: false, is_ccable: true },
    // Ara-kara
    engine::Spell::Enemy { enemy_id: 216293, spell_id: 434793, first_cast_ms: 4000, cooldown_ms: 16900, callout: "AoE Barrage", is_interruptable: true, is_ccable: true },
    engine::Spell::Enemy { enemy_id: 217531, spell_id: 434802, first_cast_ms: 9600, cooldown_ms: 20800, callout: "Fear", is_interruptable: true, is_ccable: true },
    engine::Spell::Enemy { enemy_id: 218324, spell_id: 438877, first_cast_ms: 12100, cooldown_ms: 21900, callout: "AoE", is_interruptable: false, is_ccable: true },
    engine::Spell::Enemy { enemy_id: 216338, spell_id: 1241693, first_cast_ms: 6000, cooldown_ms: 30300, callout: "AoE", is_interruptable: false, is_ccable: true },
    engine::Spell::Enemy { enemy_id: 223253, spell_id: 448248, first_cast_ms: 4800, cooldown_ms: 20600, callout: "Volley", is_interruptable: true, is_ccable: true },
    engine::Spell::Enemy { enemy_id: 216364, spell_id: 433841, first_cast_ms: 5800, cooldown_ms: 19000, callout: "Volley", is_interruptable: true, is_ccable: true },
    // Priory of the Sacred Flame
    engine::Spell::Enemy { enemy_id: 206696, spell_id: 427609, first_cast_ms: 20400, cooldown_ms: 23000, callout: "Stop casting", is_interruptable: false, is_ccable: true },
    engine::Spell::Enemy { enemy_id: 206696, spell_id: 427621, first_cast_ms: 3800, cooldown_ms: 15700, callout: "Impale bleed", is_interruptable: false, is_ccable: true },
    engine::Spell::Enemy { enemy_id: 221760, spell_id: 444743, first_cast_ms: 9500, cooldown_ms: 24300, callout: "Volley", is_interruptable: true, is_ccable: true },
    engine::Spell::Enemy { enemy_id: 212826, spell_id: 448485, first_cast_ms: 5900, cooldown_ms: 12100, callout: "Tank Buster", is_interruptable: false, is_ccable: true },
    engine::Spell::Enemy { enemy_id: 212826, spell_id: 448492, first_cast_ms: 14700, cooldown_ms: 15700, callout: "AoE", is_interruptable: false, is_ccable: true },
    engine::Spell::Enemy { enemy_id: 212831, spell_id: 427897, first_cast_ms: 10800, cooldown_ms: 18200, callout: "AoE", is_interruptable: false, is_ccable: true },
    engine::Spell::Enemy { enemy_id: 239833, spell_id: 424431, first_cast_ms: 26100, cooldown_ms: 37600, callout: "AoE", is_interruptable: false, is_ccable: true },
    engine::Spell::Enemy { enemy_id: 206704, spell_id: 448791, first_cast_ms: 15500, cooldown_ms: 21700, callout: "AoE", is_interruptable: false, is_ccable: true },
    engine::Spell::Enemy { enemy_id: 206699, spell_id: 446776, first_cast_ms: 7000, cooldown_ms: 15800, callout: "Leap bleed", is_interruptable: false, is_ccable: true },
    // Cinderbrew Meadery
    engine::Spell::Enemy { enemy_id: 214697, spell_id: 463206, first_cast_ms: 8100, cooldown_ms: 18100, callout: "Knock", is_interruptable: true, is_ccable: true }, // Tenderize
    engine::Spell::Enemy { enemy_id: 210269, spell_id: 463218, first_cast_ms: 8500, cooldown_ms: 24200, callout: "DoT", is_interruptable: true, is_ccable: true },   // Volatile Keg
    engine::Spell::Enemy { enemy_id: 223423, spell_id: 448619, first_cast_ms: 9100, cooldown_ms: 30300, callout: "Charge", is_interruptable: true, is_ccable: true }, // Reckless Delivery
    engine::Spell::Enemy { enemy_id: 220946, spell_id: 442995, first_cast_ms: 10300, cooldown_ms: 23000, callout: "AoE", is_interruptable: true, is_ccable: true },  // Swarming Surprise
    engine::Spell::Enemy { enemy_id: 222964, spell_id: 441434, first_cast_ms: 8700, cooldown_ms: 23000, callout: "Batch", is_interruptable: false, is_ccable: true }, // Failed Batch
    engine::Spell::Enemy { enemy_id: 220141, spell_id: 440687, first_cast_ms: 5900, cooldown_ms: 25400, callout: "Volley", is_interruptable: true, is_ccable: true }, // Honey Volley
    engine::Spell::Enemy { enemy_id: 218671, spell_id: 437956, first_cast_ms: 10500, cooldown_ms: 18200, callout: "Dispel", is_interruptable: true, is_ccable: true }, // Erupting Inferno
];

pub fn is_tracked_enemy(enemy_id: u32) -> bool {
    ENEMY_DATA.iter().any(|entry| matches!(entry, engine::Spell::Enemy { enemy_id: id, .. } if *id == enemy_id))
}

pub fn is_enemy_ccable(enemy_id: u32) -> bool {
    ENEMY_DATA
        .iter()
        .find(|entry| matches!(entry, engine::Spell::Enemy { enemy_id: id, .. } if *id == enemy_id))
        .map(|entry| match entry {
            engine::Spell::Enemy { is_ccable, .. } => *is_ccable,
            _ => false,
        })
        .unwrap_or(false)
}
