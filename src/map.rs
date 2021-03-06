//! Rich inner-game map bindings; see [Map] for top-level variants

use serde::Serialize;

/// Common trait for all map locations of a given hex
pub trait Location: Sized {
    /// Gets map location from api-centric name
    fn from_name(name: &str) -> Option<Self> {
        todo!("map location from name '{}'", name)
    }

    /// Generates information from current details
    fn info(&self) -> LocationInfo {
        LocationInfo::new(self)
    }

    /// Checks if this map location if major or not
    fn is_major(&self) -> bool;

    /// The `x` and `y` coordinates of this location
    fn coords(&self) -> (f64, f64);

    /// Provides the api-centric name for the overall hex map tile and inner location
    fn name_api(&self) -> (&str, &str);

    /// Provides the human-readable name for the overall hex map tile and inner location
    fn name_friendly(&self) -> (&str, &str);
}

/// Serializable location information generated from a given [Location] implementation
#[derive(Serialize, Clone)]
pub struct LocationInfo {
    /// Indicates if this location is a major point
    pub is_major: bool,
    /// Coordinates of location
    pub coords: (f64, f64),
    /// Hex name which is suitable for api usage
    pub hex_api: String,
    /// Hex name to display to users
    pub hex_friendly: String,
    /// Location name which is suitable for api usage
    pub name_api: String,
    /// Location name to display to users
    pub name_friendly: String,
}

impl LocationInfo {
    /// Generates information from given location implementation
    pub fn new(location: &impl Location) -> Self {
        let (hex_api, name_api) = location.name_api();
        let (hex_friendly, name_friendly) = location.name_friendly().to_owned();

        Self {
            is_major: location.is_major(),
            coords: location.coords(),
            hex_api: hex_api.to_string(),
            hex_friendly: hex_friendly.to_string(),
            name_api: name_api.to_string(),
            name_friendly: name_friendly.to_string(),
        }
    }
}

/// Rich location marker for each hex and then location of a map
#[allow(missing_docs)]
pub enum Map {
    Stonecradle(Stonecradle),
    AllodsBight(AllodsBight),
    TempestIsland(TempestIsland),
    GreatMarch(GreatMarch),
    MarbanHol(MarbanHol),
    ViperPit(ViperPit),
    ShackledChasm(ShackledChasm),
    DeadLands(DeadLands),
    Heartlands(Heartlands),
    LinnMercy(LinnMercy),
    EndlessShore(EndlessShore),
    Godcrofts(Godcrofts),
    FishermansRow(FishermansRow),
    Westgate(Westgate),
    ReachingTrail(ReachingTrail),
    UmbralWildwood(UmbralWildwood),
    Oarbreaker(Oarbreaker),
    CallahansPassage(CallahansPassage),
    DrownedVale(DrownedVale),
    FarranacCoast(FarranacCoast),
    MooringCounty(MooringCounty),
    WeatheredExpanse(WeatheredExpanse),
    LochMor(LochMor),
}

impl Location for Map {
    fn is_major(&self) -> bool {
        match self {
            Map::Stonecradle(val) => val.is_major(),
            Map::AllodsBight(val) => val.is_major(),
            Map::TempestIsland(val) => val.is_major(),
            Map::GreatMarch(val) => val.is_major(),
            Map::MarbanHol(val) => val.is_major(),
            Map::ViperPit(val) => val.is_major(),
            Map::ShackledChasm(val) => val.is_major(),
            Map::DeadLands(val) => val.is_major(),
            Map::Heartlands(val) => val.is_major(),
            Map::LinnMercy(val) => val.is_major(),
            Map::EndlessShore(val) => val.is_major(),
            Map::Godcrofts(val) => val.is_major(),
            Map::FishermansRow(val) => val.is_major(),
            Map::Westgate(val) => val.is_major(),
            Map::ReachingTrail(val) => val.is_major(),
            Map::UmbralWildwood(val) => val.is_major(),
            Map::Oarbreaker(val) => val.is_major(),
            Map::CallahansPassage(val) => val.is_major(),
            Map::DrownedVale(val) => val.is_major(),
            Map::FarranacCoast(val) => val.is_major(),
            Map::MooringCounty(val) => val.is_major(),
            Map::WeatheredExpanse(val) => val.is_major(),
            Map::LochMor(val) => val.is_major(),
        }
    }

    fn coords(&self) -> (f64, f64) {
        match self {
            Map::Stonecradle(val) => val.coords(),
            Map::AllodsBight(val) => val.coords(),
            Map::TempestIsland(val) => val.coords(),
            Map::GreatMarch(val) => val.coords(),
            Map::MarbanHol(val) => val.coords(),
            Map::ViperPit(val) => val.coords(),
            Map::ShackledChasm(val) => val.coords(),
            Map::DeadLands(val) => val.coords(),
            Map::Heartlands(val) => val.coords(),
            Map::LinnMercy(val) => val.coords(),
            Map::EndlessShore(val) => val.coords(),
            Map::Godcrofts(val) => val.coords(),
            Map::FishermansRow(val) => val.coords(),
            Map::Westgate(val) => val.coords(),
            Map::ReachingTrail(val) => val.coords(),
            Map::UmbralWildwood(val) => val.coords(),
            Map::Oarbreaker(val) => val.coords(),
            Map::CallahansPassage(val) => val.coords(),
            Map::DrownedVale(val) => val.coords(),
            Map::FarranacCoast(val) => val.coords(),
            Map::MooringCounty(val) => val.coords(),
            Map::WeatheredExpanse(val) => val.coords(),
            Map::LochMor(val) => val.coords(),
        }
    }

    fn name_api(&self) -> (&str, &str) {
        match self {
            Map::Stonecradle(val) => val.name_api(),
            Map::AllodsBight(val) => val.name_api(),
            Map::TempestIsland(val) => val.name_api(),
            Map::GreatMarch(val) => val.name_api(),
            Map::MarbanHol(val) => val.name_api(),
            Map::ViperPit(val) => val.name_api(),
            Map::ShackledChasm(val) => val.name_api(),
            Map::DeadLands(val) => val.name_api(),
            Map::Heartlands(val) => val.name_api(),
            Map::LinnMercy(val) => val.name_api(),
            Map::EndlessShore(val) => val.name_api(),
            Map::Godcrofts(val) => val.name_api(),
            Map::FishermansRow(val) => val.name_api(),
            Map::Westgate(val) => val.name_api(),
            Map::ReachingTrail(val) => val.name_api(),
            Map::UmbralWildwood(val) => val.name_api(),
            Map::Oarbreaker(val) => val.name_api(),
            Map::CallahansPassage(val) => val.name_api(),
            Map::DrownedVale(val) => val.name_api(),
            Map::FarranacCoast(val) => val.name_api(),
            Map::MooringCounty(val) => val.name_api(),
            Map::WeatheredExpanse(val) => val.name_api(),
            Map::LochMor(val) => val.name_api(),
        }
    }

    fn name_friendly(&self) -> (&str, &str) {
        match self {
            Map::Stonecradle(val) => val.name_friendly(),
            Map::AllodsBight(val) => val.name_friendly(),
            Map::TempestIsland(val) => val.name_friendly(),
            Map::GreatMarch(val) => val.name_friendly(),
            Map::MarbanHol(val) => val.name_friendly(),
            Map::ViperPit(val) => val.name_friendly(),
            Map::ShackledChasm(val) => val.name_friendly(),
            Map::DeadLands(val) => val.name_friendly(),
            Map::Heartlands(val) => val.name_friendly(),
            Map::LinnMercy(val) => val.name_friendly(),
            Map::EndlessShore(val) => val.name_friendly(),
            Map::Godcrofts(val) => val.name_friendly(),
            Map::FishermansRow(val) => val.name_friendly(),
            Map::Westgate(val) => val.name_friendly(),
            Map::ReachingTrail(val) => val.name_friendly(),
            Map::UmbralWildwood(val) => val.name_friendly(),
            Map::Oarbreaker(val) => val.name_friendly(),
            Map::CallahansPassage(val) => val.name_friendly(),
            Map::DrownedVale(val) => val.name_friendly(),
            Map::FarranacCoast(val) => val.name_friendly(),
            Map::MooringCounty(val) => val.name_friendly(),
            Map::WeatheredExpanse(val) => val.name_friendly(),
            Map::LochMor(val) => val.name_friendly(),
        }
    }
}

//////////////////////////////////////////////////////////////
//  ALL BELOW WHERE FULLY AUTO-GENERATED BY IN-HOUSE SCRIPT //
//////////////////////////////////////////////////////////////

/// Specific map details for the `StonecradleHex` tile
pub enum Stonecradle {
    /// Buckler Sound is a major location
    BucklerSound,
    /// Fading Lights is a major location
    FadingLights,
    /// Longing is a minor location
    Longing,
    /// The Aging Ocean is a minor location
    TheAgingOcean,
    /// The Cord is a major location
    TheCord,
    /// The Heir's Knife is a major location
    TheHeirsKnife,
    /// The Loneliest Shore is a minor location
    TheLoneliestShore,
    /// The Long Fast is a major location
    TheLongFast,
    /// The Pram is a minor location
    ThePram,
    /// The Reach is a major location
    TheReach,
    /// The Roiling Comets is a minor location
    TheRoilingComets,
    /// Trammel Pool is a major location
    TrammelPool,
    /// World's End is a major location
    WorldsEnd,
}

impl Location for Stonecradle {
    fn is_major(&self) -> bool {
        match self {
            Stonecradle::BucklerSound => true,
            Stonecradle::FadingLights => true,
            Stonecradle::Longing => false,
            Stonecradle::TheAgingOcean => false,
            Stonecradle::TheCord => true,
            Stonecradle::TheHeirsKnife => true,
            Stonecradle::TheLoneliestShore => false,
            Stonecradle::TheLongFast => true,
            Stonecradle::ThePram => false,
            Stonecradle::TheReach => true,
            Stonecradle::TheRoilingComets => false,
            Stonecradle::TrammelPool => true,
            Stonecradle::WorldsEnd => true,
        }
    }

    fn coords(&self) -> (f64, f64) {
        match self {
            Stonecradle::BucklerSound => (0.51059383, 0.5146678),
            Stonecradle::FadingLights => (0.73406035, 0.69156134),
            Stonecradle::Longing => (0.36998627, 0.73207545),
            Stonecradle::TheAgingOcean => (0.30536836, 0.3464453),
            Stonecradle::TheCord => (0.5624944, 0.69426215),
            Stonecradle::TheHeirsKnife => (0.27641714, 0.7720462),
            Stonecradle::TheLoneliestShore => (0.525158, 0.25433654),
            Stonecradle::TheLongFast => (0.41714057, 0.8898146),
            Stonecradle::ThePram => (0.39138335, 0.47735217),
            Stonecradle::TheReach => (0.63581973, 0.21885662),
            Stonecradle::TheRoilingComets => (0.85634106, 0.5443087),
            Stonecradle::TrammelPool => (0.77552485, 0.46081984),
            Stonecradle::WorldsEnd => (0.2149299, 0.55217665),
        }
    }

    fn name_api(&self) -> (&str, &str) {
        (
            "Stonecradle",
            match self {
                Stonecradle::BucklerSound => "BucklerSound",
                Stonecradle::FadingLights => "FadingLights",
                Stonecradle::Longing => "Longing",
                Stonecradle::TheAgingOcean => "TheAgingOcean",
                Stonecradle::TheCord => "TheCord",
                Stonecradle::TheHeirsKnife => "TheHeirsKnife",
                Stonecradle::TheLoneliestShore => "TheLoneliestShore",
                Stonecradle::TheLongFast => "TheLongFast",
                Stonecradle::ThePram => "ThePram",
                Stonecradle::TheReach => "TheReach",
                Stonecradle::TheRoilingComets => "TheRoilingComets",
                Stonecradle::TrammelPool => "TrammelPool",
                Stonecradle::WorldsEnd => "WorldsEnd",
            },
        )
    }

    fn name_friendly(&self) -> (&str, &str) {
        (
            "Stonecradle",
            match self {
                Stonecradle::BucklerSound => "Buckler Sound",
                Stonecradle::FadingLights => "Fading Lights",
                Stonecradle::Longing => "Longing",
                Stonecradle::TheAgingOcean => "The Aging Ocean",
                Stonecradle::TheCord => "The Cord",
                Stonecradle::TheHeirsKnife => "The Heir's Knife",
                Stonecradle::TheLoneliestShore => "The Loneliest Shore",
                Stonecradle::TheLongFast => "The Long Fast",
                Stonecradle::ThePram => "The Pram",
                Stonecradle::TheReach => "The Reach",
                Stonecradle::TheRoilingComets => "The Roiling Comets",
                Stonecradle::TrammelPool => "Trammel Pool",
                Stonecradle::WorldsEnd => "World's End",
            },
        )
    }
}

/// Specific map details for the `AllodsBightHex` tile
pub enum AllodsBight {
    /// A Captain's Repose is a minor location
    ACaptainsRepose,
    /// Allod's Children is a minor location
    AllodsChildren,
    /// Belaying Trace is a major location
    BelayingTrace,
    /// Blunder Bight is a minor location
    BlunderBight,
    /// Breath of Cetus is a minor location
    BreathofCetus,
    /// Gangrenous Hollow is a minor location
    GangrenousHollow,
    /// Harpy's Perch is a minor location
    HarpysPerch,
    /// Homesick is a major location
    Homesick,
    /// Mercy's Wail is a major location
    MercysWail,
    /// Rumhold is a major location
    Rumhold,
    /// Scurvyshire is a major location
    Scurvyshire,
    /// The List is a minor location
    TheList,
    /// The Rumroad is a minor location
    TheRumroad,
    /// The Stone Plank is a major location
    TheStonePlank,
    /// The Turncoat is a major location
    TheTurncoat,
    /// Titan's End is a minor location
    TitansEnd,
    /// Witch's Last Flight is a minor location
    WitchsLastFlight,
}

impl Location for AllodsBight {
    fn is_major(&self) -> bool {
        match self {
            AllodsBight::ACaptainsRepose => false,
            AllodsBight::AllodsChildren => false,
            AllodsBight::BelayingTrace => true,
            AllodsBight::BlunderBight => false,
            AllodsBight::BreathofCetus => false,
            AllodsBight::GangrenousHollow => false,
            AllodsBight::HarpysPerch => false,
            AllodsBight::Homesick => true,
            AllodsBight::MercysWail => true,
            AllodsBight::Rumhold => true,
            AllodsBight::Scurvyshire => true,
            AllodsBight::TheList => false,
            AllodsBight::TheRumroad => false,
            AllodsBight::TheStonePlank => true,
            AllodsBight::TheTurncoat => true,
            AllodsBight::TitansEnd => false,
            AllodsBight::WitchsLastFlight => false,
        }
    }

    fn coords(&self) -> (f64, f64) {
        match self {
            AllodsBight::ACaptainsRepose => (0.5932731, 0.8355915),
            AllodsBight::AllodsChildren => (0.11191041, 0.41417748),
            AllodsBight::BelayingTrace => (0.2464044, 0.5615137),
            AllodsBight::BlunderBight => (0.6916226, 0.12754051),
            AllodsBight::BreathofCetus => (0.696068, 0.6411313),
            AllodsBight::GangrenousHollow => (0.3076385, 0.7364839),
            AllodsBight::HarpysPerch => (0.50235146, 0.3998172),
            AllodsBight::Homesick => (0.3531582, 0.38265672),
            AllodsBight::MercysWail => (0.51238513, 0.5703627),
            AllodsBight::Rumhold => (0.25670072, 0.24363878),
            AllodsBight::Scurvyshire => (0.37265283, 0.20996757),
            AllodsBight::TheList => (0.22667567, 0.80759597),
            AllodsBight::TheRumroad => (0.18821536, 0.28423867),
            AllodsBight::TheStonePlank => (0.7455613, 0.44608068),
            AllodsBight::TheTurncoat => (0.38879034, 0.73078084),
            AllodsBight::TitansEnd => (0.53741086, 0.109029606),
            AllodsBight::WitchsLastFlight => (0.6181798, 0.26963705),
        }
    }

    fn name_api(&self) -> (&str, &str) {
        (
            "AllodsBight",
            match self {
                AllodsBight::ACaptainsRepose => "ACaptainsRepose",
                AllodsBight::AllodsChildren => "AllodsChildren",
                AllodsBight::BelayingTrace => "BelayingTrace",
                AllodsBight::BlunderBight => "BlunderBight",
                AllodsBight::BreathofCetus => "BreathofCetus",
                AllodsBight::GangrenousHollow => "GangrenousHollow",
                AllodsBight::HarpysPerch => "HarpysPerch",
                AllodsBight::Homesick => "Homesick",
                AllodsBight::MercysWail => "MercysWail",
                AllodsBight::Rumhold => "Rumhold",
                AllodsBight::Scurvyshire => "Scurvyshire",
                AllodsBight::TheList => "TheList",
                AllodsBight::TheRumroad => "TheRumroad",
                AllodsBight::TheStonePlank => "TheStonePlank",
                AllodsBight::TheTurncoat => "TheTurncoat",
                AllodsBight::TitansEnd => "TitansEnd",
                AllodsBight::WitchsLastFlight => "WitchsLastFlight",
            },
        )
    }

    fn name_friendly(&self) -> (&str, &str) {
        (
            "AllodsBight",
            match self {
                AllodsBight::ACaptainsRepose => "A Captain's Repose",
                AllodsBight::AllodsChildren => "Allod's Children",
                AllodsBight::BelayingTrace => "Belaying Trace",
                AllodsBight::BlunderBight => "Blunder Bight",
                AllodsBight::BreathofCetus => "Breath of Cetus",
                AllodsBight::GangrenousHollow => "Gangrenous Hollow",
                AllodsBight::HarpysPerch => "Harpy's Perch",
                AllodsBight::Homesick => "Homesick",
                AllodsBight::MercysWail => "Mercy's Wail",
                AllodsBight::Rumhold => "Rumhold",
                AllodsBight::Scurvyshire => "Scurvyshire",
                AllodsBight::TheList => "The List",
                AllodsBight::TheRumroad => "The Rumroad",
                AllodsBight::TheStonePlank => "The Stone Plank",
                AllodsBight::TheTurncoat => "The Turncoat",
                AllodsBight::TitansEnd => "Titan's End",
                AllodsBight::WitchsLastFlight => "Witch's Last Flight",
            },
        )
    }
}

/// Specific map details for the `TempestIslandHex` tile
pub enum TempestIsland {
    /// Alchimio Estate is a major location
    AlchimioEstate,
    /// Cirris Valve is a minor location
    CirrisValve,
    /// Eros Lagoon is a minor location
    ErosLagoon,
    /// Isle of Psyche is a major location
    IsleofPsyche,
    /// Liar's Haven is a major location
    LiarsHaven,
    /// Lost Airchal is a major location
    LostAirchal,
    /// Plana Fada is a minor location
    PlanaFada,
    /// Reef is a major location
    Reef,
    /// Sclera is a minor location
    Sclera,
    /// Stratos Valve is a minor location
    StratosValve,
    /// Surge Field is a minor location
    SurgeField,
    /// Surge Gate is a major location
    SurgeGate,
    /// The Gale is a major location
    TheGale,
    /// The Iris is a major location
    TheIris,
    /// The Outwood is a minor location
    TheOutwood,
    /// The Rush is a major location
    TheRush,
}

impl Location for TempestIsland {
    fn is_major(&self) -> bool {
        match self {
            TempestIsland::AlchimioEstate => true,
            TempestIsland::CirrisValve => false,
            TempestIsland::ErosLagoon => false,
            TempestIsland::IsleofPsyche => true,
            TempestIsland::LiarsHaven => true,
            TempestIsland::LostAirchal => true,
            TempestIsland::PlanaFada => false,
            TempestIsland::Reef => true,
            TempestIsland::Sclera => false,
            TempestIsland::StratosValve => false,
            TempestIsland::SurgeField => false,
            TempestIsland::SurgeGate => true,
            TempestIsland::TheGale => true,
            TempestIsland::TheIris => true,
            TempestIsland::TheOutwood => false,
            TempestIsland::TheRush => true,
        }
    }

    fn coords(&self) -> (f64, f64) {
        match self {
            TempestIsland::AlchimioEstate => (0.30793568, 0.09539125),
            TempestIsland::CirrisValve => (0.7130552, 0.48503992),
            TempestIsland::ErosLagoon => (0.272319, 0.8196335),
            TempestIsland::IsleofPsyche => (0.25516355, 0.7106062),
            TempestIsland::LiarsHaven => (0.6746311, 0.6596021),
            TempestIsland::LostAirchal => (0.60527116, 0.09963542),
            TempestIsland::PlanaFada => (0.6023094, 0.18427029),
            TempestIsland::Reef => (0.715016, 0.31315425),
            TempestIsland::Sclera => (0.53417283, 0.5819011),
            TempestIsland::StratosValve => (0.505426, 0.49301273),
            TempestIsland::SurgeField => (0.6105891, 0.58133745),
            TempestIsland::SurgeGate => (0.40114728, 0.43872398),
            TempestIsland::TheGale => (0.48670572, 0.7953114),
            TempestIsland::TheIris => (0.6266644, 0.48053998),
            TempestIsland::TheOutwood => (0.6131708, 0.39710408),
            TempestIsland::TheRush => (0.52295, 0.37489086),
        }
    }

    fn name_api(&self) -> (&str, &str) {
        (
            "TempestIsland",
            match self {
                TempestIsland::AlchimioEstate => "AlchimioEstate",
                TempestIsland::CirrisValve => "CirrisValve",
                TempestIsland::ErosLagoon => "ErosLagoon",
                TempestIsland::IsleofPsyche => "IsleofPsyche",
                TempestIsland::LiarsHaven => "LiarsHaven",
                TempestIsland::LostAirchal => "LostAirchal",
                TempestIsland::PlanaFada => "PlanaFada",
                TempestIsland::Reef => "Reef",
                TempestIsland::Sclera => "Sclera",
                TempestIsland::StratosValve => "StratosValve",
                TempestIsland::SurgeField => "SurgeField",
                TempestIsland::SurgeGate => "SurgeGate",
                TempestIsland::TheGale => "TheGale",
                TempestIsland::TheIris => "TheIris",
                TempestIsland::TheOutwood => "TheOutwood",
                TempestIsland::TheRush => "TheRush",
            },
        )
    }

    fn name_friendly(&self) -> (&str, &str) {
        (
            "TempestIsland",
            match self {
                TempestIsland::AlchimioEstate => "Alchimio Estate",
                TempestIsland::CirrisValve => "Cirris Valve",
                TempestIsland::ErosLagoon => "Eros Lagoon",
                TempestIsland::IsleofPsyche => "Isle of Psyche",
                TempestIsland::LiarsHaven => "Liar's Haven",
                TempestIsland::LostAirchal => "Lost Airchal",
                TempestIsland::PlanaFada => "Plana Fada",
                TempestIsland::Reef => "Reef",
                TempestIsland::Sclera => "Sclera",
                TempestIsland::StratosValve => "Stratos Valve",
                TempestIsland::SurgeField => "Surge Field",
                TempestIsland::SurgeGate => "Surge Gate",
                TempestIsland::TheGale => "The Gale",
                TempestIsland::TheIris => "The Iris",
                TempestIsland::TheOutwood => "The Outwood",
                TempestIsland::TheRush => "The Rush",
            },
        )
    }
}

/// Specific map details for the `GreatMarchHex` tile
pub enum GreatMarch {
    /// Camp Senti is a major location
    CampSenti,
    /// Dalton Meadow is a minor location
    DaltonMeadow,
    /// Dendr?? Field is a major location
    Dendr??Field,
    /// Eristown is a minor location
    Eristown,
    /// Fateless Grove is a minor location
    FatelessGrove,
    /// Fengari is a minor location
    Fengari,
    /// Halting Valley is a minor location
    HaltingValley,
    /// Jack Field is a minor location
    JackField,
    /// Jackboot Creek is a minor location
    JackbootCreek,
    /// Legacy Pasture is a minor location
    LegacyPasture,
    /// Leto is a major location
    Leto,
    /// Lionsfort is a major location
    Lionsfort,
    /// Milowood is a minor location
    Milowood,
    /// Mors Range is a minor location
    MorsRange,
    /// Myrmidon's Stay is a major location
    MyrmidonsStay,
    /// Remnant Acreage is a minor location
    RemnantAcreage,
    /// Remnant Villa is a major location
    RemnantVilla,
    /// Schala Estate is a minor location
    SchalaEstate,
    /// Scrabbling Motte is a minor location
    ScrabblingMotte,
    /// Serpent Charm is a minor location
    SerpentCharm,
    /// Sitaria is a major location
    Sitaria,
    /// The Black Wing is a minor location
    TheBlackWing,
    /// The Great March is a minor location
    TheGreatMarch,
    /// The Midmarch is a minor location
    TheMidmarch,
    /// The River Senti is a minor location
    TheRiverSenti,
    /// The Spice Road is a minor location
    TheSpiceRoad,
    /// The Swan is a major location
    TheSwan,
    /// The White Wing is a minor location
    TheWhiteWing,
    /// Violet Fields is a minor location
    VioletFields,
    /// Violethome is a major location
    Violethome,
    /// Zealous Approach is a minor location
    ZealousApproach,
}

impl Location for GreatMarch {
    fn is_major(&self) -> bool {
        match self {
            GreatMarch::CampSenti => true,
            GreatMarch::DaltonMeadow => false,
            GreatMarch::Dendr??Field => true,
            GreatMarch::Eristown => false,
            GreatMarch::FatelessGrove => false,
            GreatMarch::Fengari => false,
            GreatMarch::HaltingValley => false,
            GreatMarch::JackField => false,
            GreatMarch::JackbootCreek => false,
            GreatMarch::LegacyPasture => false,
            GreatMarch::Leto => true,
            GreatMarch::Lionsfort => true,
            GreatMarch::Milowood => false,
            GreatMarch::MorsRange => false,
            GreatMarch::MyrmidonsStay => true,
            GreatMarch::RemnantAcreage => false,
            GreatMarch::RemnantVilla => true,
            GreatMarch::SchalaEstate => false,
            GreatMarch::ScrabblingMotte => false,
            GreatMarch::SerpentCharm => false,
            GreatMarch::Sitaria => true,
            GreatMarch::TheBlackWing => false,
            GreatMarch::TheGreatMarch => false,
            GreatMarch::TheMidmarch => false,
            GreatMarch::TheRiverSenti => false,
            GreatMarch::TheSpiceRoad => false,
            GreatMarch::TheSwan => true,
            GreatMarch::TheWhiteWing => false,
            GreatMarch::VioletFields => false,
            GreatMarch::Violethome => true,
            GreatMarch::ZealousApproach => false,
        }
    }

    fn coords(&self) -> (f64, f64) {
        match self {
            GreatMarch::CampSenti => (0.37455705, 0.362185),
            GreatMarch::DaltonMeadow => (0.09930786, 0.6400397),
            GreatMarch::Dendr??Field => (0.77124, 0.46152595),
            GreatMarch::Eristown => (0.78114384, 0.8727914),
            GreatMarch::FatelessGrove => (0.45355204, 0.090748146),
            GreatMarch::Fengari => (0.60990554, 0.74922425),
            GreatMarch::HaltingValley => (0.78852123, 0.3462852),
            GreatMarch::JackField => (0.41658077, 0.94294405),
            GreatMarch::JackbootCreek => (0.2494908, 0.64345753),
            GreatMarch::LegacyPasture => (0.7758614, 0.7780664),
            GreatMarch::Leto => (0.24598461, 0.5040848),
            GreatMarch::Lionsfort => (0.69568443, 0.12216068),
            GreatMarch::Milowood => (0.7739539, 0.6969876),
            GreatMarch::MorsRange => (0.69876003, 0.72293276),
            GreatMarch::MyrmidonsStay => (0.6058823, 0.22973543),
            GreatMarch::RemnantAcreage => (0.52338797, 0.51566833),
            GreatMarch::RemnantVilla => (0.5511068, 0.43947107),
            GreatMarch::SchalaEstate => (0.33548036, 0.635879),
            GreatMarch::ScrabblingMotte => (0.19585963, 0.17810662),
            GreatMarch::SerpentCharm => (0.28096482, 0.836178),
            GreatMarch::Sitaria => (0.537554, 0.6178947),
            GreatMarch::TheBlackWing => (0.48719403, 0.7353476),
            GreatMarch::TheGreatMarch => (0.6500651, 0.7322658),
            GreatMarch::TheMidmarch => (0.29200715, 0.02563422),
            GreatMarch::TheRiverSenti => (0.30557615, 0.37844858),
            GreatMarch::TheSpiceRoad => (0.37501228, 0.4377948),
            GreatMarch::TheSwan => (0.40979624, 0.8734635),
            GreatMarch::TheWhiteWing => (0.5537074, 0.83839923),
            GreatMarch::VioletFields => (0.13429698, 0.32858482),
            GreatMarch::Violethome => (0.31300274, 0.19231363),
            GreatMarch::ZealousApproach => (0.4341586, 0.6098335),
        }
    }

    fn name_api(&self) -> (&str, &str) {
        (
            "GreatMarch",
            match self {
                GreatMarch::CampSenti => "CampSenti",
                GreatMarch::DaltonMeadow => "DaltonMeadow",
                GreatMarch::Dendr??Field => "Dendr??Field",
                GreatMarch::Eristown => "Eristown",
                GreatMarch::FatelessGrove => "FatelessGrove",
                GreatMarch::Fengari => "Fengari",
                GreatMarch::HaltingValley => "HaltingValley",
                GreatMarch::JackField => "JackField",
                GreatMarch::JackbootCreek => "JackbootCreek",
                GreatMarch::LegacyPasture => "LegacyPasture",
                GreatMarch::Leto => "Leto",
                GreatMarch::Lionsfort => "Lionsfort",
                GreatMarch::Milowood => "Milowood",
                GreatMarch::MorsRange => "MorsRange",
                GreatMarch::MyrmidonsStay => "MyrmidonsStay",
                GreatMarch::RemnantAcreage => "RemnantAcreage",
                GreatMarch::RemnantVilla => "RemnantVilla",
                GreatMarch::SchalaEstate => "SchalaEstate",
                GreatMarch::ScrabblingMotte => "ScrabblingMotte",
                GreatMarch::SerpentCharm => "SerpentCharm",
                GreatMarch::Sitaria => "Sitaria",
                GreatMarch::TheBlackWing => "TheBlackWing",
                GreatMarch::TheGreatMarch => "TheGreatMarch",
                GreatMarch::TheMidmarch => "TheMidmarch",
                GreatMarch::TheRiverSenti => "TheRiverSenti",
                GreatMarch::TheSpiceRoad => "TheSpiceRoad",
                GreatMarch::TheSwan => "TheSwan",
                GreatMarch::TheWhiteWing => "TheWhiteWing",
                GreatMarch::VioletFields => "VioletFields",
                GreatMarch::Violethome => "Violethome",
                GreatMarch::ZealousApproach => "ZealousApproach",
            },
        )
    }

    fn name_friendly(&self) -> (&str, &str) {
        (
            "GreatMarch",
            match self {
                GreatMarch::CampSenti => "Camp Senti",
                GreatMarch::DaltonMeadow => "Dalton Meadow",
                GreatMarch::Dendr??Field => "Dendr?? Field",
                GreatMarch::Eristown => "Eristown",
                GreatMarch::FatelessGrove => "Fateless Grove",
                GreatMarch::Fengari => "Fengari",
                GreatMarch::HaltingValley => "Halting Valley",
                GreatMarch::JackField => "Jack Field",
                GreatMarch::JackbootCreek => "Jackboot Creek",
                GreatMarch::LegacyPasture => "Legacy Pasture",
                GreatMarch::Leto => "Leto",
                GreatMarch::Lionsfort => "Lionsfort",
                GreatMarch::Milowood => "Milowood",
                GreatMarch::MorsRange => "Mors Range",
                GreatMarch::MyrmidonsStay => "Myrmidon's Stay",
                GreatMarch::RemnantAcreage => "Remnant Acreage",
                GreatMarch::RemnantVilla => "Remnant Villa",
                GreatMarch::SchalaEstate => "Schala Estate",
                GreatMarch::ScrabblingMotte => "Scrabbling Motte",
                GreatMarch::SerpentCharm => "Serpent Charm",
                GreatMarch::Sitaria => "Sitaria",
                GreatMarch::TheBlackWing => "The Black Wing",
                GreatMarch::TheGreatMarch => "The Great March",
                GreatMarch::TheMidmarch => "The Midmarch",
                GreatMarch::TheRiverSenti => "The River Senti",
                GreatMarch::TheSpiceRoad => "The Spice Road",
                GreatMarch::TheSwan => "The Swan",
                GreatMarch::TheWhiteWing => "The White Wing",
                GreatMarch::VioletFields => "Violet Fields",
                GreatMarch::Violethome => "Violethome",
                GreatMarch::ZealousApproach => "Zealous Approach",
            },
        )
    }
}

/// Specific map details for the `MarbanHollow` tile
pub enum MarbanHol {
    /// Bleating Plateau is a minor location
    BleatingPlateau,
    /// Bubble Basin is a minor location
    BubbleBasin,
    /// Checkpoint Bua is a major location
    CheckpointBua,
    /// Deepfleet Valley is a minor location
    DeepfleetValley,
    /// Gaping Maw is a minor location
    GapingMaw,
    /// Lockheed is a major location
    Lockheed,
    /// Lockheed Breakers is a minor location
    LockheedBreakers,
    /// Lughbone Dam is a minor location
    LughboneDam,
    /// Maiden's Veil is a major location
    MaidensVeil,
    /// Mount Mac Tire is a minor location
    MountMacTire,
    /// Mox is a major location
    Mox,
    /// Oster Wall is a major location
    OsterWall,
    /// Pilgrimage is a minor location
    Pilgrimage,
    /// Sanctum is a major location
    Sanctum,
    /// Slender Cove is a minor location
    SlenderCove,
    /// The Claim is a minor location
    TheClaim,
    /// The Clutch is a minor location
    TheClutch,
    /// The Curse is a minor location
    TheCurse,
    /// The Spitrocks is a major location
    TheSpitrocks,
}

impl Location for MarbanHol {
    fn is_major(&self) -> bool {
        match self {
            MarbanHol::BleatingPlateau => false,
            MarbanHol::BubbleBasin => false,
            MarbanHol::CheckpointBua => true,
            MarbanHol::DeepfleetValley => false,
            MarbanHol::GapingMaw => false,
            MarbanHol::Lockheed => true,
            MarbanHol::LockheedBreakers => false,
            MarbanHol::LughboneDam => false,
            MarbanHol::MaidensVeil => true,
            MarbanHol::MountMacTire => false,
            MarbanHol::Mox => true,
            MarbanHol::OsterWall => true,
            MarbanHol::Pilgrimage => false,
            MarbanHol::Sanctum => true,
            MarbanHol::SlenderCove => false,
            MarbanHol::TheClaim => false,
            MarbanHol::TheClutch => false,
            MarbanHol::TheCurse => false,
            MarbanHol::TheSpitrocks => true,
        }
    }

    fn coords(&self) -> (f64, f64) {
        match self {
            MarbanHol::BleatingPlateau => (0.2703532, 0.5908183),
            MarbanHol::BubbleBasin => (0.38355634, 0.42492712),
            MarbanHol::CheckpointBua => (0.6373692, 0.4179943),
            MarbanHol::DeepfleetValley => (0.5971318, 0.65876913),
            MarbanHol::GapingMaw => (0.5627757, 0.12033677),
            MarbanHol::Lockheed => (0.6407603, 0.8987701),
            MarbanHol::LockheedBreakers => (0.55278087, 0.84404916),
            MarbanHol::LughboneDam => (0.6062604, 0.41536748),
            MarbanHol::MaidensVeil => (0.51757264, 0.5214072),
            MarbanHol::MountMacTire => (0.30355966, 0.91705936),
            MarbanHol::Mox => (0.71036744, 0.11743515),
            MarbanHol::OsterWall => (0.7994043, 0.47127146),
            MarbanHol::Pilgrimage => (0.39667153, 0.10186438),
            MarbanHol::Sanctum => (0.22553757, 0.2721918),
            MarbanHol::SlenderCove => (0.7217157, 0.28977296),
            MarbanHol::TheClaim => (0.7366861, 0.6668448),
            MarbanHol::TheClutch => (0.5142641, 0.27500752),
            MarbanHol::TheCurse => (0.62511504, 0.5131462),
            MarbanHol::TheSpitrocks => (0.3551116, 0.7273359),
        }
    }

    fn name_api(&self) -> (&str, &str) {
        (
            "MarbanHol",
            match self {
                MarbanHol::BleatingPlateau => "BleatingPlateau",
                MarbanHol::BubbleBasin => "BubbleBasin",
                MarbanHol::CheckpointBua => "CheckpointBua",
                MarbanHol::DeepfleetValley => "DeepfleetValley",
                MarbanHol::GapingMaw => "GapingMaw",
                MarbanHol::Lockheed => "Lockheed",
                MarbanHol::LockheedBreakers => "LockheedBreakers",
                MarbanHol::LughboneDam => "LughboneDam",
                MarbanHol::MaidensVeil => "MaidensVeil",
                MarbanHol::MountMacTire => "MountMacTire",
                MarbanHol::Mox => "Mox",
                MarbanHol::OsterWall => "OsterWall",
                MarbanHol::Pilgrimage => "Pilgrimage",
                MarbanHol::Sanctum => "Sanctum",
                MarbanHol::SlenderCove => "SlenderCove",
                MarbanHol::TheClaim => "TheClaim",
                MarbanHol::TheClutch => "TheClutch",
                MarbanHol::TheCurse => "TheCurse",
                MarbanHol::TheSpitrocks => "TheSpitrocks",
            },
        )
    }

    fn name_friendly(&self) -> (&str, &str) {
        (
            "MarbanHol",
            match self {
                MarbanHol::BleatingPlateau => "Bleating Plateau",
                MarbanHol::BubbleBasin => "Bubble Basin",
                MarbanHol::CheckpointBua => "Checkpoint Bua",
                MarbanHol::DeepfleetValley => "Deepfleet Valley",
                MarbanHol::GapingMaw => "Gaping Maw",
                MarbanHol::Lockheed => "Lockheed",
                MarbanHol::LockheedBreakers => "Lockheed Breakers",
                MarbanHol::LughboneDam => "Lughbone Dam",
                MarbanHol::MaidensVeil => "Maiden's Veil",
                MarbanHol::MountMacTire => "Mount Mac Tire",
                MarbanHol::Mox => "Mox",
                MarbanHol::OsterWall => "Oster Wall",
                MarbanHol::Pilgrimage => "Pilgrimage",
                MarbanHol::Sanctum => "Sanctum",
                MarbanHol::SlenderCove => "Slender Cove",
                MarbanHol::TheClaim => "The Claim",
                MarbanHol::TheClutch => "The Clutch",
                MarbanHol::TheCurse => "The Curse",
                MarbanHol::TheSpitrocks => "The Spitrocks",
            },
        )
    }
}

/// Specific map details for the `ViperPitHex` tile
pub enum ViperPit {
    /// Afric's Approach is a minor location
    AfricsApproach,
    /// Austriaca River is a minor location
    AustriacaRiver,
    /// Blackthroat is a major location
    Blackthroat,
    /// Deadsteps is a minor location
    Deadsteps,
    /// Earl Crowley is a major location
    EarlCrowley,
    /// Earl's Welcome is a minor location
    EarlsWelcome,
    /// Fleck Crossing is a major location
    FleckCrossing,
    /// Fort Viper is a major location
    FortViper,
    /// Hardcaps is a minor location
    Hardcaps,
    /// Kirknell is a major location
    Kirknell,
    /// Lake Mioira is a minor location
    LakeMioira,
    /// Moltworth is a major location
    Moltworth,
    /// Path of the Charmed is a minor location
    PathoftheCharmed,
    /// Serenity's Blight is a major location
    SerenitysBlight,
    /// Snakehead Lake is a minor location
    SnakeheadLake,
    /// The Bloody Bowery is a minor location
    TheBloodyBowery,
    /// The Friars is a major location
    TheFriars,
    /// The Lady's Lake is a minor location
    TheLadysLake,
    /// The Rockaway is a minor location
    TheRockaway,
    /// The Slithering Scales is a minor location
    TheSlitheringScales,
    /// The Tongue is a minor location
    TheTongue,
    /// Twin Fangs is a minor location
    TwinFangs,
}

impl Location for ViperPit {
    fn is_major(&self) -> bool {
        match self {
            ViperPit::AfricsApproach => false,
            ViperPit::AustriacaRiver => false,
            ViperPit::Blackthroat => true,
            ViperPit::Deadsteps => false,
            ViperPit::EarlCrowley => true,
            ViperPit::EarlsWelcome => false,
            ViperPit::FleckCrossing => true,
            ViperPit::FortViper => true,
            ViperPit::Hardcaps => false,
            ViperPit::Kirknell => true,
            ViperPit::LakeMioira => false,
            ViperPit::Moltworth => true,
            ViperPit::PathoftheCharmed => false,
            ViperPit::SerenitysBlight => true,
            ViperPit::SnakeheadLake => false,
            ViperPit::TheBloodyBowery => false,
            ViperPit::TheFriars => true,
            ViperPit::TheLadysLake => false,
            ViperPit::TheRockaway => false,
            ViperPit::TheSlitheringScales => false,
            ViperPit::TheTongue => false,
            ViperPit::TwinFangs => false,
        }
    }

    fn coords(&self) -> (f64, f64) {
        match self {
            ViperPit::AfricsApproach => (0.26043177, 0.6742477),
            ViperPit::AustriacaRiver => (0.48925853, 0.8535161),
            ViperPit::Blackthroat => (0.38273838, 0.82834363),
            ViperPit::Deadsteps => (0.5428198, 0.21101783),
            ViperPit::EarlCrowley => (0.816489, 0.5138778),
            ViperPit::EarlsWelcome => (0.14588471, 0.40977678),
            ViperPit::FleckCrossing => (0.5321962, 0.5375857),
            ViperPit::FortViper => (0.3418565, 0.599041),
            ViperPit::Hardcaps => (0.45871973, 0.032195564),
            ViperPit::Kirknell => (0.28270397, 0.548087),
            ViperPit::LakeMioira => (0.21370772, 0.10093846),
            ViperPit::Moltworth => (0.6234502, 0.71039724),
            ViperPit::PathoftheCharmed => (0.681991, 0.9418296),
            ViperPit::SerenitysBlight => (0.44389725, 0.3511407),
            ViperPit::SnakeheadLake => (0.40173814, 0.20830554),
            ViperPit::TheBloodyBowery => (0.27641982, 0.45773628),
            ViperPit::TheFriars => (0.26904154, 0.8157426),
            ViperPit::TheLadysLake => (0.19556984, 0.6328041),
            ViperPit::TheRockaway => (0.78050023, 0.7048233),
            ViperPit::TheSlitheringScales => (0.5787324, 0.43142855),
            ViperPit::TheTongue => (0.69369084, 0.1546878),
            ViperPit::TwinFangs => (0.37024617, 0.40132323),
        }
    }

    fn name_api(&self) -> (&str, &str) {
        (
            "ViperPit",
            match self {
                ViperPit::AfricsApproach => "AfricsApproach",
                ViperPit::AustriacaRiver => "AustriacaRiver",
                ViperPit::Blackthroat => "Blackthroat",
                ViperPit::Deadsteps => "Deadsteps",
                ViperPit::EarlCrowley => "EarlCrowley",
                ViperPit::EarlsWelcome => "EarlsWelcome",
                ViperPit::FleckCrossing => "FleckCrossing",
                ViperPit::FortViper => "FortViper",
                ViperPit::Hardcaps => "Hardcaps",
                ViperPit::Kirknell => "Kirknell",
                ViperPit::LakeMioira => "LakeMioira",
                ViperPit::Moltworth => "Moltworth",
                ViperPit::PathoftheCharmed => "PathoftheCharmed",
                ViperPit::SerenitysBlight => "SerenitysBlight",
                ViperPit::SnakeheadLake => "SnakeheadLake",
                ViperPit::TheBloodyBowery => "TheBloodyBowery",
                ViperPit::TheFriars => "TheFriars",
                ViperPit::TheLadysLake => "TheLadysLake",
                ViperPit::TheRockaway => "TheRockaway",
                ViperPit::TheSlitheringScales => "TheSlitheringScales",
                ViperPit::TheTongue => "TheTongue",
                ViperPit::TwinFangs => "TwinFangs",
            },
        )
    }

    fn name_friendly(&self) -> (&str, &str) {
        (
            "ViperPit",
            match self {
                ViperPit::AfricsApproach => "Afric's Approach",
                ViperPit::AustriacaRiver => "Austriaca River",
                ViperPit::Blackthroat => "Blackthroat",
                ViperPit::Deadsteps => "Deadsteps",
                ViperPit::EarlCrowley => "Earl Crowley",
                ViperPit::EarlsWelcome => "Earl's Welcome",
                ViperPit::FleckCrossing => "Fleck Crossing",
                ViperPit::FortViper => "Fort Viper",
                ViperPit::Hardcaps => "Hardcaps",
                ViperPit::Kirknell => "Kirknell",
                ViperPit::LakeMioira => "Lake Mioira",
                ViperPit::Moltworth => "Moltworth",
                ViperPit::PathoftheCharmed => "Path of the Charmed",
                ViperPit::SerenitysBlight => "Serenity's Blight",
                ViperPit::SnakeheadLake => "Snakehead Lake",
                ViperPit::TheBloodyBowery => "The Bloody Bowery",
                ViperPit::TheFriars => "The Friars",
                ViperPit::TheLadysLake => "The Lady's Lake",
                ViperPit::TheRockaway => "The Rockaway",
                ViperPit::TheSlitheringScales => "The Slithering Scales",
                ViperPit::TheTongue => "The Tongue",
                ViperPit::TwinFangs => "Twin Fangs",
            },
        )
    }
}

/// Specific map details for the `ShackledChasmHex` tile
pub enum ShackledChasm {
    /// A Careless Net is a minor location
    ACarelessNet,
    /// A New Spring is a minor location
    ANewSpring,
    /// Autumn Pyres is a minor location
    AutumnPyres,
    /// Final Step is a minor location
    FinalStep,
    /// Firstmarch is a major location
    Firstmarch,
    /// Gorgon Grove is a major location
    GorgonGrove,
    /// Hades Ladder is a minor location
    HadesLadder,
    /// Legion's Dawn is a minor location
    LegionsDawn,
    /// Limewood Holdfast is a major location
    LimewoodHoldfast,
    /// Manky Hills is a minor location
    MankyHills,
    /// Reflection is a major location
    Reflection,
    /// Savages is a major location
    Savages,
    /// Silk Farms is a major location
    SilkFarms,
    /// Simo's Run is a minor location
    SimosRun,
    /// Southreach is a minor location
    Southreach,
    /// The Bell Toll is a major location
    TheBellToll,
    /// The Blue is a minor location
    TheBlue,
    /// The First Rung is a minor location
    TheFirstRung,
    /// The Foolish Maidens is a minor location
    TheFoolishMaidens,
    /// The Grave of Rastus is a major location
    TheGraveofRastus,
    /// The Plunging is a minor location
    ThePlunging,
    /// The Vanguard is a major location
    TheVanguard,
    /// Widow's Web is a minor location
    WidowsWeb,
}

impl Location for ShackledChasm {
    fn is_major(&self) -> bool {
        match self {
            ShackledChasm::ACarelessNet => false,
            ShackledChasm::ANewSpring => false,
            ShackledChasm::AutumnPyres => false,
            ShackledChasm::FinalStep => false,
            ShackledChasm::Firstmarch => true,
            ShackledChasm::GorgonGrove => true,
            ShackledChasm::HadesLadder => false,
            ShackledChasm::LegionsDawn => false,
            ShackledChasm::LimewoodHoldfast => true,
            ShackledChasm::MankyHills => false,
            ShackledChasm::Reflection => true,
            ShackledChasm::Savages => true,
            ShackledChasm::SilkFarms => true,
            ShackledChasm::SimosRun => false,
            ShackledChasm::Southreach => false,
            ShackledChasm::TheBellToll => true,
            ShackledChasm::TheBlue => false,
            ShackledChasm::TheFirstRung => false,
            ShackledChasm::TheFoolishMaidens => false,
            ShackledChasm::TheGraveofRastus => true,
            ShackledChasm::ThePlunging => false,
            ShackledChasm::TheVanguard => true,
            ShackledChasm::WidowsWeb => false,
        }
    }

    fn coords(&self) -> (f64, f64) {
        match self {
            ShackledChasm::ACarelessNet => (0.43234968, 0.8701505),
            ShackledChasm::ANewSpring => (0.65531296, 0.26313987),
            ShackledChasm::AutumnPyres => (0.5103819, 0.3264112),
            ShackledChasm::FinalStep => (0.30303696, 0.1718076),
            ShackledChasm::Firstmarch => (0.84196997, 0.38732836),
            ShackledChasm::GorgonGrove => (0.22335887, 0.35604578),
            ShackledChasm::HadesLadder => (0.4075216, 0.2975581),
            ShackledChasm::LegionsDawn => (0.34624228, 0.9482727),
            ShackledChasm::LimewoodHoldfast => (0.47156802, 0.4749517),
            ShackledChasm::MankyHills => (0.88514185, 0.58751076),
            ShackledChasm::Reflection => (0.21630011, 0.60432374),
            ShackledChasm::Savages => (0.39030778, 0.1846902),
            ShackledChasm::SilkFarms => (0.4394004, 0.66509956),
            ShackledChasm::SimosRun => (0.6844458, 0.8596173),
            ShackledChasm::Southreach => (0.4774761, 0.09446996),
            ShackledChasm::TheBellToll => (0.6211396, 0.3829189),
            ShackledChasm::TheBlue => (0.5409541, 0.8753167),
            ShackledChasm::TheFirstRung => (0.35477364, 0.44228175),
            ShackledChasm::TheFoolishMaidens => (0.075329885, 0.57303905),
            ShackledChasm::TheGraveofRastus => (0.6505256, 0.15729238),
            ShackledChasm::ThePlunging => (0.19269316, 0.75184214),
            ShackledChasm::TheVanguard => (0.69273907, 0.6221794),
            ShackledChasm::WidowsWeb => (0.5278859, 0.6728514),
        }
    }

    fn name_api(&self) -> (&str, &str) {
        (
            "ShackledChasm",
            match self {
                ShackledChasm::ACarelessNet => "ACarelessNet",
                ShackledChasm::ANewSpring => "ANewSpring",
                ShackledChasm::AutumnPyres => "AutumnPyres",
                ShackledChasm::FinalStep => "FinalStep",
                ShackledChasm::Firstmarch => "Firstmarch",
                ShackledChasm::GorgonGrove => "GorgonGrove",
                ShackledChasm::HadesLadder => "HadesLadder",
                ShackledChasm::LegionsDawn => "LegionsDawn",
                ShackledChasm::LimewoodHoldfast => "LimewoodHoldfast",
                ShackledChasm::MankyHills => "MankyHills",
                ShackledChasm::Reflection => "Reflection",
                ShackledChasm::Savages => "Savages",
                ShackledChasm::SilkFarms => "SilkFarms",
                ShackledChasm::SimosRun => "SimosRun",
                ShackledChasm::Southreach => "Southreach",
                ShackledChasm::TheBellToll => "TheBellToll",
                ShackledChasm::TheBlue => "TheBlue",
                ShackledChasm::TheFirstRung => "TheFirstRung",
                ShackledChasm::TheFoolishMaidens => "TheFoolishMaidens",
                ShackledChasm::TheGraveofRastus => "TheGraveofRastus",
                ShackledChasm::ThePlunging => "ThePlunging",
                ShackledChasm::TheVanguard => "TheVanguard",
                ShackledChasm::WidowsWeb => "WidowsWeb",
            },
        )
    }

    fn name_friendly(&self) -> (&str, &str) {
        (
            "ShackledChasm",
            match self {
                ShackledChasm::ACarelessNet => "A Careless Net",
                ShackledChasm::ANewSpring => "A New Spring",
                ShackledChasm::AutumnPyres => "Autumn Pyres",
                ShackledChasm::FinalStep => "Final Step",
                ShackledChasm::Firstmarch => "Firstmarch",
                ShackledChasm::GorgonGrove => "Gorgon Grove",
                ShackledChasm::HadesLadder => "Hades Ladder",
                ShackledChasm::LegionsDawn => "Legion's Dawn",
                ShackledChasm::LimewoodHoldfast => "Limewood Holdfast",
                ShackledChasm::MankyHills => "Manky Hills",
                ShackledChasm::Reflection => "Reflection",
                ShackledChasm::Savages => "Savages",
                ShackledChasm::SilkFarms => "Silk Farms",
                ShackledChasm::SimosRun => "Simo's Run",
                ShackledChasm::Southreach => "Southreach",
                ShackledChasm::TheBellToll => "The Bell Toll",
                ShackledChasm::TheBlue => "The Blue",
                ShackledChasm::TheFirstRung => "The First Rung",
                ShackledChasm::TheFoolishMaidens => "The Foolish Maidens",
                ShackledChasm::TheGraveofRastus => "The Grave of Rastus",
                ShackledChasm::ThePlunging => "The Plunging",
                ShackledChasm::TheVanguard => "The Vanguard",
                ShackledChasm::WidowsWeb => "Widow's Web",
            },
        )
    }
}

/// Specific map details for the `DeadLandsHex` tile
pub enum DeadLands {
    /// Abandoned Ward is a major location
    AbandonedWard,
    /// Biting Tarn is a minor location
    BitingTarn,
    /// Border Concourse is a minor location
    BorderConcourse,
    /// Border Thicket is a minor location
    BorderThicket,
    /// Brine Glen is a major location
    BrineGlen,
    /// Callahan's Belt is a minor location
    CallahansBelt,
    /// Callahan's Boot is a major location
    CallahansBoot,
    /// Callahan's Gate is a major location
    CallahansGate,
    /// Carpal Trail is a minor location
    CarpalTrail,
    /// Cemetary Junction is a minor location
    CemetaryJunction,
    /// Cemetary Lane is a minor location
    CemetaryLane,
    /// Coracoid Footpath is a minor location
    CoracoidFootpath,
    /// Crumbling Passage is a minor location
    CrumblingPassage,
    /// Hope's Causeway is a minor location
    HopesCauseway,
    /// Iron's End is a major location
    IronsEnd,
    /// Jaspar Range is a minor location
    JasparRange,
    /// Liberation Point is a major location
    LiberationPoint,
    /// Mandible Crossroads is a minor location
    MandibleCrossroads,
    /// Marrow Copse is a minor location
    MarrowCopse,
    /// Mercy Meadow is a minor location
    MercyMeadow,
    /// Mercy's End is a minor location
    MercysEnd,
    /// Overgrown Pasture is a minor location
    OvergrownPasture,
    /// Path to the Sun is a minor location
    PathtotheSun,
    /// Pommel Annex is a minor location
    PommelAnnex,
    /// Sun's Hollow is a major location
    SunsHollow,
    /// Sunhaven Gateway is a minor location
    SunhavenGateway,
    /// Tarsal Pathway is a minor location
    TarsalPathway,
    /// The Abbey Drag is a minor location
    TheAbbeyDrag,
    /// The Blade is a minor location
    TheBlade,
    /// The Boneyard is a minor location
    TheBoneyard,
    /// The Crossing is a minor location
    TheCrossing,
    /// The Great March is a minor location
    TheGreatMarch,
    /// The Iron Passage is a minor location
    TheIronPassage,
    /// The Iron Road is a minor location
    TheIronRoad,
    /// The Pits is a major location
    ThePits,
    /// The Plaza is a minor location
    ThePlaza,
    /// The Salt Farms is a major location
    TheSaltFarms,
    /// The Salt March is a major location
    TheSaltMarch,
    /// The Salt Trail is a minor location
    TheSaltTrail,
    /// The Shorn Fields is a minor location
    TheShornFields,
    /// The Spine is a major location
    TheSpine,
    /// The Steppes is a minor location
    TheSteppes,
}

impl Location for DeadLands {
    fn is_major(&self) -> bool {
        match self {
            DeadLands::AbandonedWard => true,
            DeadLands::BitingTarn => false,
            DeadLands::BorderConcourse => false,
            DeadLands::BorderThicket => false,
            DeadLands::BrineGlen => true,
            DeadLands::CallahansBelt => false,
            DeadLands::CallahansBoot => true,
            DeadLands::CallahansGate => true,
            DeadLands::CarpalTrail => false,
            DeadLands::CemetaryJunction => false,
            DeadLands::CemetaryLane => false,
            DeadLands::CoracoidFootpath => false,
            DeadLands::CrumblingPassage => false,
            DeadLands::HopesCauseway => false,
            DeadLands::IronsEnd => true,
            DeadLands::JasparRange => false,
            DeadLands::LiberationPoint => true,
            DeadLands::MandibleCrossroads => false,
            DeadLands::MarrowCopse => false,
            DeadLands::MercyMeadow => false,
            DeadLands::MercysEnd => false,
            DeadLands::OvergrownPasture => false,
            DeadLands::PathtotheSun => false,
            DeadLands::PommelAnnex => false,
            DeadLands::SunsHollow => true,
            DeadLands::SunhavenGateway => false,
            DeadLands::TarsalPathway => false,
            DeadLands::TheAbbeyDrag => false,
            DeadLands::TheBlade => false,
            DeadLands::TheBoneyard => false,
            DeadLands::TheCrossing => false,
            DeadLands::TheGreatMarch => false,
            DeadLands::TheIronPassage => false,
            DeadLands::TheIronRoad => false,
            DeadLands::ThePits => true,
            DeadLands::ThePlaza => false,
            DeadLands::TheSaltFarms => true,
            DeadLands::TheSaltMarch => true,
            DeadLands::TheSaltTrail => false,
            DeadLands::TheShornFields => false,
            DeadLands::TheSpine => true,
            DeadLands::TheSteppes => false,
        }
    }

    fn coords(&self) -> (f64, f64) {
        match self {
            DeadLands::AbandonedWard => (0.4065897, 0.4973474),
            DeadLands::BitingTarn => (0.82187194, 0.70537686),
            DeadLands::BorderConcourse => (0.8157356, 0.39016595),
            DeadLands::BorderThicket => (0.575372, 0.32447058),
            DeadLands::BrineGlen => (0.72561765, 0.7220242),
            DeadLands::CallahansBelt => (0.65850776, 0.31060126),
            DeadLands::CallahansBoot => (0.5475686, 0.4907771),
            DeadLands::CallahansGate => (0.62483287, 0.110333465),
            DeadLands::CarpalTrail => (0.29142216, 0.419198),
            DeadLands::CemetaryJunction => (0.3243318, 0.5793932),
            DeadLands::CemetaryLane => (0.38247642, 0.6585629),
            DeadLands::CoracoidFootpath => (0.107194014, 0.40021968),
            DeadLands::CrumblingPassage => (0.41715685, 0.094843924),
            DeadLands::HopesCauseway => (0.52830327, 0.57945776),
            DeadLands::IronsEnd => (0.46120775, 0.26207438),
            DeadLands::JasparRange => (0.28807592, 0.6632166),
            DeadLands::LiberationPoint => (0.24296169, 0.5517707),
            DeadLands::MandibleCrossroads => (0.20130293, 0.33566573),
            DeadLands::MarrowCopse => (0.35198447, 0.3130791),
            DeadLands::MercyMeadow => (0.12712973, 0.54193884),
            DeadLands::MercysEnd => (0.6229164, 0.8306555),
            DeadLands::OvergrownPasture => (0.15952048, 0.6434271),
            DeadLands::PathtotheSun => (0.2729478, 0.8931891),
            DeadLands::PommelAnnex => (0.48857507, 0.39334437),
            DeadLands::SunsHollow => (0.27335224, 0.7215378),
            DeadLands::SunhavenGateway => (0.38722476, 0.56053424),
            DeadLands::TarsalPathway => (0.20509845, 0.41586992),
            DeadLands::TheAbbeyDrag => (0.7539659, 0.12900522),
            DeadLands::TheBlade => (0.5743842, 0.38184917),
            DeadLands::TheBoneyard => (0.5943891, 0.27163824),
            DeadLands::TheCrossing => (0.41764602, 0.4466156),
            DeadLands::TheGreatMarch => (0.35632595, 0.88145),
            DeadLands::TheIronPassage => (0.48119673, 0.31544363),
            DeadLands::TheIronRoad => (0.52781564, 0.13242057),
            DeadLands::ThePits => (0.4651194, 0.6297798),
            DeadLands::ThePlaza => (0.42960146, 0.40103972),
            DeadLands::TheSaltFarms => (0.4569705, 0.8376134),
            DeadLands::TheSaltMarch => (0.82289404, 0.48192352),
            DeadLands::TheSaltTrail => (0.40397635, 0.7451171),
            DeadLands::TheShornFields => (0.45553347, 0.91392154),
            DeadLands::TheSpine => (0.30697387, 0.23582822),
            DeadLands::TheSteppes => (0.66786224, 0.5271883),
        }
    }

    fn name_api(&self) -> (&str, &str) {
        (
            "DeadLands",
            match self {
                DeadLands::AbandonedWard => "AbandonedWard",
                DeadLands::BitingTarn => "BitingTarn",
                DeadLands::BorderConcourse => "BorderConcourse",
                DeadLands::BorderThicket => "BorderThicket",
                DeadLands::BrineGlen => "BrineGlen",
                DeadLands::CallahansBelt => "CallahansBelt",
                DeadLands::CallahansBoot => "CallahansBoot",
                DeadLands::CallahansGate => "CallahansGate",
                DeadLands::CarpalTrail => "CarpalTrail",
                DeadLands::CemetaryJunction => "CemetaryJunction",
                DeadLands::CemetaryLane => "CemetaryLane",
                DeadLands::CoracoidFootpath => "CoracoidFootpath",
                DeadLands::CrumblingPassage => "CrumblingPassage",
                DeadLands::HopesCauseway => "HopesCauseway",
                DeadLands::IronsEnd => "IronsEnd",
                DeadLands::JasparRange => "JasparRange",
                DeadLands::LiberationPoint => "LiberationPoint",
                DeadLands::MandibleCrossroads => "MandibleCrossroads",
                DeadLands::MarrowCopse => "MarrowCopse",
                DeadLands::MercyMeadow => "MercyMeadow",
                DeadLands::MercysEnd => "MercysEnd",
                DeadLands::OvergrownPasture => "OvergrownPasture",
                DeadLands::PathtotheSun => "PathtotheSun",
                DeadLands::PommelAnnex => "PommelAnnex",
                DeadLands::SunsHollow => "SunsHollow",
                DeadLands::SunhavenGateway => "SunhavenGateway",
                DeadLands::TarsalPathway => "TarsalPathway",
                DeadLands::TheAbbeyDrag => "TheAbbeyDrag",
                DeadLands::TheBlade => "TheBlade",
                DeadLands::TheBoneyard => "TheBoneyard",
                DeadLands::TheCrossing => "TheCrossing",
                DeadLands::TheGreatMarch => "TheGreatMarch",
                DeadLands::TheIronPassage => "TheIronPassage",
                DeadLands::TheIronRoad => "TheIronRoad",
                DeadLands::ThePits => "ThePits",
                DeadLands::ThePlaza => "ThePlaza",
                DeadLands::TheSaltFarms => "TheSaltFarms",
                DeadLands::TheSaltMarch => "TheSaltMarch",
                DeadLands::TheSaltTrail => "TheSaltTrail",
                DeadLands::TheShornFields => "TheShornFields",
                DeadLands::TheSpine => "TheSpine",
                DeadLands::TheSteppes => "TheSteppes",
            },
        )
    }

    fn name_friendly(&self) -> (&str, &str) {
        (
            "DeadLands",
            match self {
                DeadLands::AbandonedWard => "Abandoned Ward",
                DeadLands::BitingTarn => "Biting Tarn",
                DeadLands::BorderConcourse => "Border Concourse",
                DeadLands::BorderThicket => "Border Thicket",
                DeadLands::BrineGlen => "Brine Glen",
                DeadLands::CallahansBelt => "Callahan's Belt",
                DeadLands::CallahansBoot => "Callahan's Boot",
                DeadLands::CallahansGate => "Callahan's Gate",
                DeadLands::CarpalTrail => "Carpal Trail",
                DeadLands::CemetaryJunction => "Cemetary Junction",
                DeadLands::CemetaryLane => "Cemetary Lane",
                DeadLands::CoracoidFootpath => "Coracoid Footpath",
                DeadLands::CrumblingPassage => "Crumbling Passage",
                DeadLands::HopesCauseway => "Hope's Causeway",
                DeadLands::IronsEnd => "Iron's End",
                DeadLands::JasparRange => "Jaspar Range",
                DeadLands::LiberationPoint => "Liberation Point",
                DeadLands::MandibleCrossroads => "Mandible Crossroads",
                DeadLands::MarrowCopse => "Marrow Copse",
                DeadLands::MercyMeadow => "Mercy Meadow",
                DeadLands::MercysEnd => "Mercy's End",
                DeadLands::OvergrownPasture => "Overgrown Pasture",
                DeadLands::PathtotheSun => "Path to the Sun",
                DeadLands::PommelAnnex => "Pommel Annex",
                DeadLands::SunsHollow => "Sun's Hollow",
                DeadLands::SunhavenGateway => "Sunhaven Gateway",
                DeadLands::TarsalPathway => "Tarsal Pathway",
                DeadLands::TheAbbeyDrag => "The Abbey Drag",
                DeadLands::TheBlade => "The Blade",
                DeadLands::TheBoneyard => "The Boneyard",
                DeadLands::TheCrossing => "The Crossing",
                DeadLands::TheGreatMarch => "The Great March",
                DeadLands::TheIronPassage => "The Iron Passage",
                DeadLands::TheIronRoad => "The Iron Road",
                DeadLands::ThePits => "The Pits",
                DeadLands::ThePlaza => "The Plaza",
                DeadLands::TheSaltFarms => "The Salt Farms",
                DeadLands::TheSaltMarch => "The Salt March",
                DeadLands::TheSaltTrail => "The Salt Trail",
                DeadLands::TheShornFields => "The Shorn Fields",
                DeadLands::TheSpine => "The Spine",
                DeadLands::TheSteppes => "The Steppes",
            },
        )
    }
}

/// Specific map details for the `HeartlandsHex` tile
pub enum Heartlands {
    /// 18th Sideroad is a minor location
    EighteenthSideroad,
    /// Barronshire  is a minor location
    Barronshire,
    /// Barronswall is a major location
    Barronswall,
    /// Barrony Ranch is a major location
    BarronyRanch,
    /// Barrony Road is a minor location
    BarronyRoad,
    /// Cageroad is a minor location
    Cageroad,
    /// Crater Basin is a minor location
    CraterBasin,
    /// Deeplaw Post is a major location
    DeeplawPost,
    /// Erimos Ranch is a minor location
    ErimosRanch,
    /// Fort Providence is a major location
    FortProvidence,
    /// Greenfield Orchard is a major location
    GreenfieldOrchard,
    /// Harvester's Range is a minor location
    HarvestersRange,
    /// Janus Field is a minor location
    JanusField,
    /// Kos Meadows is a minor location
    KosMeadows,
    /// Loftmire is a minor location
    Loftmire,
    /// Lower Barrony Field is a minor location
    LowerBarronyField,
    /// Oleander Fields is a minor location
    OleanderFields,
    /// Oleander Homestead is a major location
    OleanderHomestead,
    /// Pandora Compound is a minor location
    PandoraCompound,
    /// Proex?? is a major location
    Proex??,
    /// Providence Field is a minor location
    ProvidenceField,
    /// The Blemish is a major location
    TheBlemish,
    /// The Breach is a major location
    TheBreach,
    /// The Fuming Pen is a minor location
    TheFumingPen,
    /// The Orchard Wall is a minor location
    TheOrchardWall,
    /// The Plough is a major location
    ThePlough,
    /// The Rollcage is a minor location
    TheRollcage,
    /// The Salt Crossing is a minor location
    TheSaltCrossing,
    /// Upper Barrony Field is a minor location
    UpperBarronyField,
    /// Upper Heartlands is a minor location
    UpperHeartlands,
}

impl Location for Heartlands {
    fn is_major(&self) -> bool {
        match self {
            Heartlands::EighteenthSideroad => false,
            Heartlands::Barronshire => false,
            Heartlands::Barronswall => true,
            Heartlands::BarronyRanch => true,
            Heartlands::BarronyRoad => false,
            Heartlands::Cageroad => false,
            Heartlands::CraterBasin => false,
            Heartlands::DeeplawPost => true,
            Heartlands::ErimosRanch => false,
            Heartlands::FortProvidence => true,
            Heartlands::GreenfieldOrchard => true,
            Heartlands::HarvestersRange => false,
            Heartlands::JanusField => false,
            Heartlands::KosMeadows => false,
            Heartlands::Loftmire => false,
            Heartlands::LowerBarronyField => false,
            Heartlands::OleanderFields => false,
            Heartlands::OleanderHomestead => true,
            Heartlands::PandoraCompound => false,
            Heartlands::Proex?? => true,
            Heartlands::ProvidenceField => false,
            Heartlands::TheBlemish => true,
            Heartlands::TheBreach => true,
            Heartlands::TheFumingPen => false,
            Heartlands::TheOrchardWall => false,
            Heartlands::ThePlough => true,
            Heartlands::TheRollcage => false,
            Heartlands::TheSaltCrossing => false,
            Heartlands::UpperBarronyField => false,
            Heartlands::UpperHeartlands => false,
        }
    }

    fn coords(&self) -> (f64, f64) {
        match self {
            Heartlands::EighteenthSideroad => (0.5635743, 0.07919855),
            Heartlands::Barronshire => (0.30404317, 0.5293029),
            Heartlands::Barronswall => (0.39372203, 0.40942293),
            Heartlands::BarronyRanch => (0.3733477, 0.1098376),
            Heartlands::BarronyRoad => (0.4902308, 0.20186654),
            Heartlands::Cageroad => (0.08390267, 0.58169395),
            Heartlands::CraterBasin => (0.27787402, 0.20430255),
            Heartlands::DeeplawPost => (0.24315597, 0.43437782),
            Heartlands::ErimosRanch => (0.5130686, 0.84595346),
            Heartlands::FortProvidence => (0.75807965, 0.3792637),
            Heartlands::GreenfieldOrchard => (0.7317328, 0.20453982),
            Heartlands::HarvestersRange => (0.47385046, 0.5450412),
            Heartlands::JanusField => (0.41109726, 0.87362325),
            Heartlands::KosMeadows => (0.23384759, 0.87482136),
            Heartlands::Loftmire => (0.5516043, 0.44199702),
            Heartlands::LowerBarronyField => (0.4626953, 0.31559077),
            Heartlands::OleanderFields => (0.67658395, 0.639753),
            Heartlands::OleanderHomestead => (0.8278165, 0.4906847),
            Heartlands::PandoraCompound => (0.44034275, 0.7100705),
            Heartlands::Proex?? => (0.65262276, 0.8350459),
            Heartlands::ProvidenceField => (0.6118128, 0.3481131),
            Heartlands::TheBlemish => (0.6491603, 0.4668125),
            Heartlands::TheBreach => (0.36424437, 0.2708485),
            Heartlands::TheFumingPen => (0.19509101, 0.6871336),
            Heartlands::TheOrchardWall => (0.69853735, 0.24974519),
            Heartlands::ThePlough => (0.54745007, 0.24971269),
            Heartlands::TheRollcage => (0.15662682, 0.34313864),
            Heartlands::TheSaltCrossing => (0.78533167, 0.07084702),
            Heartlands::UpperBarronyField => (0.47143874, 0.092723675),
            Heartlands::UpperHeartlands => (0.32511675, 0.11369127),
        }
    }

    fn name_api(&self) -> (&str, &str) {
        (
            "Heartlands",
            match self {
                Heartlands::EighteenthSideroad => "18thSideroad",
                Heartlands::Barronshire => "Barronshire",
                Heartlands::Barronswall => "Barronswall",
                Heartlands::BarronyRanch => "BarronyRanch",
                Heartlands::BarronyRoad => "BarronyRoad",
                Heartlands::Cageroad => "Cageroad",
                Heartlands::CraterBasin => "CraterBasin",
                Heartlands::DeeplawPost => "DeeplawPost",
                Heartlands::ErimosRanch => "ErimosRanch",
                Heartlands::FortProvidence => "FortProvidence",
                Heartlands::GreenfieldOrchard => "GreenfieldOrchard",
                Heartlands::HarvestersRange => "HarvestersRange",
                Heartlands::JanusField => "JanusField",
                Heartlands::KosMeadows => "KosMeadows",
                Heartlands::Loftmire => "Loftmire",
                Heartlands::LowerBarronyField => "LowerBarronyField",
                Heartlands::OleanderFields => "OleanderFields",
                Heartlands::OleanderHomestead => "OleanderHomestead",
                Heartlands::PandoraCompound => "PandoraCompound",
                Heartlands::Proex?? => "Proex??",
                Heartlands::ProvidenceField => "ProvidenceField",
                Heartlands::TheBlemish => "TheBlemish",
                Heartlands::TheBreach => "TheBreach",
                Heartlands::TheFumingPen => "TheFumingPen",
                Heartlands::TheOrchardWall => "TheOrchardWall",
                Heartlands::ThePlough => "ThePlough",
                Heartlands::TheRollcage => "TheRollcage",
                Heartlands::TheSaltCrossing => "TheSaltCrossing",
                Heartlands::UpperBarronyField => "UpperBarronyField",
                Heartlands::UpperHeartlands => "UpperHeartlands",
            },
        )
    }

    fn name_friendly(&self) -> (&str, &str) {
        (
            "Heartlands",
            match self {
                Heartlands::EighteenthSideroad => "18th Sideroad",
                Heartlands::Barronshire => "Barronshire ",
                Heartlands::Barronswall => "Barronswall",
                Heartlands::BarronyRanch => "Barrony Ranch",
                Heartlands::BarronyRoad => "Barrony Road",
                Heartlands::Cageroad => "Cageroad",
                Heartlands::CraterBasin => "Crater Basin",
                Heartlands::DeeplawPost => "Deeplaw Post",
                Heartlands::ErimosRanch => "Erimos Ranch",
                Heartlands::FortProvidence => "Fort Providence",
                Heartlands::GreenfieldOrchard => "Greenfield Orchard",
                Heartlands::HarvestersRange => "Harvester's Range",
                Heartlands::JanusField => "Janus Field",
                Heartlands::KosMeadows => "Kos Meadows",
                Heartlands::Loftmire => "Loftmire",
                Heartlands::LowerBarronyField => "Lower Barrony Field",
                Heartlands::OleanderFields => "Oleander Fields",
                Heartlands::OleanderHomestead => "Oleander Homestead",
                Heartlands::PandoraCompound => "Pandora Compound",
                Heartlands::Proex?? => "Proex??",
                Heartlands::ProvidenceField => "Providence Field",
                Heartlands::TheBlemish => "The Blemish",
                Heartlands::TheBreach => "The Breach",
                Heartlands::TheFumingPen => "The Fuming Pen",
                Heartlands::TheOrchardWall => "The Orchard Wall",
                Heartlands::ThePlough => "The Plough",
                Heartlands::TheRollcage => "The Rollcage",
                Heartlands::TheSaltCrossing => "The Salt Crossing",
                Heartlands::UpperBarronyField => "Upper Barrony Field",
                Heartlands::UpperHeartlands => "Upper Heartlands",
            },
        )
    }
}

/// Specific map details for the `LinnMercyHex` tile
pub enum LinnMercy {
    /// Blackroad is a minor location
    Blackroad,
    /// Fort Duncan is a major location
    FortDuncan,
    /// Gallant Gough Boulevard is a minor location
    GallantGoughBoulevard,
    /// Hardline is a major location
    Hardline,
    /// Lathair is a major location
    Lathair,
    /// Merciful Strait is a minor location
    MercifulStrait,
    /// Mudhole is a minor location
    Mudhole,
    /// Nathair is a minor location
    Nathair,
    /// Outwich Ranch is a major location
    OutwichRanch,
    /// Rotdust is a major location
    Rotdust,
    /// Solas Burn is a minor location
    SolasBurn,
    /// The Crimson Gardens is a major location
    TheCrimsonGardens,
    /// The Drone is a minor location
    TheDrone,
    /// The First Coin is a major location
    TheFirstCoin,
    /// The Great Scale is a minor location
    TheGreatScale,
    /// The Last Grove is a major location
    TheLastGrove,
    /// The Long Whine is a major location
    TheLongWhine,
    /// The Prairie Bazaar is a major location
    ThePrairieBazaar,
    /// The River Mercy is a minor location
    TheRiverMercy,
    /// Ulster Falls is a major location
    UlsterFalls,
}

impl Location for LinnMercy {
    fn is_major(&self) -> bool {
        match self {
            LinnMercy::Blackroad => false,
            LinnMercy::FortDuncan => true,
            LinnMercy::GallantGoughBoulevard => false,
            LinnMercy::Hardline => true,
            LinnMercy::Lathair => true,
            LinnMercy::MercifulStrait => false,
            LinnMercy::Mudhole => false,
            LinnMercy::Nathair => false,
            LinnMercy::OutwichRanch => true,
            LinnMercy::Rotdust => true,
            LinnMercy::SolasBurn => false,
            LinnMercy::TheCrimsonGardens => true,
            LinnMercy::TheDrone => false,
            LinnMercy::TheFirstCoin => true,
            LinnMercy::TheGreatScale => false,
            LinnMercy::TheLastGrove => true,
            LinnMercy::TheLongWhine => true,
            LinnMercy::ThePrairieBazaar => true,
            LinnMercy::TheRiverMercy => false,
            LinnMercy::UlsterFalls => true,
        }
    }

    fn coords(&self) -> (f64, f64) {
        match self {
            LinnMercy::Blackroad => (0.46516654, 0.16474915),
            LinnMercy::FortDuncan => (0.73673666, 0.7660624),
            LinnMercy::GallantGoughBoulevard => (0.09820685, 0.43021995),
            LinnMercy::Hardline => (0.35991466, 0.58879375),
            LinnMercy::Lathair => (0.7258961, 0.3119082),
            LinnMercy::MercifulStrait => (0.8419282, 0.67530245),
            LinnMercy::Mudhole => (0.5700142, 0.30238822),
            LinnMercy::Nathair => (0.5938587, 0.41346693),
            LinnMercy::OutwichRanch => (0.8370126, 0.4868126),
            LinnMercy::Rotdust => (0.5856841, 0.15178874),
            LinnMercy::SolasBurn => (0.73095423, 0.22541259),
            LinnMercy::TheCrimsonGardens => (0.22958569, 0.62841123),
            LinnMercy::TheDrone => (0.19357644, 0.22261861),
            LinnMercy::TheFirstCoin => (0.40840903, 0.7184508),
            LinnMercy::TheGreatScale => (0.4111321, 0.29084763),
            LinnMercy::TheLastGrove => (0.38268816, 0.8845894),
            LinnMercy::TheLongWhine => (0.35783502, 0.209964),
            LinnMercy::ThePrairieBazaar => (0.6143232, 0.7404008),
            LinnMercy::TheRiverMercy => (0.64159554, 0.6174841),
            LinnMercy::UlsterFalls => (0.46410957, 0.45022747),
        }
    }

    fn name_api(&self) -> (&str, &str) {
        (
            "LinnMercy",
            match self {
                LinnMercy::Blackroad => "Blackroad",
                LinnMercy::FortDuncan => "FortDuncan",
                LinnMercy::GallantGoughBoulevard => "GallantGoughBoulevard",
                LinnMercy::Hardline => "Hardline",
                LinnMercy::Lathair => "Lathair",
                LinnMercy::MercifulStrait => "MercifulStrait",
                LinnMercy::Mudhole => "Mudhole",
                LinnMercy::Nathair => "Nathair",
                LinnMercy::OutwichRanch => "OutwichRanch",
                LinnMercy::Rotdust => "Rotdust",
                LinnMercy::SolasBurn => "SolasBurn",
                LinnMercy::TheCrimsonGardens => "TheCrimsonGardens",
                LinnMercy::TheDrone => "TheDrone",
                LinnMercy::TheFirstCoin => "TheFirstCoin",
                LinnMercy::TheGreatScale => "TheGreatScale",
                LinnMercy::TheLastGrove => "TheLastGrove",
                LinnMercy::TheLongWhine => "TheLongWhine",
                LinnMercy::ThePrairieBazaar => "ThePrairieBazaar",
                LinnMercy::TheRiverMercy => "TheRiverMercy",
                LinnMercy::UlsterFalls => "UlsterFalls",
            },
        )
    }

    fn name_friendly(&self) -> (&str, &str) {
        (
            "LinnMercy",
            match self {
                LinnMercy::Blackroad => "Blackroad",
                LinnMercy::FortDuncan => "Fort Duncan",
                LinnMercy::GallantGoughBoulevard => "Gallant Gough Boulevard",
                LinnMercy::Hardline => "Hardline",
                LinnMercy::Lathair => "Lathair",
                LinnMercy::MercifulStrait => "Merciful Strait",
                LinnMercy::Mudhole => "Mudhole",
                LinnMercy::Nathair => "Nathair",
                LinnMercy::OutwichRanch => "Outwich Ranch",
                LinnMercy::Rotdust => "Rotdust",
                LinnMercy::SolasBurn => "Solas Burn",
                LinnMercy::TheCrimsonGardens => "The Crimson Gardens",
                LinnMercy::TheDrone => "The Drone",
                LinnMercy::TheFirstCoin => "The First Coin",
                LinnMercy::TheGreatScale => "The Great Scale",
                LinnMercy::TheLastGrove => "The Last Grove",
                LinnMercy::TheLongWhine => "The Long Whine",
                LinnMercy::ThePrairieBazaar => "The Prairie Bazaar",
                LinnMercy::TheRiverMercy => "The River Mercy",
                LinnMercy::UlsterFalls => "Ulster Falls",
            },
        )
    }
}

/// Specific map details for the `EndlessShoreHex` tile
pub enum EndlessShore {
    /// Balor's Crown is a minor location
    BalorsCrown,
    /// Battered Landing is a minor location
    BatteredLanding,
    /// Brackish Point is a major location
    BrackishPoint,
    /// Dannan Ridge is a major location
    DannanRidge,
    /// Dearg's Fang is a minor location
    DeargsFang,
    /// Enduring Wake is a major location
    EnduringWake,
    /// Iron Junction is a major location
    IronJunction,
    /// Kelpie's Mane is a minor location
    KelpiesMane,
    /// Kelpie's Tail is a minor location
    KelpiesTail,
    /// Liegehearth is a minor location
    Liegehearth,
    /// Merrow's Rest is a minor location
    MerrowsRest,
    /// Saltbrook Channel is a major location
    SaltbrookChannel,
    /// S??dhe Fall is a minor location
    S??dheFall,
    /// The Dannan Coast is a minor location
    TheDannanCoast,
    /// The Dark Road is a minor location
    TheDarkRoad,
    /// The Evil Eye is a major location
    TheEvilEye,
    /// The North Star is a minor location
    TheNorthStar,
    /// The Old Jack Tar is a major location
    TheOldJackTar,
    /// The Overland is a major location
    TheOverland,
    /// The Selkie Bluffs is a minor location
    TheSelkieBluffs,
    /// The Styx is a minor location
    TheStyx,
    /// The Whispering Waves is a minor location
    TheWhisperingWaves,
    /// Tuatha Watchpost is a major location
    TuathaWatchpost,
    /// Vulpine Watch is a major location
    VulpineWatch,
    /// Wellchurch is a major location
    Wellchurch,
    /// Woodbind is a major location
    Woodbind,
}

impl Location for EndlessShore {
    fn is_major(&self) -> bool {
        match self {
            EndlessShore::BalorsCrown => false,
            EndlessShore::BatteredLanding => false,
            EndlessShore::BrackishPoint => true,
            EndlessShore::DannanRidge => true,
            EndlessShore::DeargsFang => false,
            EndlessShore::EnduringWake => true,
            EndlessShore::IronJunction => true,
            EndlessShore::KelpiesMane => false,
            EndlessShore::KelpiesTail => false,
            EndlessShore::Liegehearth => false,
            EndlessShore::MerrowsRest => false,
            EndlessShore::SaltbrookChannel => true,
            EndlessShore::S??dheFall => false,
            EndlessShore::TheDannanCoast => false,
            EndlessShore::TheDarkRoad => false,
            EndlessShore::TheEvilEye => true,
            EndlessShore::TheNorthStar => false,
            EndlessShore::TheOldJackTar => true,
            EndlessShore::TheOverland => true,
            EndlessShore::TheSelkieBluffs => false,
            EndlessShore::TheStyx => false,
            EndlessShore::TheWhisperingWaves => false,
            EndlessShore::TuathaWatchpost => true,
            EndlessShore::VulpineWatch => true,
            EndlessShore::Wellchurch => true,
            EndlessShore::Woodbind => true,
        }
    }

    fn coords(&self) -> (f64, f64) {
        match self {
            EndlessShore::BalorsCrown => (0.8295298, 0.33451718),
            EndlessShore::BatteredLanding => (0.3055171, 0.55011296),
            EndlessShore::BrackishPoint => (0.30299953, 0.15477471),
            EndlessShore::DannanRidge => (0.7488712, 0.22403175),
            EndlessShore::DeargsFang => (0.59473014, 0.36644128),
            EndlessShore::EnduringWake => (0.20581831, 0.6122418),
            EndlessShore::IronJunction => (0.35952666, 0.7956661),
            EndlessShore::KelpiesMane => (0.7760368, 0.63521427),
            EndlessShore::KelpiesTail => (0.7194254, 0.67486936),
            EndlessShore::Liegehearth => (0.30281493, 0.4512878),
            EndlessShore::MerrowsRest => (0.78932536, 0.77107877),
            EndlessShore::SaltbrookChannel => (0.303757, 0.38162574),
            EndlessShore::S??dheFall => (0.48438844, 0.29702434),
            EndlessShore::TheDannanCoast => (0.67474216, 0.2548824),
            EndlessShore::TheDarkRoad => (0.8369384, 0.49502707),
            EndlessShore::TheEvilEye => (0.74394375, 0.45933223),
            EndlessShore::TheNorthStar => (0.30252346, 0.26164305),
            EndlessShore::TheOldJackTar => (0.12129593, 0.50606096),
            EndlessShore::TheOverland => (0.57504135, 0.79666007),
            EndlessShore::TheSelkieBluffs => (0.57922477, 0.54058766),
            EndlessShore::TheStyx => (0.464723, 0.6391649),
            EndlessShore::TheWhisperingWaves => (0.66084594, 0.5801161),
            EndlessShore::TuathaWatchpost => (0.853922, 0.5507405),
            EndlessShore::VulpineWatch => (0.62842077, 0.1475563),
            EndlessShore::Wellchurch => (0.5146297, 0.45945293),
            EndlessShore::Woodbind => (0.6993119, 0.7336695),
        }
    }

    fn name_api(&self) -> (&str, &str) {
        (
            "EndlessShore",
            match self {
                EndlessShore::BalorsCrown => "BalorsCrown",
                EndlessShore::BatteredLanding => "BatteredLanding",
                EndlessShore::BrackishPoint => "BrackishPoint",
                EndlessShore::DannanRidge => "DannanRidge",
                EndlessShore::DeargsFang => "DeargsFang",
                EndlessShore::EnduringWake => "EnduringWake",
                EndlessShore::IronJunction => "IronJunction",
                EndlessShore::KelpiesMane => "KelpiesMane",
                EndlessShore::KelpiesTail => "KelpiesTail",
                EndlessShore::Liegehearth => "Liegehearth",
                EndlessShore::MerrowsRest => "MerrowsRest",
                EndlessShore::SaltbrookChannel => "SaltbrookChannel",
                EndlessShore::S??dheFall => "S??dheFall",
                EndlessShore::TheDannanCoast => "TheDannanCoast",
                EndlessShore::TheDarkRoad => "TheDarkRoad",
                EndlessShore::TheEvilEye => "TheEvilEye",
                EndlessShore::TheNorthStar => "TheNorthStar",
                EndlessShore::TheOldJackTar => "TheOldJackTar",
                EndlessShore::TheOverland => "TheOverland",
                EndlessShore::TheSelkieBluffs => "TheSelkieBluffs",
                EndlessShore::TheStyx => "TheStyx",
                EndlessShore::TheWhisperingWaves => "TheWhisperingWaves",
                EndlessShore::TuathaWatchpost => "TuathaWatchpost",
                EndlessShore::VulpineWatch => "VulpineWatch",
                EndlessShore::Wellchurch => "Wellchurch",
                EndlessShore::Woodbind => "Woodbind",
            },
        )
    }

    fn name_friendly(&self) -> (&str, &str) {
        (
            "EndlessShore",
            match self {
                EndlessShore::BalorsCrown => "Balor's Crown",
                EndlessShore::BatteredLanding => "Battered Landing",
                EndlessShore::BrackishPoint => "Brackish Point",
                EndlessShore::DannanRidge => "Dannan Ridge",
                EndlessShore::DeargsFang => "Dearg's Fang",
                EndlessShore::EnduringWake => "Enduring Wake",
                EndlessShore::IronJunction => "Iron Junction",
                EndlessShore::KelpiesMane => "Kelpie's Mane",
                EndlessShore::KelpiesTail => "Kelpie's Tail",
                EndlessShore::Liegehearth => "Liegehearth",
                EndlessShore::MerrowsRest => "Merrow's Rest",
                EndlessShore::SaltbrookChannel => "Saltbrook Channel",
                EndlessShore::S??dheFall => "S??dhe Fall",
                EndlessShore::TheDannanCoast => "The Dannan Coast",
                EndlessShore::TheDarkRoad => "The Dark Road",
                EndlessShore::TheEvilEye => "The Evil Eye",
                EndlessShore::TheNorthStar => "The North Star",
                EndlessShore::TheOldJackTar => "The Old Jack Tar",
                EndlessShore::TheOverland => "The Overland",
                EndlessShore::TheSelkieBluffs => "The Selkie Bluffs",
                EndlessShore::TheStyx => "The Styx",
                EndlessShore::TheWhisperingWaves => "The Whispering Waves",
                EndlessShore::TuathaWatchpost => "Tuatha Watchpost",
                EndlessShore::VulpineWatch => "Vulpine Watch",
                EndlessShore::Wellchurch => "Wellchurch",
                EndlessShore::Woodbind => "Woodbind",
            },
        )
    }
}

/// Specific map details for the `GodcroftsHex` tile
pub enum Godcrofts {
    /// Anchor Beach is a major location
    AnchorBeach,
    /// Argosa is a minor location
    Argosa,
    /// Bagh M??r is a minor location
    BaghM??r,
    /// Barreller's Bay is a minor location
    BarrellersBay,
    /// Blackwatch is a major location
    Blackwatch,
    /// Chamil Ravine is a minor location
    ChamilRavine,
    /// Den of Thieves is a minor location
    DenofThieves,
    /// Exile is a minor location
    Exile,
    /// Isawa is a major location
    Isawa,
    /// Kolas is a minor location
    Kolas,
    /// Lipsia is a minor location
    Lipsia,
    /// Peripti Depths is a minor location
    PeriptiDepths,
    /// Perpetua Channel is a minor location
    PerpetuaChannel,
    /// Primus Trames is a minor location
    PrimusTrames,
    /// Promithiens is a major location
    Promithiens,
    /// Protos is a major location
    Protos,
    /// Saegio is a minor location
    Saegio,
    /// Skodio is a major location
    Skodio,
    /// The Axehead is a major location
    TheAxehead,
    /// The Dice Road is a minor location
    TheDiceRoad,
    /// The Fleece Road is a major location
    TheFleeceRoad,
    /// The Kris Ford is a minor location
    TheKrisFord,
    /// Ursa Trail is a minor location
    UrsaTrail,
    /// Vicit Lagoon is a minor location
    VicitLagoon,
}

impl Location for Godcrofts {
    fn is_major(&self) -> bool {
        match self {
            Godcrofts::AnchorBeach => true,
            Godcrofts::Argosa => false,
            Godcrofts::BaghM??r => false,
            Godcrofts::BarrellersBay => false,
            Godcrofts::Blackwatch => true,
            Godcrofts::ChamilRavine => false,
            Godcrofts::DenofThieves => false,
            Godcrofts::Exile => false,
            Godcrofts::Isawa => true,
            Godcrofts::Kolas => false,
            Godcrofts::Lipsia => false,
            Godcrofts::PeriptiDepths => false,
            Godcrofts::PerpetuaChannel => false,
            Godcrofts::PrimusTrames => false,
            Godcrofts::Promithiens => true,
            Godcrofts::Protos => true,
            Godcrofts::Saegio => false,
            Godcrofts::Skodio => true,
            Godcrofts::TheAxehead => true,
            Godcrofts::TheDiceRoad => false,
            Godcrofts::TheFleeceRoad => true,
            Godcrofts::TheKrisFord => false,
            Godcrofts::UrsaTrail => false,
            Godcrofts::VicitLagoon => false,
        }
    }

    fn coords(&self) -> (f64, f64) {
        match self {
            Godcrofts::AnchorBeach => (0.33879337, 0.91907495),
            Godcrofts::Argosa => (0.4455359, 0.4629214),
            Godcrofts::BaghM??r => (0.45754752, 0.8384563),
            Godcrofts::BarrellersBay => (0.65878975, 0.45934907),
            Godcrofts::Blackwatch => (0.3415074, 0.7991088),
            Godcrofts::ChamilRavine => (0.47395247, 0.30361375),
            Godcrofts::DenofThieves => (0.3221958, 0.63976383),
            Godcrofts::Exile => (0.41336396, 0.6418172),
            Godcrofts::Isawa => (0.62136436, 0.80811846),
            Godcrofts::Kolas => (0.1527959, 0.64679915),
            Godcrofts::Lipsia => (0.45892027, 0.21390341),
            Godcrofts::PeriptiDepths => (0.26073462, 0.75151116),
            Godcrofts::PerpetuaChannel => (0.11155792, 0.34458795),
            Godcrofts::PrimusTrames => (0.321166, 0.47223058),
            Godcrofts::Promithiens => (0.541576, 0.6364856),
            Godcrofts::Protos => (0.1703909, 0.5030786),
            Godcrofts::Saegio => (0.4568375, 0.42920688),
            Godcrofts::Skodio => (0.5933204, 0.24777809),
            Godcrofts::TheAxehead => (0.24177985, 0.33301017),
            Godcrofts::TheDiceRoad => (0.5630271, 0.33521596),
            Godcrofts::TheFleeceRoad => (0.6615385, 0.8981667),
            Godcrofts::TheKrisFord => (0.5956406, 0.72609246),
            Godcrofts::UrsaTrail => (0.5877143, 0.94025785),
            Godcrofts::VicitLagoon => (0.33476177, 0.36637142),
        }
    }

    fn name_api(&self) -> (&str, &str) {
        (
            "Godcrofts",
            match self {
                Godcrofts::AnchorBeach => "AnchorBeach",
                Godcrofts::Argosa => "Argosa",
                Godcrofts::BaghM??r => "BaghM??r",
                Godcrofts::BarrellersBay => "BarrellersBay",
                Godcrofts::Blackwatch => "Blackwatch",
                Godcrofts::ChamilRavine => "ChamilRavine",
                Godcrofts::DenofThieves => "DenofThieves",
                Godcrofts::Exile => "Exile",
                Godcrofts::Isawa => "Isawa",
                Godcrofts::Kolas => "Kolas",
                Godcrofts::Lipsia => "Lipsia",
                Godcrofts::PeriptiDepths => "PeriptiDepths",
                Godcrofts::PerpetuaChannel => "PerpetuaChannel",
                Godcrofts::PrimusTrames => "PrimusTrames",
                Godcrofts::Promithiens => "Promithiens",
                Godcrofts::Protos => "Protos",
                Godcrofts::Saegio => "Saegio",
                Godcrofts::Skodio => "Skodio",
                Godcrofts::TheAxehead => "TheAxehead",
                Godcrofts::TheDiceRoad => "TheDiceRoad",
                Godcrofts::TheFleeceRoad => "TheFleeceRoad",
                Godcrofts::TheKrisFord => "TheKrisFord",
                Godcrofts::UrsaTrail => "UrsaTrail",
                Godcrofts::VicitLagoon => "VicitLagoon",
            },
        )
    }

    fn name_friendly(&self) -> (&str, &str) {
        (
            "Godcrofts",
            match self {
                Godcrofts::AnchorBeach => "Anchor Beach",
                Godcrofts::Argosa => "Argosa",
                Godcrofts::BaghM??r => "Bagh M??r",
                Godcrofts::BarrellersBay => "Barreller's Bay",
                Godcrofts::Blackwatch => "Blackwatch",
                Godcrofts::ChamilRavine => "Chamil Ravine",
                Godcrofts::DenofThieves => "Den of Thieves",
                Godcrofts::Exile => "Exile",
                Godcrofts::Isawa => "Isawa",
                Godcrofts::Kolas => "Kolas",
                Godcrofts::Lipsia => "Lipsia",
                Godcrofts::PeriptiDepths => "Peripti Depths",
                Godcrofts::PerpetuaChannel => "Perpetua Channel",
                Godcrofts::PrimusTrames => "Primus Trames",
                Godcrofts::Promithiens => "Promithiens",
                Godcrofts::Protos => "Protos",
                Godcrofts::Saegio => "Saegio",
                Godcrofts::Skodio => "Skodio",
                Godcrofts::TheAxehead => "The Axehead",
                Godcrofts::TheDiceRoad => "The Dice Road",
                Godcrofts::TheFleeceRoad => "The Fleece Road",
                Godcrofts::TheKrisFord => "The Kris Ford",
                Godcrofts::UrsaTrail => "Ursa Trail",
                Godcrofts::VicitLagoon => "Vicit Lagoon",
            },
        )
    }
}

/// Specific map details for the `FishermansRowHex` tile
pub enum FishermansRow {
    /// A Lost Sot is a minor location
    ALostSot,
    /// Arcadia is a major location
    Arcadia,
    /// Bident Crossroads is a minor location
    BidentCrossroads,
    /// Black Well is a major location
    BlackWell,
    /// Cat Step is a minor location
    CatStep,
    /// Dankana Post is a minor location
    DankanaPost,
    /// Eidolo is a major location
    Eidolo,
    /// Fort Ember is a major location
    FortEmber,
    /// Hangmen's Court is a major location
    HangmensCourt,
    /// Heart of Rites is a minor location
    HeartofRites,
    /// House Roloi is a minor location
    HouseRoloi,
    /// Lake Nerites is a minor location
    LakeNerites,
    /// Liberty Hill is a minor location
    LibertyHill,
    /// Oceanwatch is a major location
    Oceanwatch,
    /// Partisan Island is a major location
    PartisanIsland,
    /// Peripti Landing is a major location
    PeriptiLanding,
    /// Progonos Watch is a minor location
    ProgonosWatch,
    /// The Dire Strings is a minor location
    TheDireStrings,
    /// The Rite Road is a minor location
    TheRiteRoad,
    /// The Satyr Stone is a major location
    TheSatyrStone,
    /// The Three Sisters is a major location
    TheThreeSisters,
    /// Torch of Demeter is a minor location
    TorchofDemeter,
}

impl Location for FishermansRow {
    fn is_major(&self) -> bool {
        match self {
            FishermansRow::ALostSot => false,
            FishermansRow::Arcadia => true,
            FishermansRow::BidentCrossroads => false,
            FishermansRow::BlackWell => true,
            FishermansRow::CatStep => false,
            FishermansRow::DankanaPost => false,
            FishermansRow::Eidolo => true,
            FishermansRow::FortEmber => true,
            FishermansRow::HangmensCourt => true,
            FishermansRow::HeartofRites => false,
            FishermansRow::HouseRoloi => false,
            FishermansRow::LakeNerites => false,
            FishermansRow::LibertyHill => false,
            FishermansRow::Oceanwatch => true,
            FishermansRow::PartisanIsland => true,
            FishermansRow::PeriptiLanding => true,
            FishermansRow::ProgonosWatch => false,
            FishermansRow::TheDireStrings => false,
            FishermansRow::TheRiteRoad => false,
            FishermansRow::TheSatyrStone => true,
            FishermansRow::TheThreeSisters => true,
            FishermansRow::TorchofDemeter => false,
        }
    }

    fn coords(&self) -> (f64, f64) {
        match self {
            FishermansRow::ALostSot => (0.90655476, 0.49344444),
            FishermansRow::Arcadia => (0.38920873, 0.73095393),
            FishermansRow::BidentCrossroads => (0.43450263, 0.6158199),
            FishermansRow::BlackWell => (0.38849574, 0.3066156),
            FishermansRow::CatStep => (0.6206566, 0.032853466),
            FishermansRow::DankanaPost => (0.30517027, 0.6366062),
            FishermansRow::Eidolo => (0.46337467, 0.52302),
            FishermansRow::FortEmber => (0.50422376, 0.70527685),
            FishermansRow::HangmensCourt => (0.77422845, 0.5360584),
            FishermansRow::HeartofRites => (0.35531422, 0.46871832),
            FishermansRow::HouseRoloi => (0.26010418, 0.77387524),
            FishermansRow::LakeNerites => (0.32699662, 0.54289067),
            FishermansRow::LibertyHill => (0.3673808, 0.35783792),
            FishermansRow::Oceanwatch => (0.53076994, 0.3338183),
            FishermansRow::PartisanIsland => (0.6703404, 0.20176351),
            FishermansRow::PeriptiLanding => (0.29418412, 0.4540274),
            FishermansRow::ProgonosWatch => (0.50026, 0.4351574),
            FishermansRow::TheDireStrings => (0.8424306, 0.6141279),
            FishermansRow::TheRiteRoad => (0.45041052, 0.38507676),
            FishermansRow::TheSatyrStone => (0.67093605, 0.6439194),
            FishermansRow::TheThreeSisters => (0.684986, 0.3528056),
            FishermansRow::TorchofDemeter => (0.42485577, 0.6818831),
        }
    }

    fn name_api(&self) -> (&str, &str) {
        (
            "FishermansRow",
            match self {
                FishermansRow::ALostSot => "ALostSot",
                FishermansRow::Arcadia => "Arcadia",
                FishermansRow::BidentCrossroads => "BidentCrossroads",
                FishermansRow::BlackWell => "BlackWell",
                FishermansRow::CatStep => "CatStep",
                FishermansRow::DankanaPost => "DankanaPost",
                FishermansRow::Eidolo => "Eidolo",
                FishermansRow::FortEmber => "FortEmber",
                FishermansRow::HangmensCourt => "HangmensCourt",
                FishermansRow::HeartofRites => "HeartofRites",
                FishermansRow::HouseRoloi => "HouseRoloi",
                FishermansRow::LakeNerites => "LakeNerites",
                FishermansRow::LibertyHill => "LibertyHill",
                FishermansRow::Oceanwatch => "Oceanwatch",
                FishermansRow::PartisanIsland => "PartisanIsland",
                FishermansRow::PeriptiLanding => "PeriptiLanding",
                FishermansRow::ProgonosWatch => "ProgonosWatch",
                FishermansRow::TheDireStrings => "TheDireStrings",
                FishermansRow::TheRiteRoad => "TheRiteRoad",
                FishermansRow::TheSatyrStone => "TheSatyrStone",
                FishermansRow::TheThreeSisters => "TheThreeSisters",
                FishermansRow::TorchofDemeter => "TorchofDemeter",
            },
        )
    }

    fn name_friendly(&self) -> (&str, &str) {
        (
            "FishermansRow",
            match self {
                FishermansRow::ALostSot => "A Lost Sot",
                FishermansRow::Arcadia => "Arcadia",
                FishermansRow::BidentCrossroads => "Bident Crossroads",
                FishermansRow::BlackWell => "Black Well",
                FishermansRow::CatStep => "Cat Step",
                FishermansRow::DankanaPost => "Dankana Post",
                FishermansRow::Eidolo => "Eidolo",
                FishermansRow::FortEmber => "Fort Ember",
                FishermansRow::HangmensCourt => "Hangmen's Court",
                FishermansRow::HeartofRites => "Heart of Rites",
                FishermansRow::HouseRoloi => "House Roloi",
                FishermansRow::LakeNerites => "Lake Nerites",
                FishermansRow::LibertyHill => "Liberty Hill",
                FishermansRow::Oceanwatch => "Oceanwatch",
                FishermansRow::PartisanIsland => "Partisan Island",
                FishermansRow::PeriptiLanding => "Peripti Landing",
                FishermansRow::ProgonosWatch => "Progonos Watch",
                FishermansRow::TheDireStrings => "The Dire Strings",
                FishermansRow::TheRiteRoad => "The Rite Road",
                FishermansRow::TheSatyrStone => "The Satyr Stone",
                FishermansRow::TheThreeSisters => "The Three Sisters",
                FishermansRow::TorchofDemeter => "Torch of Demeter",
            },
        )
    }
}

/// Specific map details for the `WestgateHex` tile
pub enum Westgate {
    /// Ash Step is a minor location
    AshStep,
    /// Candle Hills is a minor location
    CandleHills,
    /// Cattle March is a minor location
    CattleMarch,
    /// Ceo Highlands is a minor location
    CeoHighlands,
    /// Cinder Road is a minor location
    CinderRoad,
    /// Coasthill is a minor location
    Coasthill,
    /// Coastway is a minor location
    Coastway,
    /// Cobber's Lane is a minor location
    CobbersLane,
    /// Ember Hills is a minor location
    EmberHills,
    /// Fand's Chain is a minor location
    FandsChain,
    /// Fields of Badb is a minor location
    FieldsofBadb,
    /// Flidais' Pasture is a minor location
    FlidaisPasture,
    /// Handsome Hideaway is a minor location
    HandsomeHideaway,
    /// Hillcrest is a minor location
    Hillcrest,
    /// Holdfast is a major location
    Holdfast,
    /// Inkwell Lane is a minor location
    InkwellLane,
    /// Kardia Road is a minor location
    KardiaRoad,
    /// Killian Quarter is a minor location
    KillianQuarter,
    /// Kingstone is a major location
    Kingstone,
    /// Longstone is a major location
    Longstone,
    /// Lord's Mouth is a major location
    LordsMouth,
    /// Lost Partition is a major location
    LostPartition,
    /// Rancher's Fast is a major location
    RanchersFast,
    /// Reaver's Cove is a minor location
    ReaversCove,
    /// Sanctified Path is a minor location
    SanctifiedPath,
    /// Sanctuary is a major location
    Sanctuary,
    /// S??och??na Valley is a minor location
    S??och??naValley,
    /// Taswell Point is a minor location
    TaswellPoint,
    /// The Aging Ocean is a minor location
    TheAgingOcean,
    /// The Bulwark is a minor location
    TheBulwark,
    /// The Divide is a minor location
    TheDivide,
    /// The Gallows is a major location
    TheGallows,
    /// The Hem is a minor location
    TheHem,
    /// The King's Road is a minor location
    TheKingsRoad,
    /// The Knight's Edge is a minor location
    TheKnightsEdge,
    /// Triton's Curse is a minor location
    TritonsCurse,
    /// Warden Walk is a minor location
    WardenWalk,
    /// Western Heartlands is a minor location
    WesternHeartlands,
    /// Westgate Keep is a major location
    WestgateKeep,
    /// Wire Road is a minor location
    WireRoad,
    /// Wyattwick is a major location
    Wyattwick,
    /// Zeus' Demise is a major location
    ZeusDemise,
}

impl Location for Westgate {
    fn is_major(&self) -> bool {
        match self {
            Westgate::AshStep => false,
            Westgate::CandleHills => false,
            Westgate::CattleMarch => false,
            Westgate::CeoHighlands => false,
            Westgate::CinderRoad => false,
            Westgate::Coasthill => false,
            Westgate::Coastway => false,
            Westgate::CobbersLane => false,
            Westgate::EmberHills => false,
            Westgate::FandsChain => false,
            Westgate::FieldsofBadb => false,
            Westgate::FlidaisPasture => false,
            Westgate::HandsomeHideaway => false,
            Westgate::Hillcrest => false,
            Westgate::Holdfast => true,
            Westgate::InkwellLane => false,
            Westgate::KardiaRoad => false,
            Westgate::KillianQuarter => false,
            Westgate::Kingstone => true,
            Westgate::Longstone => true,
            Westgate::LordsMouth => true,
            Westgate::LostPartition => true,
            Westgate::RanchersFast => true,
            Westgate::ReaversCove => false,
            Westgate::SanctifiedPath => false,
            Westgate::Sanctuary => true,
            Westgate::S??och??naValley => false,
            Westgate::TaswellPoint => false,
            Westgate::TheAgingOcean => false,
            Westgate::TheBulwark => false,
            Westgate::TheDivide => false,
            Westgate::TheGallows => true,
            Westgate::TheHem => false,
            Westgate::TheKingsRoad => false,
            Westgate::TheKnightsEdge => false,
            Westgate::TritonsCurse => false,
            Westgate::WardenWalk => false,
            Westgate::WesternHeartlands => false,
            Westgate::WestgateKeep => true,
            Westgate::WireRoad => false,
            Westgate::Wyattwick => true,
            Westgate::ZeusDemise => true,
        }
    }

    fn coords(&self) -> (f64, f64) {
        match self {
            Westgate::AshStep => (0.64588916, 1.041789),
            Westgate::CandleHills => (0.3809478, 0.876076),
            Westgate::CattleMarch => (0.43200865, 0.178799),
            Westgate::CeoHighlands => (0.6742479, 0.9047628),
            Westgate::CinderRoad => (0.41563344, 1.0379049),
            Westgate::Coasthill => (0.7492962, 0.5232728),
            Westgate::Coastway => (0.5137713, 0.98847944),
            Westgate::CobbersLane => (0.81818783, 0.3048373),
            Westgate::EmberHills => (0.555335, 1.0591571),
            Westgate::FandsChain => (0.25625902, 0.14081238),
            Westgate::FieldsofBadb => (0.51419544, 0.15018566),
            Westgate::FlidaisPasture => (0.6924164, 0.22017962),
            Westgate::HandsomeHideaway => (0.33405593, 0.4992943),
            Westgate::Hillcrest => (0.64205915, 0.6253394),
            Westgate::Holdfast => (0.46443552, 0.27771717),
            Westgate::InkwellLane => (0.5928621, 0.10583429),
            Westgate::KardiaRoad => (0.5908896, 0.178799),
            Westgate::KillianQuarter => (0.7891662, 0.16007636),
            Westgate::Kingstone => (0.64355624, 0.41088805),
            Westgate::Longstone => (0.6454746, 0.7234395),
            Westgate::LordsMouth => (0.82933867, 0.4307491),
            Westgate::LostPartition => (0.8150013, 0.5756538),
            Westgate::RanchersFast => (0.69447815, 0.18181983),
            Westgate::ReaversCove => (0.4750435, 0.5243521),
            Westgate::SanctifiedPath => (0.63349, 0.8222307),
            Westgate::Sanctuary => (0.23537217, 0.26691732),
            Westgate::S??och??naValley => (0.5804643, 0.8998509),
            Westgate::TaswellPoint => (0.54965186, 0.79319614),
            Westgate::TheAgingOcean => (0.114025585, 0.663528),
            Westgate::TheBulwark => (0.76893073, 0.6432597),
            Westgate::TheDivide => (0.48302627, 0.8998509),
            Westgate::TheGallows => (0.3637351, 0.1061458),
            Westgate::TheHem => (0.7906987, 0.74425286),
            Westgate::TheKingsRoad => (0.7325495, 0.4098496),
            Westgate::TheKnightsEdge => (0.579454, 0.28919458),
            Westgate::TritonsCurse => (0.2009395, 0.88355315),
            Westgate::WardenWalk => (0.8698958, 0.5327038),
            Westgate::WesternHeartlands => (0.7199143, 0.343739),
            Westgate::WestgateKeep => (0.59599173, 0.56367874),
            Westgate::WireRoad => (0.69118434, 0.2610858),
            Westgate::Wyattwick => (0.29039592, 0.81640434),
            Westgate::ZeusDemise => (0.4019617, 0.7723668),
        }
    }

    fn name_api(&self) -> (&str, &str) {
        (
            "Westgate",
            match self {
                Westgate::AshStep => "AshStep",
                Westgate::CandleHills => "CandleHills",
                Westgate::CattleMarch => "CattleMarch",
                Westgate::CeoHighlands => "CeoHighlands",
                Westgate::CinderRoad => "CinderRoad",
                Westgate::Coasthill => "Coasthill",
                Westgate::Coastway => "Coastway",
                Westgate::CobbersLane => "CobbersLane",
                Westgate::EmberHills => "EmberHills",
                Westgate::FandsChain => "FandsChain",
                Westgate::FieldsofBadb => "FieldsofBadb",
                Westgate::FlidaisPasture => "FlidaisPasture",
                Westgate::HandsomeHideaway => "HandsomeHideaway",
                Westgate::Hillcrest => "Hillcrest",
                Westgate::Holdfast => "Holdfast",
                Westgate::InkwellLane => "InkwellLane",
                Westgate::KardiaRoad => "KardiaRoad",
                Westgate::KillianQuarter => "KillianQuarter",
                Westgate::Kingstone => "Kingstone",
                Westgate::Longstone => "Longstone",
                Westgate::LordsMouth => "LordsMouth",
                Westgate::LostPartition => "LostPartition",
                Westgate::RanchersFast => "RanchersFast",
                Westgate::ReaversCove => "ReaversCove",
                Westgate::SanctifiedPath => "SanctifiedPath",
                Westgate::Sanctuary => "Sanctuary",
                Westgate::S??och??naValley => "S??och??naValley",
                Westgate::TaswellPoint => "TaswellPoint",
                Westgate::TheAgingOcean => "TheAgingOcean",
                Westgate::TheBulwark => "TheBulwark",
                Westgate::TheDivide => "TheDivide",
                Westgate::TheGallows => "TheGallows",
                Westgate::TheHem => "TheHem",
                Westgate::TheKingsRoad => "TheKingsRoad",
                Westgate::TheKnightsEdge => "TheKnightsEdge",
                Westgate::TritonsCurse => "TritonsCurse",
                Westgate::WardenWalk => "WardenWalk",
                Westgate::WesternHeartlands => "WesternHeartlands",
                Westgate::WestgateKeep => "WestgateKeep",
                Westgate::WireRoad => "WireRoad",
                Westgate::Wyattwick => "Wyattwick",
                Westgate::ZeusDemise => "ZeusDemise",
            },
        )
    }

    fn name_friendly(&self) -> (&str, &str) {
        (
            "Westgate",
            match self {
                Westgate::AshStep => "Ash Step",
                Westgate::CandleHills => "Candle Hills",
                Westgate::CattleMarch => "Cattle March",
                Westgate::CeoHighlands => "Ceo Highlands",
                Westgate::CinderRoad => "Cinder Road",
                Westgate::Coasthill => "Coasthill",
                Westgate::Coastway => "Coastway",
                Westgate::CobbersLane => "Cobber's Lane",
                Westgate::EmberHills => "Ember Hills",
                Westgate::FandsChain => "Fand's Chain",
                Westgate::FieldsofBadb => "Fields of Badb",
                Westgate::FlidaisPasture => "Flidais' Pasture",
                Westgate::HandsomeHideaway => "Handsome Hideaway",
                Westgate::Hillcrest => "Hillcrest",
                Westgate::Holdfast => "Holdfast",
                Westgate::InkwellLane => "Inkwell Lane",
                Westgate::KardiaRoad => "Kardia Road",
                Westgate::KillianQuarter => "Killian Quarter",
                Westgate::Kingstone => "Kingstone",
                Westgate::Longstone => "Longstone",
                Westgate::LordsMouth => "Lord's Mouth",
                Westgate::LostPartition => "Lost Partition",
                Westgate::RanchersFast => "Rancher's Fast",
                Westgate::ReaversCove => "Reaver's Cove",
                Westgate::SanctifiedPath => "Sanctified Path",
                Westgate::Sanctuary => "Sanctuary",
                Westgate::S??och??naValley => "S??och??na Valley",
                Westgate::TaswellPoint => "Taswell Point",
                Westgate::TheAgingOcean => "The Aging Ocean",
                Westgate::TheBulwark => "The Bulwark",
                Westgate::TheDivide => "The Divide",
                Westgate::TheGallows => "The Gallows",
                Westgate::TheHem => "The Hem",
                Westgate::TheKingsRoad => "The King's Road",
                Westgate::TheKnightsEdge => "The Knight's Edge",
                Westgate::TritonsCurse => "Triton's Curse",
                Westgate::WardenWalk => "Warden Walk",
                Westgate::WesternHeartlands => "Western Heartlands",
                Westgate::WestgateKeep => "Westgate Keep",
                Westgate::WireRoad => "Wire Road",
                Westgate::Wyattwick => "Wyattwick",
                Westgate::ZeusDemise => "Zeus' Demise",
            },
        )
    }
}

/// Specific map details for the `ReachingTrailHex` tile
pub enum ReachingTrail {
    /// Brodytown is a major location
    Brodytown,
    /// Camp Eos is a minor location
    CampEos,
    /// Caragtais is a minor location
    Caragtais,
    /// Duffy's Farm is a minor location
    DuffysFarm,
    /// Dugan's Approach is a minor location
    DugansApproach,
    /// Dwyersfield is a minor location
    Dwyersfield,
    /// Dwyerstown is a major location
    Dwyerstown,
    /// Elksford is a major location
    Elksford,
    /// Featherfield is a minor location
    Featherfield,
    /// Fisherman's Floe is a minor location
    FishermansFloe,
    /// Fort Mac Conaill is a major location
    FortMacConaill,
    /// Harpy is a major location
    Harpy,
    /// Hookhall is a minor location
    Hookhall,
    /// Humidus is a minor location
    Humidus,
    /// Ice Ranch is a major location
    IceRanch,
    /// Limestone Holdfast is a minor location
    LimestoneHoldfast,
    /// Mac Conaill's Pass is a minor location
    MacConaillsPass,
    /// Mousetrap is a major location
    Mousetrap,
    /// Nightchurch is a major location
    Nightchurch,
    /// Pitfall is a minor location
    Pitfall,
    /// Puncta is a minor location
    Puncta,
    /// Reprieve is a major location
    Reprieve,
    /// Scorpion is a minor location
    Scorpion,
    /// The Ark is a major location
    TheArk,
    /// The Bait is a minor location
    TheBait,
    /// The Cairns is a minor location
    TheCairns,
    /// The Chicken Coop is a minor location
    TheChickenCoop,
    /// The Deckard is a minor location
    TheDeckard,
    /// The Knot is a minor location
    TheKnot,
    /// The Reaching Heights is a minor location
    TheReachingHeights,
    /// The Rime Ledge is a minor location
    TheRimeLedge,
    /// The Rousing Fields is a minor location
    TheRousingFields,
    /// The Scar is a minor location
    TheScar,
    /// The Squeeze is a minor location
    TheSqueeze,
    /// Th??lak is a minor location
    Th??lak,
    /// Windy Way is a minor location
    WindyWay,
}

impl Location for ReachingTrail {
    fn is_major(&self) -> bool {
        match self {
            ReachingTrail::Brodytown => true,
            ReachingTrail::CampEos => false,
            ReachingTrail::Caragtais => false,
            ReachingTrail::DuffysFarm => false,
            ReachingTrail::DugansApproach => false,
            ReachingTrail::Dwyersfield => false,
            ReachingTrail::Dwyerstown => true,
            ReachingTrail::Elksford => true,
            ReachingTrail::Featherfield => false,
            ReachingTrail::FishermansFloe => false,
            ReachingTrail::FortMacConaill => true,
            ReachingTrail::Harpy => true,
            ReachingTrail::Hookhall => false,
            ReachingTrail::Humidus => false,
            ReachingTrail::IceRanch => true,
            ReachingTrail::LimestoneHoldfast => false,
            ReachingTrail::MacConaillsPass => false,
            ReachingTrail::Mousetrap => true,
            ReachingTrail::Nightchurch => true,
            ReachingTrail::Pitfall => false,
            ReachingTrail::Puncta => false,
            ReachingTrail::Reprieve => true,
            ReachingTrail::Scorpion => false,
            ReachingTrail::TheArk => true,
            ReachingTrail::TheBait => false,
            ReachingTrail::TheCairns => false,
            ReachingTrail::TheChickenCoop => false,
            ReachingTrail::TheDeckard => false,
            ReachingTrail::TheKnot => false,
            ReachingTrail::TheReachingHeights => false,
            ReachingTrail::TheRimeLedge => false,
            ReachingTrail::TheRousingFields => false,
            ReachingTrail::TheScar => false,
            ReachingTrail::TheSqueeze => false,
            ReachingTrail::Th??lak => false,
            ReachingTrail::WindyWay => false,
        }
    }

    fn coords(&self) -> (f64, f64) {
        match self {
            ReachingTrail::Brodytown => (0.44412765, 0.3227743),
            ReachingTrail::CampEos => (0.7520739, 0.70374876),
            ReachingTrail::Caragtais => (0.45057085, 0.5294064),
            ReachingTrail::DuffysFarm => (0.2719318, 0.56807905),
            ReachingTrail::DugansApproach => (0.6922709, 0.1538672),
            ReachingTrail::Dwyersfield => (0.2087289, 0.26713523),
            ReachingTrail::Dwyerstown => (0.15061852, 0.36391303),
            ReachingTrail::Elksford => (0.33049652, 0.4971746),
            ReachingTrail::Featherfield => (0.4406717, 0.798611),
            ReachingTrail::FishermansFloe => (0.6400806, 0.5572313),
            ReachingTrail::FortMacConaill => (0.70212996, 0.8482764),
            ReachingTrail::Harpy => (0.44333068, 0.9163655),
            ReachingTrail::Hookhall => (0.36672696, 0.6498256),
            ReachingTrail::Humidus => (0.27232808, 0.66734344),
            ReachingTrail::IceRanch => (0.70420206, 0.3878927),
            ReachingTrail::LimestoneHoldfast => (0.27798548, 0.16184786),
            ReachingTrail::MacConaillsPass => (0.61724085, 0.985965),
            ReachingTrail::Mousetrap => (0.8401006, 0.53316337),
            ReachingTrail::Nightchurch => (0.37822467, 0.74487805),
            ReachingTrail::Pitfall => (0.87784535, 0.37858298),
            ReachingTrail::Puncta => (0.18530942, 0.5888269),
            ReachingTrail::Reprieve => (0.54468614, 0.67606056),
            ReachingTrail::Scorpion => (0.33854625, 0.86316854),
            ReachingTrail::TheArk => (0.74158484, 0.5578975),
            ReachingTrail::TheBait => (0.708999, 0.5577328),
            ReachingTrail::TheCairns => (0.60312855, 0.23888308),
            ReachingTrail::TheChickenCoop => (0.32071957, 0.4145506),
            ReachingTrail::TheDeckard => (0.4275156, 0.041210115),
            ReachingTrail::TheKnot => (0.44904447, 0.6583204),
            ReachingTrail::TheReachingHeights => (0.546316, 0.547519),
            ReachingTrail::TheRimeLedge => (0.89936477, 0.74528646),
            ReachingTrail::TheRousingFields => (0.23695965, 0.46786177),
            ReachingTrail::TheScar => (0.29426545, 0.5436782),
            ReachingTrail::TheSqueeze => (0.3175878, 1.0565143),
            ReachingTrail::Th??lak => (0.11808629, 0.7405652),
            ReachingTrail::WindyWay => (0.22708872, 0.078611076),
        }
    }

    fn name_api(&self) -> (&str, &str) {
        (
            "ReachingTrail",
            match self {
                ReachingTrail::Brodytown => "Brodytown",
                ReachingTrail::CampEos => "CampEos",
                ReachingTrail::Caragtais => "Caragtais",
                ReachingTrail::DuffysFarm => "DuffysFarm",
                ReachingTrail::DugansApproach => "DugansApproach",
                ReachingTrail::Dwyersfield => "Dwyersfield",
                ReachingTrail::Dwyerstown => "Dwyerstown",
                ReachingTrail::Elksford => "Elksford",
                ReachingTrail::Featherfield => "Featherfield",
                ReachingTrail::FishermansFloe => "FishermansFloe",
                ReachingTrail::FortMacConaill => "FortMacConaill",
                ReachingTrail::Harpy => "Harpy",
                ReachingTrail::Hookhall => "Hookhall",
                ReachingTrail::Humidus => "Humidus",
                ReachingTrail::IceRanch => "IceRanch",
                ReachingTrail::LimestoneHoldfast => "LimestoneHoldfast",
                ReachingTrail::MacConaillsPass => "MacConaillsPass",
                ReachingTrail::Mousetrap => "Mousetrap",
                ReachingTrail::Nightchurch => "Nightchurch",
                ReachingTrail::Pitfall => "Pitfall",
                ReachingTrail::Puncta => "Puncta",
                ReachingTrail::Reprieve => "Reprieve",
                ReachingTrail::Scorpion => "Scorpion",
                ReachingTrail::TheArk => "TheArk",
                ReachingTrail::TheBait => "TheBait",
                ReachingTrail::TheCairns => "TheCairns",
                ReachingTrail::TheChickenCoop => "TheChickenCoop",
                ReachingTrail::TheDeckard => "TheDeckard",
                ReachingTrail::TheKnot => "TheKnot",
                ReachingTrail::TheReachingHeights => "TheReachingHeights",
                ReachingTrail::TheRimeLedge => "TheRimeLedge",
                ReachingTrail::TheRousingFields => "TheRousingFields",
                ReachingTrail::TheScar => "TheScar",
                ReachingTrail::TheSqueeze => "TheSqueeze",
                ReachingTrail::Th??lak => "Th??lak",
                ReachingTrail::WindyWay => "WindyWay",
            },
        )
    }

    fn name_friendly(&self) -> (&str, &str) {
        (
            "ReachingTrail",
            match self {
                ReachingTrail::Brodytown => "Brodytown",
                ReachingTrail::CampEos => "Camp Eos",
                ReachingTrail::Caragtais => "Caragtais",
                ReachingTrail::DuffysFarm => "Duffy's Farm",
                ReachingTrail::DugansApproach => "Dugan's Approach",
                ReachingTrail::Dwyersfield => "Dwyersfield",
                ReachingTrail::Dwyerstown => "Dwyerstown",
                ReachingTrail::Elksford => "Elksford",
                ReachingTrail::Featherfield => "Featherfield",
                ReachingTrail::FishermansFloe => "Fisherman's Floe",
                ReachingTrail::FortMacConaill => "Fort Mac Conaill",
                ReachingTrail::Harpy => "Harpy",
                ReachingTrail::Hookhall => "Hookhall",
                ReachingTrail::Humidus => "Humidus",
                ReachingTrail::IceRanch => "Ice Ranch",
                ReachingTrail::LimestoneHoldfast => "Limestone Holdfast",
                ReachingTrail::MacConaillsPass => "Mac Conaill's Pass",
                ReachingTrail::Mousetrap => "Mousetrap",
                ReachingTrail::Nightchurch => "Nightchurch",
                ReachingTrail::Pitfall => "Pitfall",
                ReachingTrail::Puncta => "Puncta",
                ReachingTrail::Reprieve => "Reprieve",
                ReachingTrail::Scorpion => "Scorpion",
                ReachingTrail::TheArk => "The Ark",
                ReachingTrail::TheBait => "The Bait",
                ReachingTrail::TheCairns => "The Cairns",
                ReachingTrail::TheChickenCoop => "The Chicken Coop",
                ReachingTrail::TheDeckard => "The Deckard",
                ReachingTrail::TheKnot => "The Knot",
                ReachingTrail::TheReachingHeights => "The Reaching Heights",
                ReachingTrail::TheRimeLedge => "The Rime Ledge",
                ReachingTrail::TheRousingFields => "The Rousing Fields",
                ReachingTrail::TheScar => "The Scar",
                ReachingTrail::TheSqueeze => "The Squeeze",
                ReachingTrail::Th??lak => "Th??lak",
                ReachingTrail::WindyWay => "Windy Way",
            },
        )
    }
}

/// Specific map details for the `UmbralWildwoodHex` tile
pub enum UmbralWildwood {
    /// Adze Crossroads is a minor location
    AdzeCrossroads,
    /// Amethyst is a major location
    Amethyst,
    /// Atropos' Fate is a major location
    AtroposFate,
    /// Clotho's Refuge is a major location
    ClothosRefuge,
    /// Dredgefield is a minor location
    Dredgefield,
    /// Golden Concourse is a minor location
    GoldenConcourse,
    /// GoldenRoot Ranch is a major location
    GoldenRootRanch,
    /// Hermit's Rest is a major location
    HermitsRest,
    /// Lachesis' Tally  is a major location
    LachesisTally,
    /// Leatherback Pathway is a minor location
    LeatherbackPathway,
    /// Sentry is a major location
    Sentry,
    /// Steely Fields is a minor location
    SteelyFields,
    /// Stray is a major location
    Stray,
    /// Terrapin Woods is a minor location
    TerrapinWoods,
    /// The Dredgewood is a minor location
    TheDredgewood,
    /// The Foundry is a major location
    TheFoundry,
    /// The Frontier is a minor location
    TheFrontier,
    /// The Gap is a minor location
    TheGap,
    /// The Strands is a minor location
    TheStrands,
    /// Thunder Row is a minor location
    ThunderRow,
    /// Thunderfoot is a major location
    Thunderfoot,
    /// Vagrant Bastion is a major location
    VagrantBastion,
    /// Wasting Holt is a minor location
    WastingHolt,
    /// Weaver's Trail is a minor location
    WeaversTrail,
}

impl Location for UmbralWildwood {
    fn is_major(&self) -> bool {
        match self {
            UmbralWildwood::AdzeCrossroads => false,
            UmbralWildwood::Amethyst => true,
            UmbralWildwood::AtroposFate => true,
            UmbralWildwood::ClothosRefuge => true,
            UmbralWildwood::Dredgefield => false,
            UmbralWildwood::GoldenConcourse => false,
            UmbralWildwood::GoldenRootRanch => true,
            UmbralWildwood::HermitsRest => true,
            UmbralWildwood::LachesisTally => true,
            UmbralWildwood::LeatherbackPathway => false,
            UmbralWildwood::Sentry => true,
            UmbralWildwood::SteelyFields => false,
            UmbralWildwood::Stray => true,
            UmbralWildwood::TerrapinWoods => false,
            UmbralWildwood::TheDredgewood => false,
            UmbralWildwood::TheFoundry => true,
            UmbralWildwood::TheFrontier => false,
            UmbralWildwood::TheGap => false,
            UmbralWildwood::TheStrands => false,
            UmbralWildwood::ThunderRow => false,
            UmbralWildwood::Thunderfoot => true,
            UmbralWildwood::VagrantBastion => true,
            UmbralWildwood::WastingHolt => false,
            UmbralWildwood::WeaversTrail => false,
        }
    }

    fn coords(&self) -> (f64, f64) {
        match self {
            UmbralWildwood::AdzeCrossroads => (0.357383, 0.38652703),
            UmbralWildwood::Amethyst => (0.8178504, 0.38884524),
            UmbralWildwood::AtroposFate => (0.5730031, 0.7843336),
            UmbralWildwood::ClothosRefuge => (0.61337584, 0.32398352),
            UmbralWildwood::Dredgefield => (0.7840042, 0.85631174),
            UmbralWildwood::GoldenConcourse => (0.15095828, 0.71263796),
            UmbralWildwood::GoldenRootRanch => (0.15671077, 0.5505076),
            UmbralWildwood::HermitsRest => (0.67122585, 0.6176634),
            UmbralWildwood::LachesisTally => (0.840556, 0.54475015),
            UmbralWildwood::LeatherbackPathway => (0.7109991, 0.37403926),
            UmbralWildwood::Sentry => (0.395983, 0.8252861),
            UmbralWildwood::SteelyFields => (0.16732694, 0.32295418),
            UmbralWildwood::Stray => (0.6767252, 0.1845278),
            UmbralWildwood::TerrapinWoods => (0.73006326, 0.49094966),
            UmbralWildwood::TheDredgewood => (0.78483456, 0.66172403),
            UmbralWildwood::TheFoundry => (0.27597004, 0.2246597),
            UmbralWildwood::TheFrontier => (0.61156416, 0.721657),
            UmbralWildwood::TheGap => (0.42371795, 0.5318492),
            UmbralWildwood::TheStrands => (0.5069804, 0.23784174),
            UmbralWildwood::ThunderRow => (0.26085073, 0.66316617),
            UmbralWildwood::Thunderfoot => (0.34593302, 0.51468116),
            UmbralWildwood::VagrantBastion => (0.5076587, 0.1616039),
            UmbralWildwood::WastingHolt => (0.5504174, 0.9400169),
            UmbralWildwood::WeaversTrail => (0.55006844, 0.44620684),
        }
    }

    fn name_api(&self) -> (&str, &str) {
        (
            "UmbralWildwood",
            match self {
                UmbralWildwood::AdzeCrossroads => "AdzeCrossroads",
                UmbralWildwood::Amethyst => "Amethyst",
                UmbralWildwood::AtroposFate => "AtroposFate",
                UmbralWildwood::ClothosRefuge => "ClothosRefuge",
                UmbralWildwood::Dredgefield => "Dredgefield",
                UmbralWildwood::GoldenConcourse => "GoldenConcourse",
                UmbralWildwood::GoldenRootRanch => "GoldenRootRanch",
                UmbralWildwood::HermitsRest => "HermitsRest",
                UmbralWildwood::LachesisTally => "LachesisTally",
                UmbralWildwood::LeatherbackPathway => "LeatherbackPathway",
                UmbralWildwood::Sentry => "Sentry",
                UmbralWildwood::SteelyFields => "SteelyFields",
                UmbralWildwood::Stray => "Stray",
                UmbralWildwood::TerrapinWoods => "TerrapinWoods",
                UmbralWildwood::TheDredgewood => "TheDredgewood",
                UmbralWildwood::TheFoundry => "TheFoundry",
                UmbralWildwood::TheFrontier => "TheFrontier",
                UmbralWildwood::TheGap => "TheGap",
                UmbralWildwood::TheStrands => "TheStrands",
                UmbralWildwood::ThunderRow => "ThunderRow",
                UmbralWildwood::Thunderfoot => "Thunderfoot",
                UmbralWildwood::VagrantBastion => "VagrantBastion",
                UmbralWildwood::WastingHolt => "WastingHolt",
                UmbralWildwood::WeaversTrail => "WeaversTrail",
            },
        )
    }

    fn name_friendly(&self) -> (&str, &str) {
        (
            "UmbralWildwood",
            match self {
                UmbralWildwood::AdzeCrossroads => "Adze Crossroads",
                UmbralWildwood::Amethyst => "Amethyst",
                UmbralWildwood::AtroposFate => "Atropos' Fate",
                UmbralWildwood::ClothosRefuge => "Clotho's Refuge",
                UmbralWildwood::Dredgefield => "Dredgefield",
                UmbralWildwood::GoldenConcourse => "Golden Concourse",
                UmbralWildwood::GoldenRootRanch => "GoldenRoot Ranch",
                UmbralWildwood::HermitsRest => "Hermit's Rest",
                UmbralWildwood::LachesisTally => "Lachesis' Tally ",
                UmbralWildwood::LeatherbackPathway => "Leatherback Pathway",
                UmbralWildwood::Sentry => "Sentry",
                UmbralWildwood::SteelyFields => "Steely Fields",
                UmbralWildwood::Stray => "Stray",
                UmbralWildwood::TerrapinWoods => "Terrapin Woods",
                UmbralWildwood::TheDredgewood => "The Dredgewood",
                UmbralWildwood::TheFoundry => "The Foundry",
                UmbralWildwood::TheFrontier => "The Frontier",
                UmbralWildwood::TheGap => "The Gap",
                UmbralWildwood::TheStrands => "The Strands",
                UmbralWildwood::ThunderRow => "Thunder Row",
                UmbralWildwood::Thunderfoot => "Thunderfoot",
                UmbralWildwood::VagrantBastion => "Vagrant Bastion",
                UmbralWildwood::WastingHolt => "Wasting Holt",
                UmbralWildwood::WeaversTrail => "Weaver's Trail",
            },
        )
    }
}

/// Specific map details for the `OarbreakerHex` tile
pub enum Oarbreaker {
    /// Barrenson is a minor location
    Barrenson,
    /// Base Akri is a major location
    BaseAkri,
    /// Castor is a minor location
    Castor,
    /// Crach Woods is a minor location
    CrachWoods,
    /// Fogwood is a minor location
    Fogwood,
    /// Gold is a major location
    Gold,
    /// Grisly Refuge is a minor location
    GrislyRefuge,
    /// Integrum is a major location
    Integrum,
    /// Kofteri Channel is a minor location
    KofteriChannel,
    /// Lion's Head is a major location
    LionsHead,
    /// Martius is a minor location
    Martius,
    /// Mount Marce is a minor location
    MountMarce,
    /// Neptune's Throne is a minor location
    NeptunesThrone,
    /// Oasis is a minor location
    Oasis,
    /// Obitum is a minor location
    Obitum,
    /// Pollux is a minor location
    Pollux,
    /// Posterus is a major location
    Posterus,
    /// Reliqua is a major location
    Reliqua,
    /// Sandalwood Beach is a minor location
    SandalwoodBeach,
    /// Sheep's Head is a minor location
    SheepsHead,
    /// Silver is a minor location
    Silver,
    /// Skelter Course is a major location
    SkelterCourse,
    /// Skull Beach is a minor location
    SkullBeach,
    /// The Conclave is a major location
    TheConclave,
    /// The Dirk is a minor location
    TheDirk,
    /// The Emblem is a minor location
    TheEmblem,
    /// The Ides is a major location
    TheIdes,
}

impl Location for Oarbreaker {
    fn is_major(&self) -> bool {
        match self {
            Oarbreaker::Barrenson => false,
            Oarbreaker::BaseAkri => true,
            Oarbreaker::Castor => false,
            Oarbreaker::CrachWoods => false,
            Oarbreaker::Fogwood => false,
            Oarbreaker::Gold => true,
            Oarbreaker::GrislyRefuge => false,
            Oarbreaker::Integrum => true,
            Oarbreaker::KofteriChannel => false,
            Oarbreaker::LionsHead => true,
            Oarbreaker::Martius => false,
            Oarbreaker::MountMarce => false,
            Oarbreaker::NeptunesThrone => false,
            Oarbreaker::Oasis => false,
            Oarbreaker::Obitum => false,
            Oarbreaker::Pollux => false,
            Oarbreaker::Posterus => true,
            Oarbreaker::Reliqua => true,
            Oarbreaker::SandalwoodBeach => false,
            Oarbreaker::SheepsHead => false,
            Oarbreaker::Silver => false,
            Oarbreaker::SkelterCourse => true,
            Oarbreaker::SkullBeach => false,
            Oarbreaker::TheConclave => true,
            Oarbreaker::TheDirk => false,
            Oarbreaker::TheEmblem => false,
            Oarbreaker::TheIdes => true,
        }
    }

    fn coords(&self) -> (f64, f64) {
        match self {
            Oarbreaker::Barrenson => (0.20925796, 0.58191305),
            Oarbreaker::BaseAkri => (0.9117557, 0.51201063),
            Oarbreaker::Castor => (0.25655335, 0.503573),
            Oarbreaker::CrachWoods => (0.44473228, 0.35592464),
            Oarbreaker::Fogwood => (0.72000915, 0.13007016),
            Oarbreaker::Gold => (0.49749142, 0.16744888),
            Oarbreaker::GrislyRefuge => (0.306169, 0.6532009),
            Oarbreaker::Integrum => (0.6686882, 0.773776),
            Oarbreaker::KofteriChannel => (0.4581904, 0.83506876),
            Oarbreaker::LionsHead => (0.3283593, 0.89686215),
            Oarbreaker::Martius => (0.84350735, 0.2846215),
            Oarbreaker::MountMarce => (0.5775114, 0.31234464),
            Oarbreaker::NeptunesThrone => (0.60160726, 0.17799203),
            Oarbreaker::Oasis => (0.5653535, 0.916719),
            Oarbreaker::Obitum => (0.5717222, 0.7513693),
            Oarbreaker::Pollux => (0.2113825, 0.5136759),
            Oarbreaker::Posterus => (0.79465634, 0.6755905),
            Oarbreaker::Reliqua => (0.6536689, 0.21361786),
            Oarbreaker::SandalwoodBeach => (0.5139656, 0.2979552),
            Oarbreaker::SheepsHead => (0.4270079, 0.9615619),
            Oarbreaker::Silver => (0.27285343, 0.2775804),
            Oarbreaker::SkelterCourse => (0.60323334, 0.42578885),
            Oarbreaker::SkullBeach => (0.48592106, 0.45108822),
            Oarbreaker::TheConclave => (0.51089185, 0.624818),
            Oarbreaker::TheDirk => (0.43934482, 0.7884207),
            Oarbreaker::TheEmblem => (0.72488797, 0.25433928),
            Oarbreaker::TheIdes => (0.85254693, 0.3931426),
        }
    }

    fn name_api(&self) -> (&str, &str) {
        (
            "Oarbreaker",
            match self {
                Oarbreaker::Barrenson => "Barrenson",
                Oarbreaker::BaseAkri => "BaseAkri",
                Oarbreaker::Castor => "Castor",
                Oarbreaker::CrachWoods => "CrachWoods",
                Oarbreaker::Fogwood => "Fogwood",
                Oarbreaker::Gold => "Gold",
                Oarbreaker::GrislyRefuge => "GrislyRefuge",
                Oarbreaker::Integrum => "Integrum",
                Oarbreaker::KofteriChannel => "KofteriChannel",
                Oarbreaker::LionsHead => "LionsHead",
                Oarbreaker::Martius => "Martius",
                Oarbreaker::MountMarce => "MountMarce",
                Oarbreaker::NeptunesThrone => "NeptunesThrone",
                Oarbreaker::Oasis => "Oasis",
                Oarbreaker::Obitum => "Obitum",
                Oarbreaker::Pollux => "Pollux",
                Oarbreaker::Posterus => "Posterus",
                Oarbreaker::Reliqua => "Reliqua",
                Oarbreaker::SandalwoodBeach => "SandalwoodBeach",
                Oarbreaker::SheepsHead => "SheepsHead",
                Oarbreaker::Silver => "Silver",
                Oarbreaker::SkelterCourse => "SkelterCourse",
                Oarbreaker::SkullBeach => "SkullBeach",
                Oarbreaker::TheConclave => "TheConclave",
                Oarbreaker::TheDirk => "TheDirk",
                Oarbreaker::TheEmblem => "TheEmblem",
                Oarbreaker::TheIdes => "TheIdes",
            },
        )
    }

    fn name_friendly(&self) -> (&str, &str) {
        (
            "Oarbreaker",
            match self {
                Oarbreaker::Barrenson => "Barrenson",
                Oarbreaker::BaseAkri => "Base Akri",
                Oarbreaker::Castor => "Castor",
                Oarbreaker::CrachWoods => "Crach Woods",
                Oarbreaker::Fogwood => "Fogwood",
                Oarbreaker::Gold => "Gold",
                Oarbreaker::GrislyRefuge => "Grisly Refuge",
                Oarbreaker::Integrum => "Integrum",
                Oarbreaker::KofteriChannel => "Kofteri Channel",
                Oarbreaker::LionsHead => "Lion's Head",
                Oarbreaker::Martius => "Martius",
                Oarbreaker::MountMarce => "Mount Marce",
                Oarbreaker::NeptunesThrone => "Neptune's Throne",
                Oarbreaker::Oasis => "Oasis",
                Oarbreaker::Obitum => "Obitum",
                Oarbreaker::Pollux => "Pollux",
                Oarbreaker::Posterus => "Posterus",
                Oarbreaker::Reliqua => "Reliqua",
                Oarbreaker::SandalwoodBeach => "Sandalwood Beach",
                Oarbreaker::SheepsHead => "Sheep's Head",
                Oarbreaker::Silver => "Silver",
                Oarbreaker::SkelterCourse => "Skelter Course",
                Oarbreaker::SkullBeach => "Skull Beach",
                Oarbreaker::TheConclave => "The Conclave",
                Oarbreaker::TheDirk => "The Dirk",
                Oarbreaker::TheEmblem => "The Emblem",
                Oarbreaker::TheIdes => "The Ides",
            },
        )
    }
}

/// Specific map details for the `CallahansPassageHex` tile
pub enum CallahansPassage {
    /// Callahan's Eye is a minor location
    CallahansEye,
    /// Chapel Access is a minor location
    ChapelAccess,
    /// Cragsfield is a minor location
    Cragsfield,
    /// Cragsroad is a minor location
    Cragsroad,
    /// Cragstown is a major location
    Cragstown,
    /// Crumbling Post is a major location
    CrumblingPost,
    /// Lingering Lashes is a minor location
    LingeringLashes,
    /// Lochan is a minor location
    Lochan,
    /// Lochan Berth is a major location
    LochanBerth,
    /// Lost Tops is a minor location
    LostTops,
    /// Overlook Hill is a major location
    OverlookHill,
    /// Sc??th Passing is a major location
    Sc??thPassing,
    /// Sioc Approach is a minor location
    SiocApproach,
    /// Solas Gateway is a minor location
    SolasGateway,
    /// Solas Gorge is a major location
    SolasGorge,
    /// Soured Fields is a minor location
    SouredFields,
    /// The Crumbling Passage is a major location
    TheCrumblingPassage,
    /// The Key is a minor location
    TheKey,
    /// The Lance is a minor location
    TheLance,
    /// The Latch is a major location
    TheLatch,
    /// The Procession is a major location
    TheProcession,
    /// The Rust Road is a minor location
    TheRustRoad,
    /// The Stern is a minor location
    TheStern,
    /// Twisted Mumble is a minor location
    TwistedMumble,
    /// Whispering Gulch is a minor location
    WhisperingGulch,
    /// White Chapel is a major location
    WhiteChapel,
    /// Winding Crag is a minor location
    WindingCrag,
}

impl Location for CallahansPassage {
    fn is_major(&self) -> bool {
        match self {
            CallahansPassage::CallahansEye => false,
            CallahansPassage::ChapelAccess => false,
            CallahansPassage::Cragsfield => false,
            CallahansPassage::Cragsroad => false,
            CallahansPassage::Cragstown => true,
            CallahansPassage::CrumblingPost => true,
            CallahansPassage::LingeringLashes => false,
            CallahansPassage::Lochan => false,
            CallahansPassage::LochanBerth => true,
            CallahansPassage::LostTops => false,
            CallahansPassage::OverlookHill => true,
            CallahansPassage::Sc??thPassing => true,
            CallahansPassage::SiocApproach => false,
            CallahansPassage::SolasGateway => false,
            CallahansPassage::SolasGorge => true,
            CallahansPassage::SouredFields => false,
            CallahansPassage::TheCrumblingPassage => true,
            CallahansPassage::TheKey => false,
            CallahansPassage::TheLance => false,
            CallahansPassage::TheLatch => true,
            CallahansPassage::TheProcession => true,
            CallahansPassage::TheRustRoad => false,
            CallahansPassage::TheStern => false,
            CallahansPassage::TwistedMumble => false,
            CallahansPassage::WhisperingGulch => false,
            CallahansPassage::WhiteChapel => true,
            CallahansPassage::WindingCrag => false,
        }
    }

    fn coords(&self) -> (f64, f64) {
        match self {
            CallahansPassage::CallahansEye => (0.30864334, 0.26224336),
            CallahansPassage::ChapelAccess => (0.8670075, 0.52318686),
            CallahansPassage::Cragsfield => (0.5690361, 0.19898543),
            CallahansPassage::Cragsroad => (0.44606456, 0.35310647),
            CallahansPassage::Cragstown => (0.51669085, 0.13976866),
            CallahansPassage::CrumblingPost => (0.35385895, 0.7248818),
            CallahansPassage::LingeringLashes => (0.39600572, 0.16319244),
            CallahansPassage::Lochan => (0.54940885, 0.5079033),
            CallahansPassage::LochanBerth => (0.56270015, 0.42374668),
            CallahansPassage::LostTops => (0.2450918, 0.87107843),
            CallahansPassage::OverlookHill => (0.5252302, 0.6481513),
            CallahansPassage::Sc??thPassing => (0.34909955, 0.53184307),
            CallahansPassage::SiocApproach => (0.6218272, 0.67211956),
            CallahansPassage::SolasGateway => (0.23241858, 0.42319626),
            CallahansPassage::SolasGorge => (0.19406947, 0.33073047),
            CallahansPassage::SouredFields => (0.09100611, 0.532157),
            CallahansPassage::TheCrumblingPassage => (0.45544657, 0.7847246),
            CallahansPassage::TheKey => (0.7205697, 0.9363465),
            CallahansPassage::TheLance => (0.38485914, 0.07471772),
            CallahansPassage::TheLatch => (0.72713715, 0.68313295),
            CallahansPassage::TheProcession => (0.7513888, 0.43929663),
            CallahansPassage::TheRustRoad => (0.20239393, 0.69895375),
            CallahansPassage::TheStern => (0.7807102, 0.7850171),
            CallahansPassage::TwistedMumble => (0.6754054, 0.17212075),
            CallahansPassage::WhisperingGulch => (0.49615276, 1.0449303),
            CallahansPassage::WhiteChapel => (0.7634679, 0.2669874),
            CallahansPassage::WindingCrag => (0.52042425, 0.24549948),
        }
    }

    fn name_api(&self) -> (&str, &str) {
        (
            "CallahansPassage",
            match self {
                CallahansPassage::CallahansEye => "CallahansEye",
                CallahansPassage::ChapelAccess => "ChapelAccess",
                CallahansPassage::Cragsfield => "Cragsfield",
                CallahansPassage::Cragsroad => "Cragsroad",
                CallahansPassage::Cragstown => "Cragstown",
                CallahansPassage::CrumblingPost => "CrumblingPost",
                CallahansPassage::LingeringLashes => "LingeringLashes",
                CallahansPassage::Lochan => "Lochan",
                CallahansPassage::LochanBerth => "LochanBerth",
                CallahansPassage::LostTops => "LostTops",
                CallahansPassage::OverlookHill => "OverlookHill",
                CallahansPassage::Sc??thPassing => "Sc??thPassing",
                CallahansPassage::SiocApproach => "SiocApproach",
                CallahansPassage::SolasGateway => "SolasGateway",
                CallahansPassage::SolasGorge => "SolasGorge",
                CallahansPassage::SouredFields => "SouredFields",
                CallahansPassage::TheCrumblingPassage => "TheCrumblingPassage",
                CallahansPassage::TheKey => "TheKey",
                CallahansPassage::TheLance => "TheLance",
                CallahansPassage::TheLatch => "TheLatch",
                CallahansPassage::TheProcession => "TheProcession",
                CallahansPassage::TheRustRoad => "TheRustRoad",
                CallahansPassage::TheStern => "TheStern",
                CallahansPassage::TwistedMumble => "TwistedMumble",
                CallahansPassage::WhisperingGulch => "WhisperingGulch",
                CallahansPassage::WhiteChapel => "WhiteChapel",
                CallahansPassage::WindingCrag => "WindingCrag",
            },
        )
    }

    fn name_friendly(&self) -> (&str, &str) {
        (
            "CallahansPassage",
            match self {
                CallahansPassage::CallahansEye => "Callahan's Eye",
                CallahansPassage::ChapelAccess => "Chapel Access",
                CallahansPassage::Cragsfield => "Cragsfield",
                CallahansPassage::Cragsroad => "Cragsroad",
                CallahansPassage::Cragstown => "Cragstown",
                CallahansPassage::CrumblingPost => "Crumbling Post",
                CallahansPassage::LingeringLashes => "Lingering Lashes",
                CallahansPassage::Lochan => "Lochan",
                CallahansPassage::LochanBerth => "Lochan Berth",
                CallahansPassage::LostTops => "Lost Tops",
                CallahansPassage::OverlookHill => "Overlook Hill",
                CallahansPassage::Sc??thPassing => "Sc??th Passing",
                CallahansPassage::SiocApproach => "Sioc Approach",
                CallahansPassage::SolasGateway => "Solas Gateway",
                CallahansPassage::SolasGorge => "Solas Gorge",
                CallahansPassage::SouredFields => "Soured Fields",
                CallahansPassage::TheCrumblingPassage => "The Crumbling Passage",
                CallahansPassage::TheKey => "The Key",
                CallahansPassage::TheLance => "The Lance",
                CallahansPassage::TheLatch => "The Latch",
                CallahansPassage::TheProcession => "The Procession",
                CallahansPassage::TheRustRoad => "The Rust Road",
                CallahansPassage::TheStern => "The Stern",
                CallahansPassage::TwistedMumble => "Twisted Mumble",
                CallahansPassage::WhisperingGulch => "Whispering Gulch",
                CallahansPassage::WhiteChapel => "White Chapel",
                CallahansPassage::WindingCrag => "Winding Crag",
            },
        )
    }
}

/// Specific map details for the `DrownedValeHex` tile
pub enum DrownedVale {
    /// Bootnap is a major location
    Bootnap,
    /// Coaldrifter Stead is a major location
    CoaldrifterStead,
    /// Eastmarch is a major location
    Eastmarch,
    /// Esterfal is a minor location
    Esterfal,
    /// Fleetsfall River is a minor location
    FleetsfallRiver,
    /// Linger is a minor location
    Linger,
    /// Loggerhead is a major location
    Loggerhead,
    /// Singing Serpents is a major location
    SingingSerpents,
    /// Sop Fields is a minor location
    SopFields,
    /// Splinter Pens is a major location
    SplinterPens,
    /// Sprite's Game is a minor location
    SpritesGame,
    /// The Baths is a major location
    TheBaths,
    /// The Other Vein is a minor location
    TheOtherVein,
    /// The Saltcaps is a major location
    TheSaltcaps,
    /// The Turtlerocks is a minor location
    TheTurtlerocks,
    /// The Wash is a major location
    TheWash,
    /// The Willow Wood is a minor location
    TheWillowWood,
    /// Vessel is a major location
    Vessel,
    /// Wisp's Warning is a major location
    WispsWarning,
}

impl Location for DrownedVale {
    fn is_major(&self) -> bool {
        match self {
            DrownedVale::Bootnap => true,
            DrownedVale::CoaldrifterStead => true,
            DrownedVale::Eastmarch => true,
            DrownedVale::Esterfal => false,
            DrownedVale::FleetsfallRiver => false,
            DrownedVale::Linger => false,
            DrownedVale::Loggerhead => true,
            DrownedVale::SingingSerpents => true,
            DrownedVale::SopFields => false,
            DrownedVale::SplinterPens => true,
            DrownedVale::SpritesGame => false,
            DrownedVale::TheBaths => true,
            DrownedVale::TheOtherVein => false,
            DrownedVale::TheSaltcaps => true,
            DrownedVale::TheTurtlerocks => false,
            DrownedVale::TheWash => true,
            DrownedVale::TheWillowWood => false,
            DrownedVale::Vessel => true,
            DrownedVale::WispsWarning => true,
        }
    }

    fn coords(&self) -> (f64, f64) {
        match self {
            DrownedVale::Bootnap => (0.80842125, 0.5582795),
            DrownedVale::CoaldrifterStead => (0.66231805, 0.82284665),
            DrownedVale::Eastmarch => (0.2335479, 0.5278097),
            DrownedVale::Esterfal => (0.7868035, 0.6955586),
            DrownedVale::FleetsfallRiver => (0.48596796, 0.08352051),
            DrownedVale::Linger => (0.4844155, 0.3447626),
            DrownedVale::Loggerhead => (0.4150385, 0.20794752),
            DrownedVale::SingingSerpents => (0.6632067, 0.31274968),
            DrownedVale::SopFields => (0.51988655, 0.6732017),
            DrownedVale::SplinterPens => (0.47195154, 0.8008652),
            DrownedVale::SpritesGame => (0.32643563, 0.4329372),
            DrownedVale::TheBaths => (0.5683618, 0.52607054),
            DrownedVale::TheOtherVein => (0.120904915, 0.6001116),
            DrownedVale::TheSaltcaps => (0.63300836, 0.16928926),
            DrownedVale::TheTurtlerocks => (0.5794284, 0.35845056),
            DrownedVale::TheWash => (0.38150814, 0.5121856),
            DrownedVale::TheWillowWood => (0.30738774, 0.8374743),
            DrownedVale::Vessel => (0.36351255, 0.715022),
            DrownedVale::WispsWarning => (0.33081633, 0.29379123),
        }
    }

    fn name_api(&self) -> (&str, &str) {
        (
            "DrownedVale",
            match self {
                DrownedVale::Bootnap => "Bootnap",
                DrownedVale::CoaldrifterStead => "CoaldrifterStead",
                DrownedVale::Eastmarch => "Eastmarch",
                DrownedVale::Esterfal => "Esterfal",
                DrownedVale::FleetsfallRiver => "FleetsfallRiver",
                DrownedVale::Linger => "Linger",
                DrownedVale::Loggerhead => "Loggerhead",
                DrownedVale::SingingSerpents => "SingingSerpents",
                DrownedVale::SopFields => "SopFields",
                DrownedVale::SplinterPens => "SplinterPens",
                DrownedVale::SpritesGame => "SpritesGame",
                DrownedVale::TheBaths => "TheBaths",
                DrownedVale::TheOtherVein => "TheOtherVein",
                DrownedVale::TheSaltcaps => "TheSaltcaps",
                DrownedVale::TheTurtlerocks => "TheTurtlerocks",
                DrownedVale::TheWash => "TheWash",
                DrownedVale::TheWillowWood => "TheWillowWood",
                DrownedVale::Vessel => "Vessel",
                DrownedVale::WispsWarning => "WispsWarning",
            },
        )
    }

    fn name_friendly(&self) -> (&str, &str) {
        (
            "DrownedVale",
            match self {
                DrownedVale::Bootnap => "Bootnap",
                DrownedVale::CoaldrifterStead => "Coaldrifter Stead",
                DrownedVale::Eastmarch => "Eastmarch",
                DrownedVale::Esterfal => "Esterfal",
                DrownedVale::FleetsfallRiver => "Fleetsfall River",
                DrownedVale::Linger => "Linger",
                DrownedVale::Loggerhead => "Loggerhead",
                DrownedVale::SingingSerpents => "Singing Serpents",
                DrownedVale::SopFields => "Sop Fields",
                DrownedVale::SplinterPens => "Splinter Pens",
                DrownedVale::SpritesGame => "Sprite's Game",
                DrownedVale::TheBaths => "The Baths",
                DrownedVale::TheOtherVein => "The Other Vein",
                DrownedVale::TheSaltcaps => "The Saltcaps",
                DrownedVale::TheTurtlerocks => "The Turtlerocks",
                DrownedVale::TheWash => "The Wash",
                DrownedVale::TheWillowWood => "The Willow Wood",
                DrownedVale::Vessel => "Vessel",
                DrownedVale::WispsWarning => "Wisp's Warning",
            },
        )
    }
}

/// Specific map details for the `FarranacCoastHex` tile
pub enum FarranacCoast {
    /// Apollo's Landing is a minor location
    ApollosLanding,
    /// Carrion Fields is a minor location
    CarrionFields,
    /// Cora Lushlands is a minor location
    CoraLushlands,
    /// Cormac Beach is a minor location
    CormacBeach,
    /// Gulf of the Daughters is a minor location
    GulfoftheDaughters,
    /// Hermes Inlet is a minor location
    HermesInlet,
    /// Huskhollow is a major location
    Huskhollow,
    /// Iuxta Homestead is a major location
    IuxtaHomestead,
    /// Kardia is a minor location
    Kardia,
    /// Liberation Street is a minor location
    LiberationStreet,
    /// Macha's Keening is a major location
    MachasKeening,
    /// Mara is a major location
    Mara,
    /// McCarthy Fields is a minor location
    McCarthyFields,
    /// Mooring Dens is a minor location
    MooringDens,
    /// Pleading Wharf is a major location
    PleadingWharf,
    /// Scarp of Ambrose is a major location
    ScarpofAmbrose,
    /// Scythe is a major location
    Scythe,
    /// Sickle Hill is a minor location
    SickleHill,
    /// Skeleton Road is a minor location
    SkeletonRoad,
    /// Sunder Beach is a minor location
    SunderBeach,
    /// Terra is a major location
    Terra,
    /// The Bay of Artemis is a minor location
    TheBayofArtemis,
    /// The Bone Haft is a major location
    TheBoneHaft,
    /// The Heart Road is a minor location
    TheHeartRoad,
    /// The Iron Beach is a minor location
    TheIronBeach,
    /// The Jade Cove is a major location
    TheJadeCove,
    /// The Mirror is a minor location
    TheMirror,
    /// The Reaping Fields is a minor location
    TheReapingFields,
    /// The River Mercy is a minor location
    TheRiverMercy,
    /// The Snag is a minor location
    TheSnag,
    /// The Spearhead is a major location
    TheSpearhead,
    /// The Winged Walk is a minor location
    TheWingedWalk,
    /// Transient Valley is a minor location
    TransientValley,
    /// Victa is a major location
    Victa,
}

impl Location for FarranacCoast {
    fn is_major(&self) -> bool {
        match self {
            FarranacCoast::ApollosLanding => false,
            FarranacCoast::CarrionFields => false,
            FarranacCoast::CoraLushlands => false,
            FarranacCoast::CormacBeach => false,
            FarranacCoast::GulfoftheDaughters => false,
            FarranacCoast::HermesInlet => false,
            FarranacCoast::Huskhollow => true,
            FarranacCoast::IuxtaHomestead => true,
            FarranacCoast::Kardia => false,
            FarranacCoast::LiberationStreet => false,
            FarranacCoast::MachasKeening => true,
            FarranacCoast::Mara => true,
            FarranacCoast::McCarthyFields => false,
            FarranacCoast::MooringDens => false,
            FarranacCoast::PleadingWharf => true,
            FarranacCoast::ScarpofAmbrose => true,
            FarranacCoast::Scythe => true,
            FarranacCoast::SickleHill => false,
            FarranacCoast::SkeletonRoad => false,
            FarranacCoast::SunderBeach => false,
            FarranacCoast::Terra => true,
            FarranacCoast::TheBayofArtemis => false,
            FarranacCoast::TheBoneHaft => true,
            FarranacCoast::TheHeartRoad => false,
            FarranacCoast::TheIronBeach => false,
            FarranacCoast::TheJadeCove => true,
            FarranacCoast::TheMirror => false,
            FarranacCoast::TheReapingFields => false,
            FarranacCoast::TheRiverMercy => false,
            FarranacCoast::TheSnag => false,
            FarranacCoast::TheSpearhead => true,
            FarranacCoast::TheWingedWalk => false,
            FarranacCoast::TransientValley => false,
            FarranacCoast::Victa => true,
        }
    }

    fn coords(&self) -> (f64, f64) {
        match self {
            FarranacCoast::ApollosLanding => (0.4314927, 0.57976526),
            FarranacCoast::CarrionFields => (0.61662257, 0.45785448),
            FarranacCoast::CoraLushlands => (0.54585594, 0.8671113),
            FarranacCoast::CormacBeach => (0.4727384, 0.78347987),
            FarranacCoast::GulfoftheDaughters => (0.016857829, 0.58329874),
            FarranacCoast::HermesInlet => (0.2707489, 0.432295),
            FarranacCoast::Huskhollow => (0.5860919, 0.56278074),
            FarranacCoast::IuxtaHomestead => (0.7127172, 0.19546622),
            FarranacCoast::Kardia => (0.6261059, 0.86518013),
            FarranacCoast::LiberationStreet => (0.8314815, 0.8458243),
            FarranacCoast::MachasKeening => (0.70205057, 0.7824685),
            FarranacCoast::Mara => (0.8549706, 0.48427492),
            FarranacCoast::McCarthyFields => (0.6015004, 0.7810918),
            FarranacCoast::MooringDens => (0.5923805, 0.28515506),
            FarranacCoast::PleadingWharf => (0.6262762, 0.71746904),
            FarranacCoast::ScarpofAmbrose => (0.4770394, 0.37934086),
            FarranacCoast::Scythe => (0.78855526, 0.34850213),
            FarranacCoast::SickleHill => (0.4659355, 0.44424376),
            FarranacCoast::SkeletonRoad => (0.59403133, 0.35402375),
            FarranacCoast::SunderBeach => (0.35613468, 0.4198639),
            FarranacCoast::Terra => (0.6119086, 0.40515536),
            FarranacCoast::TheBayofArtemis => (0.35195225, 0.6874335),
            FarranacCoast::TheBoneHaft => (0.49903932, 0.20359577),
            FarranacCoast::TheHeartRoad => (0.66623056, 0.9169244),
            FarranacCoast::TheIronBeach => (0.3352979, 0.26261732),
            FarranacCoast::TheJadeCove => (0.4322609, 0.6490256),
            FarranacCoast::TheMirror => (0.8247223, 0.41111124),
            FarranacCoast::TheReapingFields => (0.6509468, 0.51992047),
            FarranacCoast::TheRiverMercy => (0.57574064, 0.6521498),
            FarranacCoast::TheSnag => (0.89351374, 0.7870541),
            FarranacCoast::TheSpearhead => (0.25413433, 0.2014643),
            FarranacCoast::TheWingedWalk => (0.7980949, 0.6511161),
            FarranacCoast::TransientValley => (0.62650084, 0.17376602),
            FarranacCoast::Victa => (0.4417568, 0.8744741),
        }
    }

    fn name_api(&self) -> (&str, &str) {
        (
            "FarranacCoast",
            match self {
                FarranacCoast::ApollosLanding => "ApollosLanding",
                FarranacCoast::CarrionFields => "CarrionFields",
                FarranacCoast::CoraLushlands => "CoraLushlands",
                FarranacCoast::CormacBeach => "CormacBeach",
                FarranacCoast::GulfoftheDaughters => "GulfoftheDaughters",
                FarranacCoast::HermesInlet => "HermesInlet",
                FarranacCoast::Huskhollow => "Huskhollow",
                FarranacCoast::IuxtaHomestead => "IuxtaHomestead",
                FarranacCoast::Kardia => "Kardia",
                FarranacCoast::LiberationStreet => "LiberationStreet",
                FarranacCoast::MachasKeening => "MachasKeening",
                FarranacCoast::Mara => "Mara",
                FarranacCoast::McCarthyFields => "McCarthyFields",
                FarranacCoast::MooringDens => "MooringDens",
                FarranacCoast::PleadingWharf => "PleadingWharf",
                FarranacCoast::ScarpofAmbrose => "ScarpofAmbrose",
                FarranacCoast::Scythe => "Scythe",
                FarranacCoast::SickleHill => "SickleHill",
                FarranacCoast::SkeletonRoad => "SkeletonRoad",
                FarranacCoast::SunderBeach => "SunderBeach",
                FarranacCoast::Terra => "Terra",
                FarranacCoast::TheBayofArtemis => "TheBayofArtemis",
                FarranacCoast::TheBoneHaft => "TheBoneHaft",
                FarranacCoast::TheHeartRoad => "TheHeartRoad",
                FarranacCoast::TheIronBeach => "TheIronBeach",
                FarranacCoast::TheJadeCove => "TheJadeCove",
                FarranacCoast::TheMirror => "TheMirror",
                FarranacCoast::TheReapingFields => "TheReapingFields",
                FarranacCoast::TheRiverMercy => "TheRiverMercy",
                FarranacCoast::TheSnag => "TheSnag",
                FarranacCoast::TheSpearhead => "TheSpearhead",
                FarranacCoast::TheWingedWalk => "TheWingedWalk",
                FarranacCoast::TransientValley => "TransientValley",
                FarranacCoast::Victa => "Victa",
            },
        )
    }

    fn name_friendly(&self) -> (&str, &str) {
        (
            "FarranacCoast",
            match self {
                FarranacCoast::ApollosLanding => "Apollo's Landing",
                FarranacCoast::CarrionFields => "Carrion Fields",
                FarranacCoast::CoraLushlands => "Cora Lushlands",
                FarranacCoast::CormacBeach => "Cormac Beach",
                FarranacCoast::GulfoftheDaughters => "Gulf of the Daughters",
                FarranacCoast::HermesInlet => "Hermes Inlet",
                FarranacCoast::Huskhollow => "Huskhollow",
                FarranacCoast::IuxtaHomestead => "Iuxta Homestead",
                FarranacCoast::Kardia => "Kardia",
                FarranacCoast::LiberationStreet => "Liberation Street",
                FarranacCoast::MachasKeening => "Macha's Keening",
                FarranacCoast::Mara => "Mara",
                FarranacCoast::McCarthyFields => "McCarthy Fields",
                FarranacCoast::MooringDens => "Mooring Dens",
                FarranacCoast::PleadingWharf => "Pleading Wharf",
                FarranacCoast::ScarpofAmbrose => "Scarp of Ambrose",
                FarranacCoast::Scythe => "Scythe",
                FarranacCoast::SickleHill => "Sickle Hill",
                FarranacCoast::SkeletonRoad => "Skeleton Road",
                FarranacCoast::SunderBeach => "Sunder Beach",
                FarranacCoast::Terra => "Terra",
                FarranacCoast::TheBayofArtemis => "The Bay of Artemis",
                FarranacCoast::TheBoneHaft => "The Bone Haft",
                FarranacCoast::TheHeartRoad => "The Heart Road",
                FarranacCoast::TheIronBeach => "The Iron Beach",
                FarranacCoast::TheJadeCove => "The Jade Cove",
                FarranacCoast::TheMirror => "The Mirror",
                FarranacCoast::TheReapingFields => "The Reaping Fields",
                FarranacCoast::TheRiverMercy => "The River Mercy",
                FarranacCoast::TheSnag => "The Snag",
                FarranacCoast::TheSpearhead => "The Spearhead",
                FarranacCoast::TheWingedWalk => "The Winged Walk",
                FarranacCoast::TransientValley => "Transient Valley",
                FarranacCoast::Victa => "Victa",
            },
        )
    }
}

/// Specific map details for the `MooringCountyHex` tile
pub enum MooringCounty {
    /// Borderlane is a minor location
    Borderlane,
    /// Gravekeeper's Holdfast is a major location
    GravekeepersHoldfast,
    /// Headstone is a major location
    Headstone,
    /// Luch's Workshop is a major location
    LuchsWorkshop,
    /// Lyon's Wood is a minor location
    LyonsWood,
    /// MacConmara Barrows is a major location
    MacConmaraBarrows,
    /// Moon's Walk is a minor location
    MoonsWalk,
    /// Morrighan's Grave is a major location
    MorrighansGrave,
    /// Ogmaran is a major location
    Ogmaran,
    /// Reaching River is a minor location
    ReachingRiver,
    /// Riverhill is a minor location
    Riverhill,
    /// Sc??th Copse is a minor location
    Sc??thCopse,
    /// The Cut is a major location
    TheCut,
    /// The Graveyard is a minor location
    TheGraveyard,
    /// The Mound is a minor location
    TheMound,
    /// The Spade is a major location
    TheSpade,
    /// The Wind Hills is a major location
    TheWindHills,
    /// Wiccwalk is a major location
    Wiccwalk,
    /// Wiccwood is a minor location
    Wiccwood,
}

impl Location for MooringCounty {
    fn is_major(&self) -> bool {
        match self {
            MooringCounty::Borderlane => false,
            MooringCounty::GravekeepersHoldfast => true,
            MooringCounty::Headstone => true,
            MooringCounty::LuchsWorkshop => true,
            MooringCounty::LyonsWood => false,
            MooringCounty::MacConmaraBarrows => true,
            MooringCounty::MoonsWalk => false,
            MooringCounty::MorrighansGrave => true,
            MooringCounty::Ogmaran => true,
            MooringCounty::ReachingRiver => false,
            MooringCounty::Riverhill => false,
            MooringCounty::Sc??thCopse => false,
            MooringCounty::TheCut => true,
            MooringCounty::TheGraveyard => false,
            MooringCounty::TheMound => false,
            MooringCounty::TheSpade => true,
            MooringCounty::TheWindHills => true,
            MooringCounty::Wiccwalk => true,
            MooringCounty::Wiccwood => false,
        }
    }

    fn coords(&self) -> (f64, f64) {
        match self {
            MooringCounty::Borderlane => (0.68016315, 0.05430618),
            MooringCounty::GravekeepersHoldfast => (0.4243501, 0.68474805),
            MooringCounty::Headstone => (0.47541752, 0.49520585),
            MooringCounty::LuchsWorkshop => (0.7218184, 0.20239134),
            MooringCounty::LyonsWood => (0.88474876, 0.3929681),
            MooringCounty::MacConmaraBarrows => (0.5829396, 0.8054959),
            MooringCounty::MoonsWalk => (0.42077473, 0.3419856),
            MooringCounty::MorrighansGrave => (0.7286388, 0.6069982),
            MooringCounty::Ogmaran => (0.4312115, 0.17043234),
            MooringCounty::ReachingRiver => (0.63385016, 0.15762547),
            MooringCounty::Riverhill => (0.6587685, 0.23843731),
            MooringCounty::Sc??thCopse => (0.8642956, 0.55962217),
            MooringCounty::TheCut => (0.2518527, 0.74158436),
            MooringCounty::TheGraveyard => (0.3741754, 0.5396971),
            MooringCounty::TheMound => (0.5293958, 0.6347808),
            MooringCounty::TheSpade => (0.24432048, 0.43665987),
            MooringCounty::TheWindHills => (0.37567636, 0.89409083),
            MooringCounty::Wiccwalk => (0.77282596, 0.7926232),
            MooringCounty::Wiccwood => (0.64314055, 0.4514375),
        }
    }

    fn name_api(&self) -> (&str, &str) {
        (
            "MooringCounty",
            match self {
                MooringCounty::Borderlane => "Borderlane",
                MooringCounty::GravekeepersHoldfast => "GravekeepersHoldfast",
                MooringCounty::Headstone => "Headstone",
                MooringCounty::LuchsWorkshop => "LuchsWorkshop",
                MooringCounty::LyonsWood => "LyonsWood",
                MooringCounty::MacConmaraBarrows => "MacConmaraBarrows",
                MooringCounty::MoonsWalk => "MoonsWalk",
                MooringCounty::MorrighansGrave => "MorrighansGrave",
                MooringCounty::Ogmaran => "Ogmaran",
                MooringCounty::ReachingRiver => "ReachingRiver",
                MooringCounty::Riverhill => "Riverhill",
                MooringCounty::Sc??thCopse => "Sc??thCopse",
                MooringCounty::TheCut => "TheCut",
                MooringCounty::TheGraveyard => "TheGraveyard",
                MooringCounty::TheMound => "TheMound",
                MooringCounty::TheSpade => "TheSpade",
                MooringCounty::TheWindHills => "TheWindHills",
                MooringCounty::Wiccwalk => "Wiccwalk",
                MooringCounty::Wiccwood => "Wiccwood",
            },
        )
    }

    fn name_friendly(&self) -> (&str, &str) {
        (
            "MooringCounty",
            match self {
                MooringCounty::Borderlane => "Borderlane",
                MooringCounty::GravekeepersHoldfast => "Gravekeeper's Holdfast",
                MooringCounty::Headstone => "Headstone",
                MooringCounty::LuchsWorkshop => "Luch's Workshop",
                MooringCounty::LyonsWood => "Lyon's Wood",
                MooringCounty::MacConmaraBarrows => "MacConmara Barrows",
                MooringCounty::MoonsWalk => "Moon's Walk",
                MooringCounty::MorrighansGrave => "Morrighan's Grave",
                MooringCounty::Ogmaran => "Ogmaran",
                MooringCounty::ReachingRiver => "Reaching River",
                MooringCounty::Riverhill => "Riverhill",
                MooringCounty::Sc??thCopse => "Sc??th Copse",
                MooringCounty::TheCut => "The Cut",
                MooringCounty::TheGraveyard => "The Graveyard",
                MooringCounty::TheMound => "The Mound",
                MooringCounty::TheSpade => "The Spade",
                MooringCounty::TheWindHills => "The Wind Hills",
                MooringCounty::Wiccwalk => "Wiccwalk",
                MooringCounty::Wiccwood => "Wiccwood",
            },
        )
    }
}

/// Specific map details for the `WeatheredExpanseHex` tile
pub enum WeatheredExpanse {
    /// Bannerwatch is a minor location
    Bannerwatch,
    /// Barrowsfield is a minor location
    Barrowsfield,
    /// Crow's Nest is a major location
    CrowsNest,
    /// Dullahan's Crest is a minor location
    DullahansCrest,
    /// Eapoe is a minor location
    Eapoe,
    /// Foxcatcher is a major location
    Foxcatcher,
    /// Frostmarch is a major location
    Frostmarch,
    /// Huntsfort is a major location
    Huntsfort,
    /// Kirkyard is a minor location
    Kirkyard,
    /// Necropolis is a major location
    Necropolis,
    /// Port of Rime is a major location
    PortofRime,
    /// Revenant's Path is a minor location
    RevenantsPath,
    /// Rime Wastes is a minor location
    RimeWastes,
    /// Shattered Advance is a major location
    ShatteredAdvance,
    /// Spirit Watch is a major location
    SpiritWatch,
    /// The Ivory Bank is a minor location
    TheIvoryBank,
    /// The Ivory Sea is a minor location
    TheIvorySea,
    /// The Spear is a minor location
    TheSpear,
    /// The Stand is a minor location
    TheStand,
    /// The Weathered Wall is a minor location
    TheWeatheredWall,
    /// The Weathering Halls is a major location
    TheWeatheringHalls,
    /// Wightwalk is a major location
    Wightwalk,
    /// Wraith's Gate is a minor location
    WraithsGate,
}

impl Location for WeatheredExpanse {
    fn is_major(&self) -> bool {
        match self {
            WeatheredExpanse::Bannerwatch => false,
            WeatheredExpanse::Barrowsfield => false,
            WeatheredExpanse::CrowsNest => true,
            WeatheredExpanse::DullahansCrest => false,
            WeatheredExpanse::Eapoe => false,
            WeatheredExpanse::Foxcatcher => true,
            WeatheredExpanse::Frostmarch => true,
            WeatheredExpanse::Huntsfort => true,
            WeatheredExpanse::Kirkyard => false,
            WeatheredExpanse::Necropolis => true,
            WeatheredExpanse::PortofRime => true,
            WeatheredExpanse::RevenantsPath => false,
            WeatheredExpanse::RimeWastes => false,
            WeatheredExpanse::ShatteredAdvance => true,
            WeatheredExpanse::SpiritWatch => true,
            WeatheredExpanse::TheIvoryBank => false,
            WeatheredExpanse::TheIvorySea => false,
            WeatheredExpanse::TheSpear => false,
            WeatheredExpanse::TheStand => false,
            WeatheredExpanse::TheWeatheredWall => false,
            WeatheredExpanse::TheWeatheringHalls => true,
            WeatheredExpanse::Wightwalk => true,
            WeatheredExpanse::WraithsGate => false,
        }
    }

    fn coords(&self) -> (f64, f64) {
        match self {
            WeatheredExpanse::Bannerwatch => (0.04483044, 0.87344664),
            WeatheredExpanse::Barrowsfield => (0.43067294, 0.4653978),
            WeatheredExpanse::CrowsNest => (0.27000988, 0.52244484),
            WeatheredExpanse::DullahansCrest => (0.14920744, 0.61690384),
            WeatheredExpanse::Eapoe => (0.4408721, 0.6718143),
            WeatheredExpanse::Foxcatcher => (0.5164818, 0.7780801),
            WeatheredExpanse::Frostmarch => (0.15492144, 0.4631941),
            WeatheredExpanse::Huntsfort => (0.48584694, 0.5349745),
            WeatheredExpanse::Kirkyard => (0.29680702, 0.622848),
            WeatheredExpanse::Necropolis => (0.34740227, 0.688721),
            WeatheredExpanse::PortofRime => (0.68206143, 0.12976617),
            WeatheredExpanse::RevenantsPath => (0.5494643, 0.15349369),
            WeatheredExpanse::RimeWastes => (0.2378497, 0.39045998),
            WeatheredExpanse::ShatteredAdvance => (0.38944197, 0.30851248),
            WeatheredExpanse::SpiritWatch => (0.6523898, 0.7255189),
            WeatheredExpanse::TheIvoryBank => (0.42168927, 0.92702323),
            WeatheredExpanse::TheIvorySea => (0.17670101, 0.7257215),
            WeatheredExpanse::TheSpear => (0.7011145, 0.3084089),
            WeatheredExpanse::TheStand => (0.089226045, 0.36507604),
            WeatheredExpanse::TheWeatheredWall => (0.18046375, 0.18551047),
            WeatheredExpanse::TheWeatheringHalls => (0.46095482, 0.157184),
            WeatheredExpanse::Wightwalk => (0.69054574, 0.49786863),
            WeatheredExpanse::WraithsGate => (0.5287201, 0.43321595),
        }
    }

    fn name_api(&self) -> (&str, &str) {
        (
            "WeatheredExpanse",
            match self {
                WeatheredExpanse::Bannerwatch => "Bannerwatch",
                WeatheredExpanse::Barrowsfield => "Barrowsfield",
                WeatheredExpanse::CrowsNest => "CrowsNest",
                WeatheredExpanse::DullahansCrest => "DullahansCrest",
                WeatheredExpanse::Eapoe => "Eapoe",
                WeatheredExpanse::Foxcatcher => "Foxcatcher",
                WeatheredExpanse::Frostmarch => "Frostmarch",
                WeatheredExpanse::Huntsfort => "Huntsfort",
                WeatheredExpanse::Kirkyard => "Kirkyard",
                WeatheredExpanse::Necropolis => "Necropolis",
                WeatheredExpanse::PortofRime => "PortofRime",
                WeatheredExpanse::RevenantsPath => "RevenantsPath",
                WeatheredExpanse::RimeWastes => "RimeWastes",
                WeatheredExpanse::ShatteredAdvance => "ShatteredAdvance",
                WeatheredExpanse::SpiritWatch => "SpiritWatch",
                WeatheredExpanse::TheIvoryBank => "TheIvoryBank",
                WeatheredExpanse::TheIvorySea => "TheIvorySea",
                WeatheredExpanse::TheSpear => "TheSpear",
                WeatheredExpanse::TheStand => "TheStand",
                WeatheredExpanse::TheWeatheredWall => "TheWeatheredWall",
                WeatheredExpanse::TheWeatheringHalls => "TheWeatheringHalls",
                WeatheredExpanse::Wightwalk => "Wightwalk",
                WeatheredExpanse::WraithsGate => "WraithsGate",
            },
        )
    }

    fn name_friendly(&self) -> (&str, &str) {
        (
            "WeatheredExpanse",
            match self {
                WeatheredExpanse::Bannerwatch => "Bannerwatch",
                WeatheredExpanse::Barrowsfield => "Barrowsfield",
                WeatheredExpanse::CrowsNest => "Crow's Nest",
                WeatheredExpanse::DullahansCrest => "Dullahan's Crest",
                WeatheredExpanse::Eapoe => "Eapoe",
                WeatheredExpanse::Foxcatcher => "Foxcatcher",
                WeatheredExpanse::Frostmarch => "Frostmarch",
                WeatheredExpanse::Huntsfort => "Huntsfort",
                WeatheredExpanse::Kirkyard => "Kirkyard",
                WeatheredExpanse::Necropolis => "Necropolis",
                WeatheredExpanse::PortofRime => "Port of Rime",
                WeatheredExpanse::RevenantsPath => "Revenant's Path",
                WeatheredExpanse::RimeWastes => "Rime Wastes",
                WeatheredExpanse::ShatteredAdvance => "Shattered Advance",
                WeatheredExpanse::SpiritWatch => "Spirit Watch",
                WeatheredExpanse::TheIvoryBank => "The Ivory Bank",
                WeatheredExpanse::TheIvorySea => "The Ivory Sea",
                WeatheredExpanse::TheSpear => "The Spear",
                WeatheredExpanse::TheStand => "The Stand",
                WeatheredExpanse::TheWeatheredWall => "The Weathered Wall",
                WeatheredExpanse::TheWeatheringHalls => "The Weathering Halls",
                WeatheredExpanse::Wightwalk => "Wightwalk",
                WeatheredExpanse::WraithsGate => "Wraith's Gate",
            },
        )
    }
}

/// Specific map details for the `LochMorHex` tile
pub enum LochMor {
    /// Bastard's Blade is a minor location
    BastardsBlade,
    /// Chattering Prairie is a minor location
    ChatteringPrairie,
    /// Escape is a minor location
    Escape,
    /// Fallen Fields is a minor location
    FallenFields,
    /// Feirmor is a major location
    Feirmor,
    /// Lake Severspring is a minor location
    LakeSeverspring,
    /// Loch Mor is a minor location
    LochMor,
    /// Market Road is a major location
    MarketRoad,
    /// Mercy's Wish is a major location
    MercysWish,
    /// Missing Bones is a minor location
    MissingBones,
    /// Moon's Copse is a major location
    MoonsCopse,
    /// Ousterdown is a minor location
    Ousterdown,
    /// Pockfields is a minor location
    Pockfields,
    /// Rip is a minor location
    Rip,
    /// Tear is a minor location
    Tear,
    /// The Founding Fields is a minor location
    TheFoundingFields,
    /// The Glean is a minor location
    TheGlean,
    /// The Reaping Road is a minor location
    TheReapingRoad,
    /// The Roilfort is a major location
    TheRoilfort,
    /// Tomb of the First is a major location
    TomboftheFirst,
    /// Westmarch is a major location
    Westmarch,
    /// Widow's Wail is a minor location
    WidowsWail,
}

impl Location for LochMor {
    fn is_major(&self) -> bool {
        match self {
            LochMor::BastardsBlade => false,
            LochMor::ChatteringPrairie => false,
            LochMor::Escape => false,
            LochMor::FallenFields => false,
            LochMor::Feirmor => true,
            LochMor::LakeSeverspring => false,
            LochMor::LochMor => false,
            LochMor::MarketRoad => true,
            LochMor::MercysWish => true,
            LochMor::MissingBones => false,
            LochMor::MoonsCopse => true,
            LochMor::Ousterdown => false,
            LochMor::Pockfields => false,
            LochMor::Rip => false,
            LochMor::Tear => false,
            LochMor::TheFoundingFields => false,
            LochMor::TheGlean => false,
            LochMor::TheReapingRoad => false,
            LochMor::TheRoilfort => true,
            LochMor::TomboftheFirst => true,
            LochMor::Westmarch => true,
            LochMor::WidowsWail => false,
        }
    }

    fn coords(&self) -> (f64, f64) {
        match self {
            LochMor::BastardsBlade => (0.21080314, 0.80878794),
            LochMor::ChatteringPrairie => (0.6458292, 0.30650327),
            LochMor::Escape => (0.7199372, 0.9053458),
            LochMor::FallenFields => (0.51105756, 0.5317777),
            LochMor::Feirmor => (0.5025071, 0.43424636),
            LochMor::LakeSeverspring => (0.13459364, 0.67867845),
            LochMor::LochMor => (0.46350145, 0.2706248),
            LochMor::MarketRoad => (0.20417798, 0.5151306),
            LochMor::MercysWish => (0.45450157, 0.13036008),
            LochMor::MissingBones => (0.8544347, 0.30717325),
            LochMor::MoonsCopse => (0.77075344, 0.5080013),
            LochMor::Ousterdown => (0.20533797, 0.66187173),
            LochMor::Pockfields => (0.5352733, 0.66082114),
            LochMor::Rip => (0.48696944, 0.7814131),
            LochMor::Tear => (0.6765988, 0.7775864),
            LochMor::TheFoundingFields => (0.51761055, 0.9304205),
            LochMor::TheGlean => (0.14058694, 0.49662516),
            LochMor::TheReapingRoad => (0.30018386, 0.03367163),
            LochMor::TheRoilfort => (0.5841027, 0.75919765),
            LochMor::TomboftheFirst => (0.26635438, 0.20696227),
            LochMor::Westmarch => (0.62941134, 0.15675731),
            LochMor::WidowsWail => (0.33518988, 0.76240003),
        }
    }

    fn name_api(&self) -> (&str, &str) {
        (
            "LochMor",
            match self {
                LochMor::BastardsBlade => "BastardsBlade",
                LochMor::ChatteringPrairie => "ChatteringPrairie",
                LochMor::Escape => "Escape",
                LochMor::FallenFields => "FallenFields",
                LochMor::Feirmor => "Feirmor",
                LochMor::LakeSeverspring => "LakeSeverspring",
                LochMor::LochMor => "LochMor",
                LochMor::MarketRoad => "MarketRoad",
                LochMor::MercysWish => "MercysWish",
                LochMor::MissingBones => "MissingBones",
                LochMor::MoonsCopse => "MoonsCopse",
                LochMor::Ousterdown => "Ousterdown",
                LochMor::Pockfields => "Pockfields",
                LochMor::Rip => "Rip",
                LochMor::Tear => "Tear",
                LochMor::TheFoundingFields => "TheFoundingFields",
                LochMor::TheGlean => "TheGlean",
                LochMor::TheReapingRoad => "TheReapingRoad",
                LochMor::TheRoilfort => "TheRoilfort",
                LochMor::TomboftheFirst => "TomboftheFirst",
                LochMor::Westmarch => "Westmarch",
                LochMor::WidowsWail => "WidowsWail",
            },
        )
    }

    fn name_friendly(&self) -> (&str, &str) {
        (
            "LochMor",
            match self {
                LochMor::BastardsBlade => "Bastard's Blade",
                LochMor::ChatteringPrairie => "Chattering Prairie",
                LochMor::Escape => "Escape",
                LochMor::FallenFields => "Fallen Fields",
                LochMor::Feirmor => "Feirmor",
                LochMor::LakeSeverspring => "Lake Severspring",
                LochMor::LochMor => "Loch Mor",
                LochMor::MarketRoad => "Market Road",
                LochMor::MercysWish => "Mercy's Wish",
                LochMor::MissingBones => "Missing Bones",
                LochMor::MoonsCopse => "Moon's Copse",
                LochMor::Ousterdown => "Ousterdown",
                LochMor::Pockfields => "Pockfields",
                LochMor::Rip => "Rip",
                LochMor::Tear => "Tear",
                LochMor::TheFoundingFields => "The Founding Fields",
                LochMor::TheGlean => "The Glean",
                LochMor::TheReapingRoad => "The Reaping Road",
                LochMor::TheRoilfort => "The Roilfort",
                LochMor::TomboftheFirst => "Tomb of the First",
                LochMor::Westmarch => "Westmarch",
                LochMor::WidowsWail => "Widow's Wail",
            },
        )
    }
}
