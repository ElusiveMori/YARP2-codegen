use crate::records::*;
use fxhash::FxHashMap as HashMap;
use heck::*;
use idcontain::{Id, IdSlab};
use std::mem::replace;

#[derive(Debug, Default)]
pub struct IdRegistry {
    slab: IdSlab<UniqueId>,
    by_uid: HashMap<String, Id<UniqueId>>,
    // by_rawid: HashMap<String, Id<UniqueId>>,
}

impl IdRegistry {
    fn insert(&mut self, uid: UniqueId) -> UniqueId {
        let str_uid = uid.uid.to_string();

        let id = self.slab.insert(uid);

        self.by_uid.insert(str_uid, id);
        // self.by_rawid.insert(str_rawid, id);

        self.slab[id].clone()
    }

    fn get_by_uid<'a, 'b>(&'a self, uid: &'b str) -> &'a UniqueId {
        &self.slab[self.by_uid[uid]]
    }

    // fn get_by_rawid<'a, 'b>(&'a self, rawid: &'b str) -> &'a UniqueId {
    //     &self.slab[self.by_rawid[rawid]]
    // }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum IdLiteral {
    Raw(String),
    Constant(String),
}

impl IdLiteral {
    fn new_raw(input: String) -> IdLiteral {
        IdLiteral::Raw(input)
    }

    fn new_constant(input: &str) -> IdLiteral {
        IdLiteral::Constant(input.to_shouty_snake_case())
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct UniqueId {
    pub uid: String,
    pub rawid: IdLiteral,
}

impl UniqueId {
    fn new(uid: String, rawid: IdLiteral) -> UniqueId {
        UniqueId { uid, rawid }
    }
}

#[derive(Debug, Default)]
pub struct UnitRegistry {
    pub registry: HashMap<UniqueId, YarpUnit>,
}

impl UnitRegistry {
    fn insert(&mut self, unit: YarpUnit) {
        if let YarpUnit::Custom {uid, ..} = &unit {
            self.registry.insert(uid.clone(), unit);
        }
    }

    fn get_mut(&mut self, uid: &UniqueId) -> &mut YarpUnit {
        self.registry.get_mut(uid).unwrap()
    }

    pub fn get(&self, uid: &UniqueId) -> &YarpUnit {
        &self.registry[uid]
    }
}

pub struct ModelRegistry {
    registry: HashMap<String, String>,
}

#[derive(Debug)]
pub enum YarpUnitVariant {
    Unit,
    Building,
    UnitShop { sold_ids: Vec<UniqueId> },
    Builder { built_ids: Vec<UniqueId> },
}

#[derive(Debug)]
pub enum YarpUnit {
    Custom {
        uid: UniqueId,
        variant: YarpUnitVariant,
        name: String,
        model: String,
        icon: String,
    },
    Stock,
}

impl YarpUnit {
    fn new_unit(uid: UniqueId, name: String, model: String, icon: String) -> YarpUnit {
        YarpUnit::Custom {
            uid,
            name,
            icon,
            model: model.trim().to_string(),
            variant: YarpUnitVariant::Unit,
        }
    }

    fn new_building(uid: UniqueId, name: String, model: String, icon: String) -> YarpUnit {
        YarpUnit::Custom {
            uid,
            name,
            icon,
            model: model.trim().to_string(),
            variant: YarpUnitVariant::Building,
        }
    }

    fn new_shop(
        uid: UniqueId,
        name: String,
        model: String,
        icon: String,
        sold_ids: &[UniqueId],
    ) -> YarpUnit {
        YarpUnit::Custom {
            uid,
            name,
            icon,
            model: model.trim().to_string(),
            variant: YarpUnitVariant::UnitShop {
                sold_ids: sold_ids.into(),
            },
        }
    }

    fn new_builder(
        uid: UniqueId,
        name: String,
        model: String,
        icon: String,
        built_ids: &[UniqueId],
    ) -> YarpUnit {
        YarpUnit::Custom {
            uid,
            name,
            icon,
            model: model.trim().to_string(),
            variant: YarpUnitVariant::Builder {
                built_ids: built_ids.into(),
            },
        }
    }
}

#[derive(Default)]
pub struct RecordConsumerContext {
    pub unit_queue: Vec<UniqueId>,
    pub building_queue: Vec<UniqueId>,
    pub shop_queue: Vec<UniqueId>,
    pub current_shop_model: String,
    pub current_icon_path: String,
}

pub fn consume_record(
    record: Record,
    consumer_context: &mut RecordConsumerContext,
    id_registry: &mut IdRegistry,
    unit_registry: &mut UnitRegistry,
) {
    match record {
        Record::CustomUnit(unit) => {
            let id_constant = IdLiteral::new_constant(unit.uid);
            let uid = id_registry.insert(UniqueId::new(unit.uid.to_string(), id_constant));
            let yarp_unit = YarpUnit::new_unit(
                uid.clone(),
                unit.name.to_string(),
                unit.model.to_string(),
                consumer_context.current_icon_path.to_string(),
            );
            consumer_context.unit_queue.push(uid);
            unit_registry.insert(yarp_unit);
        }
        Record::CustomBuilding(building) => {
            let id_constant = IdLiteral::new_constant(building.uid);
            let uid = id_registry.insert(UniqueId::new(building.uid.to_string(), id_constant));
            let yarp_unit = YarpUnit::new_building(
                uid.clone(),
                building.name.to_string(),
                building.model.to_string(),
                consumer_context.current_icon_path.to_string(),
            );
            consumer_context.building_queue.push(uid);
            unit_registry.insert(yarp_unit);
        }
        Record::CustomBuilder(builder) => {
            let id_constant = IdLiteral::new_constant(builder.uid);
            let uid = id_registry.insert(UniqueId::new(builder.uid.to_string(), id_constant));
            let yarp_unit = YarpUnit::new_builder(
                uid.clone(),
                builder.name.to_string(),
                "".to_string(),
                consumer_context.current_icon_path.to_string(),
                &consumer_context.building_queue,
            );
            consumer_context.building_queue.clear();
            consumer_context.unit_queue.push(uid);
            unit_registry.insert(yarp_unit);
        }
        Record::UnitShop(shop) => {
            let id_constant = IdLiteral::new_constant(shop.uid);
            let uid = id_registry.insert(UniqueId::new(shop.uid.to_string(), id_constant));
            let yarp_unit = YarpUnit::new_shop(
                uid.clone(),
                shop.name.to_string(),
                consumer_context.current_shop_model.to_string(),
                consumer_context.current_icon_path.to_string(),
                &consumer_context.unit_queue,
            );
            consumer_context.unit_queue.clear();
            unit_registry.insert(yarp_unit);
            consumer_context.shop_queue.push(uid);
        }
        Record::SetShopModel(model) => {
            consumer_context.current_shop_model = model.model.to_string();
        }
        Record::SetDefaultIcon(icon) => {
            consumer_context.current_icon_path = icon.icon.to_string();
        }

        _ => {}
    }
}
