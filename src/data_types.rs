use serde::Deserialize;
use serde_repr::*;

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct TypeId {
    pub groupID: i32,
    pub categoryID: i32,
    pub capacity: Option<f64>,
    pub mass: Option<f64>,
    pub radius: Option<f64>,
    pub volume: Option<f64>,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct TypeDogmaAttribute {
    pub attributeID: i32,
    pub value: f64,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct TypeDogmaEffect {
    pub effectID: i32,
    pub isDefault: bool,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct TypeDogma {
    pub dogmaAttributes: Vec<TypeDogmaAttribute>,
    pub dogmaEffects: Vec<TypeDogmaEffect>,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct DogmaAttribute {
    pub defaultValue: f64,
    pub highIsGood: bool,
    pub stackable: bool,
}

#[allow(non_snake_case)]
#[derive(Deserialize_repr, Debug)]
#[repr(i32)]
pub enum DogmaEffectModifierInfoDomain {
    ItemID = 0,
    ShipID = 1,
    CharID = 2,
    OtherID = 3,
    StructureID = 4,
    Target = 5,
    TargetID = 6,
}

#[allow(non_snake_case)]
#[derive(Deserialize_repr, Debug)]
#[repr(i32)]
pub enum DogmaEffectModifierInfoFunc {
    ItemModifier = 0,
    LocationGroupModifier = 1,
    LocationModifier = 2,
    LocationRequiredSkillModifier = 3,
    OwnerRequiredSkillModifier = 4,
    EffectStopper = 5,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct DogmaEffectModifierInfo {
    pub domain: DogmaEffectModifierInfoDomain,
    pub func: DogmaEffectModifierInfoFunc,
    pub modifiedAttributeID: Option<i32>,
    pub modifyingAttributeID: Option<i32>,
    pub operation: Option<i32>,
    pub groupID: Option<i32>,
    pub skillTypeID: Option<i32>,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct DogmaEffect {
    pub dischargeAttributeID: Option<i32>,
    pub durationAttributeID: Option<i32>,
    pub effectCategory: i32,
    pub electronicChance: bool,
    pub isAssistance: bool,
    pub isOffensive: bool,
    pub isWarpSafe: bool,
    pub propulsionChance: bool,
    pub rangeChance: bool,
    pub rangeAttributeID: Option<i32>,
    pub falloffAttributeID: Option<i32>,
    pub trackingSpeedAttributeID: Option<i32>,
    pub fittingUsageChanceAttributeID: Option<i32>,
    pub resistanceAttributeID: Option<i32>,
    pub modifierInfo: Vec<DogmaEffectModifierInfo>,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
pub enum EsiState {
    Passive,
    Online,
    Active,
    Overload,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct EsiItem {
    pub type_id: i32,
    pub quantity: i32,
    pub flag: i32,
    pub state: Option<EsiState>,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct EsiFit {
    pub ship_type_id: i32,
    pub items: Vec<EsiItem>,
}
