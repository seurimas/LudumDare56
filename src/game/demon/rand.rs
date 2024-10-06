use crate::DeskItem;

use super::DemonDna;

pub fn whisky2(i0: u32, i1: u32) -> u32 {
    /*
    uint32_t z0 = (i1 * 1833778363) ^ i0;
    uint32_t z1 = (z0 *  337170863) ^ (z0 >> 13) ^ z0;
    uint32_t z2 = (z1 *  620363059) ^ (z1 >> 10);
    uint32_t z3 = (z2 *  232140641) ^ (z2 >> 21);
     */
    let z0 = (i1.wrapping_mul(1833778363)) ^ i0;
    let z1 = (z0.wrapping_mul(337170863)) ^ (z0 >> 13) ^ z0;
    let z2 = (z1.wrapping_mul(620363059)) ^ (z1 >> 10);
    let z3 = (z2.wrapping_mul(232140641)) ^ (z2 >> 21);
    z3
}

fn characteristic(genes: &[u8; 16], idx: usize) -> u32 {
    let i0 = u32::from_le_bytes([
        genes[idx % genes.len()],
        genes[(idx + 1) % genes.len()],
        genes[(idx + 2) % genes.len()],
        genes[(idx + 3) % genes.len()],
    ]);
    let i1 = u32::from_le_bytes([
        genes[(idx + 4) % genes.len()],
        genes[(idx + 5) % genes.len()],
        genes[(idx + 6) % genes.len()],
        idx as u8,
    ]);
    whisky2(i0, i1)
}

fn nonced_characteristic(genes: &[u8; 16], idx: usize, nonce: u32) -> u32 {
    let i0 = u32::from_le_bytes([
        genes[idx % genes.len()],
        genes[(idx + 1) % genes.len()],
        genes[(idx + 2) % genes.len()],
        genes[(idx + 3) % genes.len()],
    ]);
    let i1 = nonce;
    whisky2(i0, i1)
}

pub fn random_genes() -> DemonDna {
    let mut genes = [0; 16];
    for i in 0..16 {
        genes[i] = rand::random();
    }
    DemonDna(genes)
}

pub fn get_characteristic_chance(dna: &DemonDna, idx: usize, chance_basis: f32) -> f32 {
    let characteristic = characteristic(&dna.0, idx);
    let characteristic = characteristic as f32 / u32::MAX as f32;
    characteristic * chance_basis
}

pub fn roll_characteristic(dna: &DemonDna, idx: usize, nonce: u32, chance_basis: f32) -> bool {
    let chance = get_characteristic_chance(dna, idx, chance_basis);
    let characteristic = nonced_characteristic(&dna.0, idx, nonce);
    let characteristic = characteristic as f32 / u32::MAX as f32;
    characteristic < chance
}

const TOOL_PREFERENCE_IDX: usize = 867;
pub fn pick_random_tool(dna: &DemonDna, nonce: u32) -> DeskItem {
    let characteristic = nonced_characteristic(&dna.0, TOOL_PREFERENCE_IDX, nonce);
    match characteristic % 2 {
        0 => DeskItem::Alembic,
        1 => DeskItem::Journal,
        _ => unreachable!(),
    }
}

// const HEAD_SKINS: [&'static str; 1] = ["debug/head"];
// const BODY_SKINS: [&'static str; 1] = ["debug/torso"];
// const HAND_SKINS: [&'static str; 1] = ["debug/hand"];
// const FOOT_SKINS: [&'static str; 1] = ["debug/foot"];
const HEAD_SKINS: [&'static str; 2] = ["heads/bug", "heads/goat"];
const BODY_SKINS: [&'static str; 2] = ["torsos/eye_see", "torsos/pumpkin"];
const HAND_SKINS: [&'static str; 2] = ["hands/claws", "hands/bear"];
const FOOT_SKINS: [&'static str; 2] = ["feet/claws", "feet/bear"];

const HEAD_SKIN_IDX: usize = 12;
const BODY_SKIN_IDX: usize = 34;
const HAND_SKIN_IDX: usize = 56;
const FOOT_SKIN_IDX: usize = 78;

pub fn get_skins(dna: &DemonDna) -> [&'static str; 4] {
    let head_skin = HEAD_SKINS[characteristic(&dna.0, HEAD_SKIN_IDX) as usize % HEAD_SKINS.len()];
    let body_skin = BODY_SKINS[characteristic(&dna.0, BODY_SKIN_IDX) as usize % BODY_SKINS.len()];
    let hand_skin = HAND_SKINS[characteristic(&dna.0, HAND_SKIN_IDX) as usize % HAND_SKINS.len()];
    let foot_skin = FOOT_SKINS[characteristic(&dna.0, FOOT_SKIN_IDX) as usize % FOOT_SKINS.len()];
    [head_skin, body_skin, hand_skin, foot_skin]
}

const POTION_TYPE_IDX: usize = 13; // These are the same, so they match up.
const POTION_EMOTION_IDX: usize = 13;
const POTION_AFTERTASTE_IDX: usize = 35;
const POTION_COLOR_IDX: usize = 57;

const POTION_TYPE: [&'static str; 8] = [
    "potion of euphoria",
    "potion of despair",
    "potion of sickness",
    "potion of mirth",
    "potion of euphoria",
    "potion of despair",
    "potion of sickness",
    "potion of mirth",
];
const POTION_EMOTIONS: [&'static str; 8] = [
    "You feel a sudden giddiness",
    "The world feels empty",
    "You are overtaken by a coughing fit",
    "Your body feels warmer",
    "You feel overwhelmed with excitement",
    "Gray gloom fills your mind",
    "Dizziness overtakes you",
    "You desperately crave some company",
];
const POTION_AFTERTASTES: [&'static str; 4] = [
    ", and the taste of copper fills your mouth.",
    ", and your mouth overflows with saliva.",
    ", and you choke back the taste of bile.",
    ", and your mouth feels dry and parched.",
];
const POTION_COLORS: [&'static str; 4] = ["red", "blue", "green", "yellow"];

pub fn get_potion_name(dna: &DemonDna) -> &'static str {
    POTION_TYPE[characteristic(&dna.0, POTION_TYPE_IDX) as usize % POTION_TYPE.len()]
}

pub fn get_potion(dna: &DemonDna) -> [&'static str; 3] {
    let emotion = POTION_EMOTIONS
        [characteristic(&dna.0, POTION_EMOTION_IDX) as usize % POTION_EMOTIONS.len()];
    let aftertaste = POTION_AFTERTASTES
        [characteristic(&dna.0, POTION_AFTERTASTE_IDX) as usize % POTION_AFTERTASTES.len()];
    let color =
        POTION_COLORS[characteristic(&dna.0, POTION_COLOR_IDX) as usize % POTION_COLORS.len()];
    [emotion, aftertaste, color]
}

const LORE_TYPE_IDX: usize = 14;
const LORE_IDX: usize = 14;
const LORE_QUALITY_IDX: usize = 36;
const LORE_QUANTITY_IDX: usize = 58;

const LORE_QUANTITIES: [&'static str; 4] = [
    "a scrap",
    "one page",
    "one very dense page",
    "several pages",
];
const LORE_TYPES: [&'static str; 4] = ["arcane knowledge", "dark magic", "demonology", "gossip"];
const LORES: [&'static str; 4] = [
    "of arcane knowledge",
    "detailing a dark ritual",
    "explaining the nature of demons",
    "of salacious gossip",
];
const LORE_QUALITIES: [&'static str; 16] = [
    "It's barely legible and consists mostly of doodles.",
    "You are pretty sure it is written in excrement.",
    "The writing is red. You're not sure where the blood came from.",
    "The paper drips with an unknown fluid.",
    "The smell emanating from the paper is nauseating.",
    "It's actually quite well written.",
    "Or... maybe it's just a shopping list?",
    "Your fingers tingle as you touch it.",
    "Alas, you're pretty sure it's all lies.",
    "You don't know what any of it means.",
    "The insights are profound.",
    "The insights are concerning.",
    "They should find a new hobby.",
    "There are a lot of exclamation marks.",
    "It's a direct copy of something you've already read.",
    "A rude doodle in the corner looks a lot like you.",
];
