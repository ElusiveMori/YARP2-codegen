use crate::records::*;
use fxhash::FxHashMap as HashMap;
use heck::*;
use idcontain::{Id, IdSlab};
use std::mem::replace;

#[derive(Debug, Default)]
pub struct IdRegistry {
    slab: IdSlab<UnitIdentifier>,
    // by_uid: HashMap<String, Id<UnitIdentifier>>,
    // by_rawid: HashMap<String, Id<UnitIdentifier>>,
}

impl IdRegistry {
    fn insert(&mut self, id: UnitIdentifier) -> UnitIdentifier {
        let id = self.slab.insert(id);

        // self.by_uid.insert(str_uid, id);
        // self.by_rawid.insert(str_rawid, id);

        self.slab[id].clone()
    }

    // fn get_by_uid<'a, 'b>(&'a self, uid: &'b str) -> &'a UnitIdentifier {
    //     &self.slab[self.by_uid[uid]]
    // }

    // fn get_by_rawid<'a, 'b>(&'a self, constant_name: &'b str) -> &'a UnitIdentifier {
    //     &self.slab[self.by_rawid[constant_name]]
    // }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum UnitIdentifier {
    UID { uid: String, constant_name: String },
    RawID { rawid: String },
}

impl UnitIdentifier {
    fn new_custom(uid: String) -> UnitIdentifier {
        UnitIdentifier::UID {
            constant_name: uid.to_shouty_snake_case(),
            uid,
        }
    }

    fn new_stock(rawid: String) -> UnitIdentifier {
        UnitIdentifier::RawID { rawid }
    }

    pub fn uid(&self) -> &str {
        if let UnitIdentifier::UID { uid, .. } = &self {
            &uid
        } else {
            panic!("cannot call .uid() on non-UID variant")
        }
    }

    pub fn rawid(&self) -> &str {
        if let UnitIdentifier::RawID { rawid } = &self {
            &rawid
        } else {
            panic!("cannot call .rawid() on non-RawID variant")
        }
    }
}

#[derive(Debug, Default)]
pub struct UnitRegistry {
    pub registry: HashMap<UnitIdentifier, YarpUnit>,
}

impl UnitRegistry {
    fn insert(&mut self, unit: YarpUnit) {
        self.registry.insert(unit.id().clone(), unit);
    }

    fn get_mut(&mut self, id: &UnitIdentifier) -> &mut YarpUnit {
        self.registry.get_mut(id).unwrap()
    }

    pub fn get(&self, id: &UnitIdentifier) -> &YarpUnit {
        &self.registry[id]
    }
}

pub struct ModelRegistry {
    registry: HashMap<String, String>,
}

#[derive(Debug)]
pub enum YarpUnitVariant {
    Unit,
    Building,
    UnitShop { sold_ids: Vec<UnitIdentifier> },
    Builder { built_ids: Vec<UnitIdentifier> },
}

#[derive(Debug)]
pub enum YarpUnit {
    Custom {
        id: UnitIdentifier,
        variant: YarpUnitVariant,
        name: String,
        model: String,
        icon: String,
    },
    Stock {
        id: UnitIdentifier,
        model: String,
    },
}

impl YarpUnit {
    fn new_unit(id: UnitIdentifier, name: String, model: String, icon: String) -> YarpUnit {
        YarpUnit::Custom {
            id,
            name,
            icon,
            model: model.trim().to_string(),
            variant: YarpUnitVariant::Unit,
        }
    }

    fn new_building(id: UnitIdentifier, name: String, model: String, icon: String) -> YarpUnit {
        YarpUnit::Custom {
            id,
            name,
            icon,
            model: model.trim().to_string(),
            variant: YarpUnitVariant::Building,
        }
    }

    fn new_shop(
        id: UnitIdentifier,
        name: String,
        model: String,
        icon: String,
        sold_ids: &[UnitIdentifier],
    ) -> YarpUnit {
        YarpUnit::Custom {
            id,
            name,
            icon,
            model: model.trim().to_string(),
            variant: YarpUnitVariant::UnitShop {
                sold_ids: sold_ids.into(),
            },
        }
    }

    fn new_builder(
        id: UnitIdentifier,
        name: String,
        model: String,
        icon: String,
        built_ids: &[UnitIdentifier],
    ) -> YarpUnit {
        YarpUnit::Custom {
            id,
            name,
            icon,
            model: model.trim().to_string(),
            variant: YarpUnitVariant::Builder {
                built_ids: built_ids.into(),
            },
        }
    }

    fn new_stock(id: UnitIdentifier, model: String) -> YarpUnit {
        YarpUnit::Stock { id, model }
    }

    pub fn id(&self) -> &UnitIdentifier {
        match &self {
            YarpUnit::Custom { id, .. } => id,
            YarpUnit::Stock { id, .. } => id,
        }
    }

    pub fn model(&self) -> &str {
        match &self {
            YarpUnit::Custom { model, .. } => model,
            YarpUnit::Stock { model, .. } => model,
        }
    }
}

#[derive(Default)]
pub struct RecordConsumerContext {
    pub unit_queue: Vec<UnitIdentifier>,
    pub building_queue: Vec<UnitIdentifier>,
    pub shop_queue: Vec<UnitIdentifier>,
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
            let id = id_registry.insert(UnitIdentifier::new_custom(unit.uid.to_string()));
            let yarp_unit = YarpUnit::new_unit(
                id.clone(),
                unit.name.to_string(),
                unit.model.to_string(),
                consumer_context.current_icon_path.to_string(),
            );
            consumer_context.unit_queue.push(id);
            unit_registry.insert(yarp_unit);
        }
        Record::CustomBuilding(building) => {
            let id = id_registry.insert(UnitIdentifier::new_custom(building.uid.to_string()));
            let yarp_unit = YarpUnit::new_building(
                id.clone(),
                building.name.to_string(),
                building.model.to_string(),
                consumer_context.current_icon_path.to_string(),
            );
            consumer_context.building_queue.push(id);
            unit_registry.insert(yarp_unit);
        }
        Record::CustomBuilder(builder) => {
            let id = id_registry.insert(UnitIdentifier::new_custom(builder.uid.to_string()));
            let yarp_unit = YarpUnit::new_builder(
                id.clone(),
                builder.name.to_string(),
                "".to_string(),
                consumer_context.current_icon_path.to_string(),
                &consumer_context.building_queue,
            );
            consumer_context.building_queue.clear();
            consumer_context.unit_queue.push(id);
            unit_registry.insert(yarp_unit);
        }
        Record::UnitShop(shop) => {
            let id = id_registry.insert(UnitIdentifier::new_custom(shop.uid.to_string()));
            let yarp_unit = YarpUnit::new_shop(
                id.clone(),
                shop.name.to_string(),
                consumer_context.current_shop_model.to_string(),
                consumer_context.current_icon_path.to_string(),
                &consumer_context.unit_queue,
            );
            consumer_context.unit_queue.clear();
            unit_registry.insert(yarp_unit);
            consumer_context.shop_queue.push(id);
        }
        Record::StockUnit(unit) => {
            let id = id_registry.insert(UnitIdentifier::new_stock(unit.id.to_string()));
            let yarp_unit = YarpUnit::new_stock(id.clone(), unit.model.to_string());
            consumer_context.current_shop_model = unit.model.to_string();
            consumer_context.unit_queue.push(id);
            unit_registry.insert(yarp_unit);
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
