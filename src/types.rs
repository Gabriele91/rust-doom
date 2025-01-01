use crate::map::Thing;

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(u16)]

pub enum ThingType {
    // Monsters
    Arachnotron = 68,
    ArchVile = 64,
    BaronOfHell = 3003,
    Cacodemon = 3005,
    CommanderKeen = 72,
    Cyberdemon = 16,
    Demon = 3002,
    HeavyWeaponDude = 65,
    HellKnight = 69,
    Imp = 3001,
    LostSoul = 3006,
    Mancubus = 67,
    PainElemental = 71,
    Revenant = 66,
    ShotgunGuy = 9,
    Spectre = 58,
    SpiderDemon = 7,
    WolfensteinSS = 84,
    ZombieMan = 3004,

    // Weapons
    BFG9000 = 2006,
    Chaingun = 2002,
    Chainsaw = 2005,
    PlasmaRifle = 2004,
    RocketLauncher = 2003,
    Shotgun = 2001,
    SuperShotgun = 82,

    // Ammo
    Shell4 = 2008,
    BulletBox = 2048,
    RocketBox = 2046,
    ShellBox = 2049,
    Clip = 2007,
    EnergyCell = 2047,
    EnergyPack = 17,
    Rocket = 2010,

    // Artifacts
    ArmorBonus = 2015,
    Berserk = 2023,
    ComputerMap = 2026,
    HealthBonus = 2014,
    Invulnerability = 2022,
    LightAmp = 2045,
    Megasphere = 83,
    PartialInvisibility = 2024,
    Soulsphere = 2013,

    // Powerups
    GreenArmor = 2018,
    Backpack = 8,
    Medikit = 2012,
    BlueArmor = 2019,
    RadSuit = 2025,
    Stimpack = 2011,

    // Keys
    BlueCard = 5,
    BlueSkull = 40,
    RedCard = 13,
    RedSkull = 38,
    YellowCard = 6,
    YellowSkull = 39,

    // Special
    DeathMatchStart = 11,
    MonsterSpawner = 89,
    Player1Start = 1,
    Player2Start = 2,
    Player3Start = 3,
    Player4Start = 4,
    RomeroHead = 88,
    SpawnSpot = 87,
    TeleportLanding = 14,

    // Unknown
    Unknown = 0xFFFF
}

impl TryFrom<u16> for ThingType {
    type Error = &'static str;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            // Monsters
            68 => Ok(ThingType::Arachnotron),
            64 => Ok(ThingType::ArchVile),
            3003 => Ok(ThingType::BaronOfHell),
            3005 => Ok(ThingType::Cacodemon),
            72 => Ok(ThingType::CommanderKeen),
            16 => Ok(ThingType::Cyberdemon),
            3002 => Ok(ThingType::Demon),
            65 => Ok(ThingType::HeavyWeaponDude),
            69 => Ok(ThingType::HellKnight),
            3001 => Ok(ThingType::Imp),
            3006 => Ok(ThingType::LostSoul),
            67 => Ok(ThingType::Mancubus),
            71 => Ok(ThingType::PainElemental),
            66 => Ok(ThingType::Revenant),
            9 => Ok(ThingType::ShotgunGuy),
            58 => Ok(ThingType::Spectre),
            7 => Ok(ThingType::SpiderDemon),
            84 => Ok(ThingType::WolfensteinSS),
            3004 => Ok(ThingType::ZombieMan),

            // Weapons
            2006 => Ok(ThingType::BFG9000),
            2002 => Ok(ThingType::Chaingun),
            2005 => Ok(ThingType::Chainsaw),
            2004 => Ok(ThingType::PlasmaRifle),
            2003 => Ok(ThingType::RocketLauncher),
            2001 => Ok(ThingType::Shotgun),
            82 => Ok(ThingType::SuperShotgun),

            // Ammo
            2008 => Ok(ThingType::Shell4),
            2048 => Ok(ThingType::BulletBox),
            2046 => Ok(ThingType::RocketBox),
            2049 => Ok(ThingType::ShellBox),
            2007 => Ok(ThingType::Clip),
            2047 => Ok(ThingType::EnergyCell),
            17 => Ok(ThingType::EnergyPack),
            2010 => Ok(ThingType::Rocket),

            // Artifacts
            2015 => Ok(ThingType::ArmorBonus),
            2023 => Ok(ThingType::Berserk),
            2026 => Ok(ThingType::ComputerMap),
            2014 => Ok(ThingType::HealthBonus),
            2022 => Ok(ThingType::Invulnerability),
            2045 => Ok(ThingType::LightAmp),
            83 => Ok(ThingType::Megasphere),
            2024 => Ok(ThingType::PartialInvisibility),
            2013 => Ok(ThingType::Soulsphere),

            // Powerups
            2018 => Ok(ThingType::GreenArmor),
            8 => Ok(ThingType::Backpack),
            2012 => Ok(ThingType::Medikit),
            2019 => Ok(ThingType::BlueArmor),
            2025 => Ok(ThingType::RadSuit),
            2011 => Ok(ThingType::Stimpack),

            // Keys
            5 => Ok(ThingType::BlueCard),
            40 => Ok(ThingType::BlueSkull),
            13 => Ok(ThingType::RedCard),
            38 => Ok(ThingType::RedSkull),
            6 => Ok(ThingType::YellowCard),
            39 => Ok(ThingType::YellowSkull),

            // Special
            11 => Ok(ThingType::DeathMatchStart),
            89 => Ok(ThingType::MonsterSpawner),
            1 => Ok(ThingType::Player1Start),
            2 => Ok(ThingType::Player2Start),
            3 => Ok(ThingType::Player3Start),
            4 => Ok(ThingType::Player4Start),
            88 => Ok(ThingType::RomeroHead),
            87 => Ok(ThingType::SpawnSpot),
            14 => Ok(ThingType::TeleportLanding),

            // Unknown
            _ => Err("Unknown ThingType"),
        }
    }
}

impl ThingType {
    pub fn id(&self) -> u16 {
        *self as u16
    }
}
