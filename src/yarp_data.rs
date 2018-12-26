use crate::yarp_meta::*;
use fxhash::FxHashMap;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct YarpData {
    #[serde(flatten)]
    categories: FxHashMap<String, Vec<YarpDataUnitShop>>,
}

impl YarpData {
    pub fn from_meta(
        consumer_context: &RecordConsumerContext,
        id_registry: &IdRegistry,
        unit_registry: &UnitRegistry,
    ) -> YarpData {
        let mut yarp_data = YarpData::default();
        let mut shops = Vec::new();

        for unit in consumer_context
            .shop_queue
            .iter()
            .map(|s| unit_registry.get(s))
        {
            if let YarpUnit::Custom {
                uid,
                name,
                icon,
                model,
                variant: YarpUnitVariant::UnitShop { sold_ids },
            } = unit
            {
                shops.push(YarpDataUnitShop {
                    uid: uid.uid.to_string(),
                    name: name.to_string(),
                    model: model.to_string(),
                    row: 0,
                    col: 0,
                    sold: sold_ids
                        .iter()
                        .map(|s| {
                            let unit = unit_registry.get(s);
                            YarpDataUnit::from_unit(unit, id_registry, unit_registry)
                        })
                        .collect(),
                })
            }
        }

        let mut map = FxHashMap::default();
        map.insert("other".to_string(), shops);

        YarpData { categories: map }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct YarpDataUnitShop {
    uid: String,
    name: String,
    model: String,
    row: i32,
    col: i32,
    sold: Vec<YarpDataUnit>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum YarpDataUnit {
    Custom(YarpDataCustomUnit),
    Stock(YarpDataStockUnit),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct YarpDataCustomUnit {
    uid: String,
    name: String,
    model: String,
    icon: String,
    #[serde(flatten)]
    variant: YarpDataUnitVariant,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct YarpDataStockUnit {
    rawid: String,
}

impl YarpDataUnit {
    fn from_unit(
        unit: &YarpUnit,
        id_registry: &IdRegistry,
        unit_registry: &UnitRegistry,
    ) -> YarpDataUnit {
        if let YarpUnit::Custom {
            uid,
            name,
            icon,
            model,
            variant,
        } = unit
        {
            let variant = match variant {
                YarpUnitVariant::Unit => YarpDataUnitVariant::Unit,
                YarpUnitVariant::Building => YarpDataUnitVariant::Building,
                YarpUnitVariant::Builder { built_ids } => YarpDataUnitVariant::Builder {
                    built: built_ids
                        .iter()
                        .map(|s| {
                            YarpDataUnit::from_unit(
                                unit_registry.get(s),
                                id_registry,
                                unit_registry,
                            )
                        })
                        .collect(),
                },
                _ => unimplemented!(),
            };

            YarpDataUnit::Custom(YarpDataCustomUnit {
                uid: uid.uid.to_string(),
                name: name.to_string(),
                icon: icon.to_string(),
                model: model.to_string(),
                variant,
            })
        } else {
            unimplemented!()
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum YarpDataUnitVariant {
    #[serde(rename = "unit")]
    Unit,
    #[serde(rename = "building")]
    Building,
    #[serde(rename = "builder")]
    Builder { built: Vec<YarpDataUnit> },
}
