use super::item::Attribute;
use super::{Info, Item, Pass, Ship};

pub struct PassFour {}

mod align_time;
mod capacitor;
mod cpu_power;
mod damage;
mod drone;
mod ehp;
mod recharge;
mod scan_strength;

#[allow(non_camel_case_types)]
pub enum AttributeId {
    alignTime = -1,
    scanStrength = -2,
    cpuUsed = -3,
    powerUsed = -4,
    cpuUnused = -5,
    powerUnused = -6,
    #[allow(dead_code)]
    velocityBoost = -7, // Is taken care of with effects.
    shieldEhpMultiplier = -8,
    armorEhpMultiplier = -9,
    hullEhpMultiplier = -10,
    shieldEhp = -11,
    armorEhp = -12,
    hullEhp = -13,
    ehp = -14,
    passiveShieldRecharge = -15,
    shieldRecharge = -16,
    armorRecharge = -17,
    hullRecharge = -18,
    passiveShieldRechargeEhp = -19,
    shieldRechargeEhp = -20,
    armorRechargeEhp = -21,
    hullRechargeEhp = -22,
    capacitorPeakRecharge = -23,
    capacitorPeakUsage = -24,
    capacitorPeakDelta = -25,
    capacitorPeakDeltaPercentage = -26,
    capacitorDepletesIn = -27,
    damageWithoutReloadDps = -28,
    damageWithReloadDps = -29,
    damageAlphaHp = -30,
    droneActive = -31,
    droneBandwidthUsedTotal = -32,
    droneDamageAlphaHp = -33,
    droneDamageDps = -34,
    droneCapacityUsed = -35,

    mass = 4,
    capacitorNeed = 6,
    hp = 9,
    powerOutput = 11,
    power = 30,
    capacity = 38,

    cpuOutput = 48,
    cpu = 50,
    speed = 51,
    rechargeRate = 55,
    damageMultiplier = 64,
    shieldBonus = 68,
    agility = 70,
    duration = 73,
    structureDamageAmount = 83,
    armorDamageAmount = 84,

    kineticDamageResonance = 109,
    thermalDamageResonance = 110,
    explosiveDamageResonance = 111,
    emDamageResonance = 113,

    emDamage = 114,
    explosiveDamage = 116,
    kineticDamage = 117,
    thermalDamage = 118,

    volume = 161,

    scanRadarStrength = 208,
    scanLadarStrength = 209,
    scanMagnetometricStrength = 210,
    scanGravimetricStrength = 211,

    shieldCapacity = 263,
    armorHp = 265,

    armorEmDamageResonance = 267,
    armorExplosiveDamageResonance = 268,
    armorKineticDamageResonance = 269,
    armorThermalDamageResonance = 270,
    shieldEmDamageResonance = 271,
    shieldExplosiveDamageResonance = 272,
    shieldKineticDamageResonance = 273,
    shieldThermalDamageResonance = 274,

    shieldRechargeRate = 479,
    capacitorCapacity = 482,

    droneBandwidthUsed = 1272,
    reloadTime = 1795,
}

impl Item {
    pub fn add_attribute(&mut self, attribute_id: AttributeId, base_value: f64, value: f64) {
        let mut attribute = Attribute::new(base_value);
        attribute.value = Some(value);
        self.attributes.insert(attribute_id as i32, attribute);
    }
}

/* Attributes don't contain all information displayed, so we calculate some fake attributes with those values. */
impl Pass for PassFour {
    fn pass(_info: &Info, ship: &mut Ship) {
        align_time::attribute_align_time(ship);
        scan_strength::attribute_scan_strength(ship);

        cpu_power::attribute_cpu_used(ship);
        cpu_power::attribute_power_used(ship);
        cpu_power::attribute_cpu_unused(ship);
        cpu_power::attribute_power_unused(ship);

        ehp::attribute_shield_ehp_multiplier(ship);
        ehp::attribute_armor_ehp_multiplier(ship);
        ehp::attribute_hull_ehp_multiplier(ship);
        ehp::attribute_shield_ehp(ship);
        ehp::attribute_armor_ehp(ship);
        ehp::attribute_hull_ehp(ship);
        ehp::attribute_ehp(ship);

        recharge::attribute_passive_shield_recharge(ship);
        recharge::attribute_shield_recharge(ship);
        recharge::attribute_armor_recharge(ship);
        recharge::attribute_hull_recharge(ship);

        capacitor::attribute_capacitor_peak_recharge(ship);
        capacitor::attribute_capacitor_peak_usage(ship);
        capacitor::attribute_capacitor_peak_delta(ship);
        capacitor::attribute_capacitor_peak_delta_percentage(ship);
        capacitor::attribute_capacitor_depletes_in(ship);

        damage::attribute_damage_alpha_hp(ship);
        damage::attribute_damage_without_reload(ship);
        damage::attribute_damage_with_reload(ship);

        drone::attribute_drone_active(ship);
        drone::attribute_drone_capacity_used(ship);
        drone::attribute_drone_bandwidth_used(ship);
        drone::attribute_drone_damage(ship);
    }
}
