use crate::records::*;
use crate::yarp_data::*;
use heck::*;
use idcontain::{Id, IdSlab};
use indexmap::IndexMap;
use std::mem::replace;

#[derive(Debug, Default)]
pub struct IdRegistry {
    slab: IdSlab<UnitIdentifier>,
    // by_uid: IndexMap<String, Id<UnitIdentifier>>,
    // by_rawid: IndexMap<String, Id<UnitIdentifier>>,
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
    pub registry: IndexMap<UnitIdentifier, YarpUnit>,
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

#[derive(Debug)]
pub enum YarpUnitVariant {
    Unit,
    Building,
    UnitShop {
        sold_ids: Vec<UnitIdentifier>,
        scale: f32,
    },
    Builder {
        built_ids: Vec<UnitIdentifier>,
    },
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
        sold_ids: &[UnitIdentifier],
        scale: f32,
    ) -> YarpUnit {
        YarpUnit::Custom {
            id,
            name,
            icon: "".to_string(),
            model: model.trim().to_string(),
            variant: YarpUnitVariant::UnitShop {
                sold_ids: sold_ids.into(),
                scale,
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

    fn new_with_variant(
        id: UnitIdentifier,
        name: String,
        model: String,
        icon: String,
        variant: YarpUnitVariant,
    ) -> YarpUnit {
        YarpUnit::Custom {
            id,
            name,
            icon,
            model: model.trim().to_string(),
            variant,
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
pub struct ModelRegistry {
    pub registry: IndexMap<UnitIdentifier, String>,
}

impl ModelRegistry {
    fn insert(&mut self, id: &UnitIdentifier, model: String) {
        self.registry.insert(id.clone(), model);
    }
}

#[derive(Default)]
pub struct RecordConsumerContext {
    pub unit_queue: Vec<UnitIdentifier>,
    pub building_queue: Vec<UnitIdentifier>,
    pub shop_queue: Vec<UnitIdentifier>,
    pub current_shop_model: String,
    pub current_icon_path: String,
    pub current_scale: f32,
}

pub fn consume_record(
    record: Record,
    consumer_context: &mut RecordConsumerContext,
    id_registry: &mut IdRegistry,
    unit_registry: &mut UnitRegistry,
    model_registry: &mut ModelRegistry,
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
            model_registry.insert(&id, yarp_unit.model().to_string());
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
            model_registry.insert(&id, yarp_unit.model().to_string());
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
            model_registry.insert(&id, yarp_unit.model().to_string());
            consumer_context.unit_queue.push(id);
            unit_registry.insert(yarp_unit);
        }
        Record::UnitShop(shop) => {
            let id = id_registry.insert(UnitIdentifier::new_custom(shop.uid.to_string()));
            let yarp_unit = YarpUnit::new_shop(
                id.clone(),
                shop.name.to_string(),
                consumer_context.current_shop_model.to_string(),
                &consumer_context.unit_queue,
                consumer_context.current_scale,
            );
            model_registry.insert(&id, yarp_unit.model().to_string());
            consumer_context.unit_queue.clear();
            unit_registry.insert(yarp_unit);
            consumer_context.shop_queue.push(id);
        }
        Record::StockUnit(unit) => {
            let id = id_registry.insert(UnitIdentifier::new_stock(unit.id.to_string()));
            let yarp_unit = YarpUnit::new_stock(id.clone(), unit.model.to_string());
            model_registry.insert(&id, yarp_unit.model().to_string());
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
        Record::StockUnitModel(model) => {
            model_registry.insert(
                &UnitIdentifier::new_stock(model.id.to_string()),
                model.model.to_string(),
            );
        }
        Record::SetShopScale(scale) => {
            consumer_context.current_scale = scale.scale;
        }

        _ => {}
    }
}

#[derive(Default)]
pub struct Registries {
    pub id: IdRegistry,
    pub unit: UnitRegistry,
    pub model: ModelRegistry,
}

fn transform_yarp_data_unit(unit: &YarpDataUnit, registries: &mut Registries) -> UnitIdentifier {
    match unit {
        YarpDataUnit::Custom(custom_unit) => {
            let variant = match &custom_unit.variant {
                YarpDataUnitVariant::Unit => YarpUnitVariant::Unit,
                YarpDataUnitVariant::Building => YarpUnitVariant::Building,
                YarpDataUnitVariant::Builder { built } => YarpUnitVariant::Builder {
                    built_ids: built
                        .iter()
                        .map(|s| transform_yarp_data_unit(s, registries))
                        .collect(),
                },
            };

            let id = registries
                .id
                .insert(UnitIdentifier::new_custom(custom_unit.uid.to_string()));

            let yarp_unit = YarpUnit::new_with_variant(
                id.clone(),
                custom_unit.name.to_string(),
                custom_unit.model.to_string(),
                custom_unit.icon.to_string(),
                variant,
            );

            registries.unit.insert(yarp_unit);

            id
        }
        YarpDataUnit::Stock(stock_unit) => {
            let id = registries
                .id
                .insert(UnitIdentifier::new_stock(stock_unit.rawid.to_string()));
            let yarp_unit = YarpUnit::new_stock(id.clone(), stock_unit.model.to_string());
            registries.unit.insert(yarp_unit);

            id
        }
    }
}

pub fn transform_yarp_data(data: &YarpData) -> Registries {
    let mut registries = Registries::default();

    for unit_shop in data.shops.iter().flat_map(|(_, s)| s.iter()) {
        let mut sold_ids: Vec<UnitIdentifier> = Vec::new();

        for unit in unit_shop.sold.iter() {
            sold_ids.push(transform_yarp_data_unit(unit, &mut registries));
        }

        let id = registries
            .id
            .insert(UnitIdentifier::new_custom(unit_shop.uid.to_string()));
        let yarp_unit = YarpUnit::new_shop(
            id,
            unit_shop.name.to_string(),
            unit_shop.model.to_string(),
            &sold_ids,
            unit_shop.scale,
        );
        registries.unit.insert(yarp_unit);
    }

    for (rawid, model) in data.stock_model_registry.iter() {
        let id = registries.id.insert(UnitIdentifier::new_stock(rawid.to_string()));
        registries.model.insert(&id, model.to_string());
    }

    registries
}
