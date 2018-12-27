use crate::yarp_meta::*;
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct YarpData {
    pub shops: IndexMap<String, Vec<YarpDataUnitShop>>,
    pub stock_model_registry: IndexMap<String, String>,
}

impl YarpData {
    pub fn from_meta(
        // consumer_context: &RecordConsumerContext,
        id_registry: &IdRegistry,
        unit_registry: &UnitRegistry,
        model_registry: &ModelRegistry,
    ) -> YarpData {
        let mut unit_shops = Vec::new();

        // for unit in consumer_context
        //     .shop_queue
        //     .iter()
        //     .map(|s| unit_registry.get(s))
        for (_, unit) in unit_registry.registry.iter()
        {
            if let YarpUnit::Custom {
                id,
                name,
                model,
                variant: YarpUnitVariant::UnitShop { sold_ids, scale },
                ..
            } = unit
            {
                unit_shops.push(YarpDataUnitShop {
                    uid: id.uid().to_string(),
                    name: name.to_string(),
                    model: model.to_string(),
                    row: 0,
                    col: 0,
                    scale: *scale,
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

        let mut shops = IndexMap::default();
        shops.insert("other".to_string(), unit_shops);

        let mut stock_model_registry: IndexMap<String, String> = IndexMap::default();

        for (id, model) in model_registry.registry.iter() {
            if let UnitIdentifier::RawID { rawid } = id {
                stock_model_registry.insert(rawid.to_string(), model.to_string());
            }
        }

        YarpData {
            shops,
            stock_model_registry,
        }
    }
}

pub enum YarpDataModelEntry {
    ByRawid { rawid: String, model: String },
    ByUid { uid: String, model: String },
}

#[derive(Serialize, Deserialize, Debug)]
pub struct YarpDataUnitShop {
    pub uid: String,
    pub name: String,
    pub model: String,
    pub row: i32,
    pub col: i32,
    pub scale: f32,
    pub sold: Vec<YarpDataUnit>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum YarpDataUnit {
    Custom(YarpDataCustomUnit),
    Stock(YarpDataStockUnit),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct YarpDataCustomUnit {
    pub uid: String,
    pub name: String,
    pub model: String,
    pub icon: String,
    #[serde(flatten)]
    pub variant: YarpDataUnitVariant,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct YarpDataStockUnit {
    pub rawid: String,
    pub model: String,
}

impl YarpDataUnit {
    fn from_unit(
        unit: &YarpUnit,
        id_registry: &IdRegistry,
        unit_registry: &UnitRegistry,
    ) -> YarpDataUnit {
        if let YarpUnit::Custom {
            id,
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
                uid: id.uid().to_string(),
                name: name.to_string(),
                icon: icon.to_string(),
                model: model.to_string(),
                variant,
            })
        } else {
            YarpDataUnit::Stock(YarpDataStockUnit {
                rawid: unit.id().rawid().to_string(),
                model: unit.model().to_string(),
            })
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
