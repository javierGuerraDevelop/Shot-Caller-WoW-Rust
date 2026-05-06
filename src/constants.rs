use core::time::Duration;

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
            Self::DeathKnight => engine::Spell::new(String::from("Mind Freeze"), 47528, 15000),
            Self::DemonHunter => engine::Spell::new(String::from("Disrupt"), 183752, 15000),
            Self::Druid => engine::Spell::new(String::from("Skull Bash"), 106839, 15000),
            Self::Evoker => engine::Spell::new(String::from("Quell"), 351338, 20000),
            Self::Hunter => engine::Spell::new(String::from("Muzzle"), 187707, 15000),
            Self::Mage => engine::Spell::new(String::from("Counterspell"), 2139, 24000),
            Self::Monk => engine::Spell::new(String::from("Spear Hand Strike"), 116705, 15000),
            Self::Paladin => engine::Spell::new(String::from("Rebuke"), 96231, 15000),
            Self::Priest => engine::Spell::new(String::from("Silence"), 15487, 45000),
            Self::Rogue => engine::Spell::new(String::from("Kick"), 1766, 15000),
            Self::Shaman => engine::Spell::new(String::from("Wind Shear"), 57994, 12000),
            Self::Warlock => engine::Spell::new(String::from("Spell Lock"), 19647, 24000),
            Self::Warrior => engine::Spell::new(String::from("Pummel"), 6552, 15000),
        }
    }

    pub fn get_crowd_control(&self) -> Vec<engine::Spell> {
        match self {
            Self::DeathKnight => {
                vec![
                    engine::Spell::new(String::from("Blinding Sleet"), 207127, 60000),
                    engine::Spell::new(String::from("Gorefiend's Grasp"), 207167, 90000),
                ]
            }
            Self::DemonHunter => {
                vec![engine::Spell::new(
                    String::from("Chaos Nova"),
                    179057,
                    60000,
                )]
            }
            Self::Druid => {
                vec![
                    engine::Spell::new(String::from("Ursol's Vortex"), 102793, 60000),
                    engine::Spell::new(String::from("Typhoon"), 132469, 30000),
                ]
            }
            Self::Evoker => {
                vec![engine::Spell::new(
                    String::from("Tail Swipe"),
                    368725,
                    90000,
                )]
            }
            Self::Hunter => {
                vec![engine::Spell::new(
                    String::from("Binding Shot"),
                    109248,
                    45000,
                )]
            }
            Self::Mage => {
                vec![
                    engine::Spell::new(String::from("Frost Nova"), 122, 30000),
                    engine::Spell::new(String::from("Dragon's Breath"), 31661, 45000),
                    engine::Spell::new(String::from("Ring of Frost"), 113724, 45000),
                    engine::Spell::new(String::from("Blast Wave"), 157981, 30000),
                ]
            }
            Self::Monk => {
                vec![
                    engine::Spell::new(String::from("Leg Sweep"), 119381, 60000),
                    engine::Spell::new(String::from("Ring of Peace"), 116844, 60000),
                ]
            }
            Self::Paladin => {
                vec![engine::Spell::new(
                    String::from("Blinding Light"),
                    105421,
                    90000,
                )]
            }
            Self::Priest => {
                vec![engine::Spell::new(
                    String::from("Psychic Scream"),
                    8122,
                    60000,
                )]
            }
            Self::Rogue => {
                // No Rogue spells were listed in the provided CROWD_CONTROL_DATA
                vec![]
            }
            Self::Shaman => {
                vec![
                    engine::Spell::new(String::from("Capacitor Totem"), 192058, 60000),
                    engine::Spell::new(String::from("Earthgrab Totem"), 51485, 30000),
                    engine::Spell::new(String::from("Thunderstorm"), 51490, 45000),
                    engine::Spell::new(String::from("Sundering"), 197214, 40000),
                ]
            }
            Self::Warlock => {
                vec![
                    engine::Spell::new(String::from("Howl of Terror"), 5484, 45000),
                    engine::Spell::new(String::from("Shadowfury"), 30283, 60000),
                ]
            }
            Self::Warrior => {
                vec![
                    engine::Spell::new(String::from("Intimidating Shout"), 5246, 90000),
                    engine::Spell::new(String::from("Shockwave"), 46968, 40000),
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

struct EnemySpellEntry {
    enemy_id: u32,
    spell_id: i32,
    first_cast_ms: i64,
    cooldown_ms: i64,
    callout: &'static str,
    is_interruptable: bool,
    is_ccable: bool,
}

impl EnemySpellEntry {
    pub const fn new(
        enemy_id: u32,
        spell_id: i32,
        first_cast_ms: i64,
        cooldown_ms: i64,
        callout: &'static str,
        is_interruptable: bool,
        is_ccable: bool,
    ) -> Self {
        Self {
            enemy_id,
            spell_id,
            first_cast_ms,
            cooldown_ms,
            callout,
            is_interruptable,
            is_ccable,
        }
    }
}

const ENEMY_DATA: [EnemySpellEntry; 54] = [
    // Eco-dome
    EnemySpellEntry::new(245092, 1215850, 20000, 37000, "AoE", false, true),
    EnemySpellEntry::new(234883, 1221152, 6500, 18200, "AoE", false, true),
    EnemySpellEntry::new(242631, 1235368, 6900, 15800, "Tank Frontal", false, true),
    EnemySpellEntry::new(236995, 1226111, 15000, 20600, "Ejection", false, true),
    EnemySpellEntry::new(234957, 1221483, 15000, 20600, "Dispel", false, true),
    EnemySpellEntry::new(234962, 1221679, 6000, 13300, "Leap", false, true),
    // Tazavesh
    EnemySpellEntry::new(180567, 357827, 5000, 17000, "Leap", false, true),
    EnemySpellEntry::new(246285, 1240912, 14300, 23000, "Buster", false, true),
    EnemySpellEntry::new(246285, 1240821, 8000, 23000, "Spread", false, true),
    EnemySpellEntry::new(178165, 355429, 11300, 23000, "AOE", false, true),
    EnemySpellEntry::new(178141, 355132, 9700, 27900, "Fish sticks", false, true),
    EnemySpellEntry::new(180429, 357238, 13600, 26700, "Pulsar", false, true),
    EnemySpellEntry::new(179386, 368661, 8300, 14500, "Toss", false, true),
    EnemySpellEntry::new(177716, 351119, 8000, 18200, "Tee Pee", true, true),
    EnemySpellEntry::new(177816, 355915, 7300, 17000, "Dispel", false, true),
    EnemySpellEntry::new(180431, 357260, 13300, 21800, "Unstable Rift", true, true),
    // Halls of Atonement
    EnemySpellEntry::new(164557, 326409, 8900, 23000, "AOE", false, true),
    EnemySpellEntry::new(167607, 1235326, 15900, 32800, "Stop casting", false, true),
    EnemySpellEntry::new(164562, 326450, 15300, 24200, "Loyal Beast", true, true),
    EnemySpellEntry::new(165414, 325876, 9700, 24200, "Dispel", false, true),
    // Floodgate
    EnemySpellEntry::new(230748, 465827, 6800, 19400, "Warp blood", false, true),
    EnemySpellEntry::new(
        231014,
        465120,
        8300,
        17000,
        "Loaderbots spinning",
        false,
        true,
    ),
    // Dawnbreaker
    EnemySpellEntry::new(214761, 432448, 8300, 23000, "Seed", false, true),
    EnemySpellEntry::new(214761, 431364, 3300, 10900, "Ray", false, true),
    EnemySpellEntry::new(210966, 451107, 4900, 20600, "Cocoon", false, true),
    EnemySpellEntry::new(228540, 431309, 12400, 23000, "Curse", false, true),
    EnemySpellEntry::new(213892, 431309, 12400, 23000, "Curse", false, true),
    EnemySpellEntry::new(211261, 451102, 14300, 27800, "Aoe", false, true),
    EnemySpellEntry::new(211261, 451119, 8300, 12100, "Dot", false, true),
    EnemySpellEntry::new(211262, 451119, 3900, 12100, "Dot", false, true),
    EnemySpellEntry::new(211263, 451119, 4900, 12100, "Dot", false, true),
    EnemySpellEntry::new(211263, 450854, 12100, 24300, "Orb", false, true),
    // Ara-kara
    EnemySpellEntry::new(216293, 434793, 4000, 16900, "AoE Barrage", true, true),
    EnemySpellEntry::new(217531, 434802, 9600, 20800, "Fear", true, true),
    EnemySpellEntry::new(218324, 438877, 12100, 21900, "AoE", false, true),
    EnemySpellEntry::new(216338, 1241693, 6000, 30300, "AoE", false, true),
    EnemySpellEntry::new(223253, 448248, 4800, 20600, "Volley", true, true),
    EnemySpellEntry::new(216364, 433841, 5800, 19000, "Volley", true, true),
    // Priory of the Sacred Flame
    EnemySpellEntry::new(206696, 427609, 20400, 23000, "Stop casting", false, true),
    EnemySpellEntry::new(206696, 427621, 3800, 15700, "Impale bleed", false, true),
    EnemySpellEntry::new(221760, 444743, 9500, 24300, "Volley", true, true),
    EnemySpellEntry::new(212826, 448485, 5900, 12100, "Tank Buster", false, true),
    EnemySpellEntry::new(212826, 448492, 14700, 15700, "AoE", false, true),
    EnemySpellEntry::new(212831, 427897, 10800, 18200, "AoE", false, true),
    EnemySpellEntry::new(239833, 424431, 26100, 37600, "AoE", false, true),
    EnemySpellEntry::new(206704, 448791, 15500, 21700, "AoE", false, true),
    EnemySpellEntry::new(206699, 446776, 7000, 15800, "Leap bleed", false, true),
    // Cinderbrew Meadery
    EnemySpellEntry::new(214697, 463206, 8100, 18100, "Knock", true, true), // Tenderize
    EnemySpellEntry::new(210269, 463218, 8500, 24200, "DoT", true, true),   // Volatile Keg
    EnemySpellEntry::new(223423, 448619, 9100, 30300, "Charge", true, true), // Reckless Delivery
    EnemySpellEntry::new(220946, 442995, 10300, 23000, "AoE", true, true),  // Swarming Surprise
    EnemySpellEntry::new(222964, 441434, 8700, 23000, "Batch", false, true), // Failed Batch
    EnemySpellEntry::new(220141, 440687, 5900, 25400, "Volley", true, true), // Honey Volley
    EnemySpellEntry::new(218671, 437956, 10500, 18200, "Dispel", true, true), // Erupting Inferno
];

pub fn is_tracked_enemy(enemy_id: u32) -> bool {
    ENEMY_DATA.iter().any(|entry| entry.enemy_id == enemy_id)
}

pub fn is_enemy_ccable(enemy_id: u32) -> bool {
    ENEMY_DATA
        .iter()
        .find(|entry| entry.enemy_id == enemy_id)
        .map(|entry| entry.is_ccable)
        .unwrap_or(false)
}
