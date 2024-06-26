use super::item::{Attribute, EffectCategory, Item};
use super::{Info, Pass, Ship};
use crate::data_types::EsiState;

const ATTRIBUTE_MASS_ID: i32 = 4;
const ATTRIBUTE_CAPACITY_ID: i32 = 38;
const ATTRIBUTE_VOLUME_ID: i32 = 161;
const ATTRIBUTE_RADIUS_ID: i32 = 162;
const ATTRIBUTE_SKILL_LEVEL_ID: i32 = 280;

const ESI_FLAG_FILTER: [i32; 32] = [
    11, 12, 13, 14, 15, 16, 17, 18, // lowslots
    19, 20, 21, 22, 23, 24, 25, 26, // medslots
    27, 28, 29, 30, 31, 32, 33, 34, // hislots
    87, // dronebay
    92, 93, 94, // rigs
    125, 126, 127, 128, // subsystems
];

pub struct PassOne {}

impl Item {
    pub fn set_attribute(&mut self, attribute_id: i32, value: f64) {
        self.attributes.insert(attribute_id, Attribute::new(value));
    }

    fn set_attributes(&mut self, info: &Info) {
        for dogma_attribute in info.get_dogma_attributes(self.type_id) {
            self.set_attribute(dogma_attribute.attributeID, dogma_attribute.value);
        }

        /* Some attributes of items come from the TypeID information. */
        let type_id = info.get_type_id(self.type_id);
        if let Some(mass) = type_id.mass {
            self.set_attribute(ATTRIBUTE_MASS_ID, mass);
        }
        if let Some(capacity) = type_id.capacity {
            self.set_attribute(ATTRIBUTE_CAPACITY_ID, capacity);
        }
        if let Some(volume) = type_id.volume {
            self.set_attribute(ATTRIBUTE_VOLUME_ID, volume);
        }
        if let Some(radius) = type_id.radius {
            self.set_attribute(ATTRIBUTE_RADIUS_ID, radius);
        }
    }
}

impl Pass for PassOne {
    fn pass(info: &Info, ship: &mut Ship) {
        ship.hull.set_attributes(info);

        for (skill_id, skill_level) in info.skills {
            let mut skill = Item::new_fake(*skill_id);

            skill.set_attributes(info);
            skill.set_attribute(ATTRIBUTE_SKILL_LEVEL_ID, *skill_level as f64);

            ship.skills.push(skill);
        }

        for esi_item in &info.esi_fit.items {
            /* Only process items that are in slots, not in cargo etc. */
            if !ESI_FLAG_FILTER.contains(&esi_item.flag) {
                continue;
            }

            let state = match esi_item.state {
                None => EffectCategory::Active,
                Some(EsiState::Passive) => EffectCategory::Passive,
                Some(EsiState::Online) => EffectCategory::Online,
                Some(EsiState::Active) => EffectCategory::Active,
                Some(EsiState::Overload) => EffectCategory::Overload,
            };
            for _ in 0..esi_item.quantity {
                let mut item = Item::new_esi(
                    esi_item.type_id,
                    1,
                    esi_item.flag,
                    esi_item.charge.as_ref().map(|charge| charge.type_id),
                    state,
                );

                item.set_attributes(info);
                item.charge
                    .as_mut()
                    .map(|charge| charge.set_attributes(info));

                ship.items.push(item);
            }
        }
    }
}
