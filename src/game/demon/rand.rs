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

const TOOL_PREFERENCE_IDX: usize = 0;
const TOOL_TIME_IDX: usize = 1;
pub fn pick_random_tool(dna: &DemonDna, nonce: u32) -> DeskItem {
    let max_tool = if nonce > 10 { 3 } else { 2 };
    let characteristic = nonced_characteristic(&dna.0, TOOL_PREFERENCE_IDX, nonce);
    match characteristic % max_tool {
        0 => DeskItem::Alembic,
        1 => DeskItem::Journal,
        2 => DeskItem::Doorway,
        _ => unreachable!(),
    }
}

pub fn tool_time(dna: &DemonDna, nonce: u32, min: f32, max: f32) -> f32 {
    let characteristic = nonced_characteristic(&dna.0, TOOL_TIME_IDX, nonce);
    let characteristic = characteristic as f32 / u32::MAX as f32;
    min + (max - min) * characteristic
}

// const HEAD_SKINS: [&'static str; 1] = ["debug/head"];
// const BODY_SKINS: [&'static str; 1] = ["debug/torso"];
// const HAND_SKINS: [&'static str; 1] = ["debug/hand"];
// const FOOT_SKINS: [&'static str; 1] = ["debug/foot"];
const HEAD_SKINS: [&'static str; 4] = ["heads/bug", "heads/goat", "heads/mimic", "heads/big_eyes"];
const BODY_SKINS: [&'static str; 5] = [
    "torsos/eye_see",
    "torsos/pumpkin",
    "torsos/bear",
    "torsos/spiky",
    "torsos/empty",
];
const HAND_SKINS: [&'static str; 2] = ["hands/claws", "hands/bear"];
const FOOT_SKINS: [&'static str; 2] = ["feet/claws", "feet/bear"];

const HEAD_SKIN_IDX: usize = 2;
const BODY_SKIN_IDX: usize = 3;
const HAND_SKIN_IDX: usize = 4;
const FOOT_SKIN_IDX: usize = 5;

pub fn get_skins(dna: &DemonDna) -> [&'static str; 4] {
    let head_skin = HEAD_SKINS[characteristic(&dna.0, HEAD_SKIN_IDX) as usize % HEAD_SKINS.len()];
    let body_skin = BODY_SKINS[characteristic(&dna.0, BODY_SKIN_IDX) as usize % BODY_SKINS.len()];
    let hand_skin = HAND_SKINS[characteristic(&dna.0, HAND_SKIN_IDX) as usize % HAND_SKINS.len()];
    let foot_skin = FOOT_SKINS[characteristic(&dna.0, FOOT_SKIN_IDX) as usize % FOOT_SKINS.len()];
    [head_skin, body_skin, hand_skin, foot_skin]
}

const POTION_TYPE_IDX: usize = 55; // These are the same, so they match up.
const POTION_EMOTION_IDX: usize = 55;
const POTION_AFTERTASTE_IDX: usize = 56;
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
    "You suddenly crave some company",
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

pub fn get_potion(dna: &DemonDna) -> String {
    let emotion = POTION_EMOTIONS
        [characteristic(&dna.0, POTION_EMOTION_IDX) as usize % POTION_EMOTIONS.len()];
    let aftertaste = POTION_AFTERTASTES
        [characteristic(&dna.0, POTION_AFTERTASTE_IDX) as usize % POTION_AFTERTASTES.len()];
    format!("{}{}", emotion, aftertaste)
}

const LORE_TYPE_IDX: usize = 6;
const LORE_IDX: usize = 6;
const LORE_QUALITY_IDX: usize = 7;
const LORE_QUANTITY_IDX: usize = 8;

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

pub fn get_lore(dna: &DemonDna) -> String {
    let name = get_name(dna);
    let quantity =
        LORE_QUANTITIES[characteristic(&dna.0, LORE_QUANTITY_IDX) as usize % LORE_QUANTITIES.len()];
    let lore = LORES[characteristic(&dna.0, LORE_IDX) as usize % LORES.len()];
    let quality =
        LORE_QUALITIES[characteristic(&dna.0, LORE_QUALITY_IDX) as usize % LORE_QUALITIES.len()];
    format!("{} has written {} {} {}", name, quantity, lore, quality)
}

const INTRO_GREETING_IDX: usize = 9;
const INTRO_GREETING: [&'static str; 4] = [
    "Hello, I am ",
    "Greetings, I am ",
    "Salutations, I am ",
    "Hi, I am ",
];
const INTRO_NAME_IDX: usize = 10;
const INTRO_NAME: [&'static str; 26] = [
    "Allen", "Billy", "Charles", "David", "Edward", "Frank", "George", "Henry", "Isaac", "John",
    "Kevin", "Larry", "Michael", "Nathan", "Oscar", "Peter", "Quentin", "Robert", "Samuel",
    "Thomas", "Ulysses", "Victor", "William", "Xander", "Yuri", "Zachary",
];
const INTRO_STINGER_IDX: usize = 11;
const INTRO_STINGER: [&'static str; 4] = [
    " and it is your great honor to meet me!",
    "... I think.",
    "... and, um... what was I doing?",
    " and I'm here to... do something?",
];

pub fn get_name(dna: &DemonDna) -> String {
    INTRO_GREETING[characteristic(&dna.0, INTRO_GREETING_IDX) as usize % INTRO_GREETING.len()];
    let name = INTRO_NAME[characteristic(&dna.0, INTRO_NAME_IDX) as usize % INTRO_NAME.len()];
    name.to_string()
}

pub fn get_introduction(dna: &DemonDna) -> String {
    let greeting =
        INTRO_GREETING[characteristic(&dna.0, INTRO_GREETING_IDX) as usize % INTRO_GREETING.len()];
    let name = INTRO_NAME[characteristic(&dna.0, INTRO_NAME_IDX) as usize % INTRO_NAME.len()];
    let stinger =
        INTRO_STINGER[characteristic(&dna.0, INTRO_STINGER_IDX) as usize % INTRO_STINGER.len()];
    format!("{}{}{}", greeting, name, stinger)
}

const BERATE_EXPLETIVE_IDX: usize = 12;
const BERATE_EXPLETIVE: [&'static str; 4] = ["Nerd!", "Dork!", "Geek!", "Dweeb!"];
const BERATE_REASON_IDX: usize = 13;
const BERATE_REASON: [&'static str; 4] = [
    " You're not even trying!",
    " Hurry up!",
    "!!",
    " You're wasting my time!",
];

pub fn get_berate(dna: &DemonDna) -> String {
    let expletive = BERATE_EXPLETIVE
        [characteristic(&dna.0, BERATE_EXPLETIVE_IDX) as usize % BERATE_EXPLETIVE.len()];
    let reason =
        BERATE_REASON[characteristic(&dna.0, BERATE_REASON_IDX) as usize % BERATE_REASON.len()];
    format!("{}{}", expletive, reason)
}

const COMPLAIN_EXPLETIVE_IDX: usize = 14;
const COMPLAIN_EXPLETIVE: [&'static str; 4] = ["Hey!", "Oi!", "Grrr!", "Argh!"];
const COMPLAIN_REASON_IDX: usize = 15;
const COMPLAIN_REASON: [&'static str; 4] = [
    " This place is too cold, can't you turn up the heat!",
    " I'm bored! Do something interesting!",
    " I'm hungry! Maybe I should eat you!",
    " I'm thirsty! Get me some water, you big oaf!",
];

pub fn get_complain(dna: &DemonDna) -> String {
    let expletive = COMPLAIN_EXPLETIVE
        [characteristic(&dna.0, COMPLAIN_EXPLETIVE_IDX) as usize % COMPLAIN_EXPLETIVE.len()];
    let reason = COMPLAIN_REASON
        [characteristic(&dna.0, COMPLAIN_REASON_IDX) as usize % COMPLAIN_REASON.len()];
    format!("{}{}", expletive, reason)
}

const INTERRUPTED_EXPLETIVE_IDX: usize = 16;
const INTERRUPTED_EXPLETIVE: [&'static str; 4] =
    ["What's the big idea!?", "You big oaf!", "Jerk!", "Huh??"];
const INTERRUPTED_REASON_IDX: usize = 17;
const INTERRUPTED_REASON: [&'static str; 4] = [
    " I was in the middle of something!",
    " You're so rude!",
    " I was just about to do something!",
    " I was thinking!",
];

pub fn get_interrupted(dna: &DemonDna) -> String {
    let expletive = INTERRUPTED_EXPLETIVE
        [characteristic(&dna.0, INTERRUPTED_EXPLETIVE_IDX) as usize % INTERRUPTED_EXPLETIVE.len()];
    let reason = INTERRUPTED_REASON
        [characteristic(&dna.0, INTERRUPTED_REASON_IDX) as usize % INTERRUPTED_REASON.len()];
    format!("{}{}", expletive, reason)
}
