use std::collections::BTreeMap;
use std::collections::HashMap;
use std::io::Write;

use heck::*;
use records::*;
use std::mem::replace;

use std::iter::*;
use itertools::Itertools;
use codegen::IdReference::Constant;
use codegen::IdReference::Literal;

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Clone)]
enum IdReference {
    Constant(String),
    Literal(String)
}

impl IdReference {
    fn to_string(&self) -> String {
        match self {
            Constant(s) => s.to_string(),
            Literal(s) => s.to_string()
        }
    }
}

#[derive(Debug)]
struct YarpUnit {
    uid: String,
    id_constant: String,
    name: String,
    model: String,
}

impl YarpUnit {
    fn new(uid: String, name: String, model: String) -> YarpUnit {
        YarpUnit {
            id_constant: uid.to_shouty_snake_case(),
            uid,
            name,
            model,
        }
    }
}

#[derive(Debug)]
struct YarpBuilder {
    uid: String,
    id_constant: String,
    name: String,
    model: String,
    built_ids: Vec<IdReference>
}

impl YarpBuilder {
    fn new(uid: String, name: String, model: String, built_ids: Vec<IdReference>) -> YarpBuilder {
        YarpBuilder {
            id_constant: uid.to_shouty_snake_case(),
            uid,
            name,
            model,
            built_ids
        }
    }
}

struct YarpBuilding {
    uid: String,
    id_constant: String,
    name: String,
    model: String,
}

impl YarpBuilding {
    fn new(uid: String, name: String, model: String) -> YarpBuilding {
        YarpBuilding {
            id_constant: uid.to_shouty_snake_case(),
            uid,
            name,
            model,
        }
    }
}

struct YarpUnitShop {
    uid: String,
    id_constant: String,
    name: String,
    model: String,
    unit_ids: Vec<IdReference>,
    x: i32,
    y: i32,
}

impl YarpUnitShop {
    fn new(
        uid: String,
        name: String,
        model: String,
        unit_ids: Vec<IdReference>,
        x: i32,
        y: i32,
    ) -> YarpUnitShop {
        YarpUnitShop {
            id_constant: uid.to_shouty_snake_case(),
            uid,
            name,
            model,
            unit_ids,
            x,
            y,
        }
    }
}

pub struct CodeGenerator<W: Write> {
    writer: W,
    units: Vec<YarpUnit>,
    buildings: Vec<YarpBuilding>,
    uid_registry: BTreeMap<String, String>,
    model_registry: BTreeMap<IdReference, String>,
    current_unit_list: Vec<IdReference>,
    current_building_list: Vec<IdReference>,
    unit_shops: Vec<YarpUnitShop>,
    builders: Vec<YarpBuilder>,
    current_shop_x: i32,
    current_shop_y: i32,
    current_shop_model: String,
}

impl<W: Write> CodeGenerator<W> {
    pub fn new(writer: W) -> CodeGenerator<W> {
        CodeGenerator {
            writer,
            units: Vec::new(),
            buildings: Vec::new(),
            current_unit_list: Vec::new(),
            current_building_list: Vec::new(),
            uid_registry: BTreeMap::new(),
            model_registry: BTreeMap::new(),
            unit_shops: Vec::new(),
            builders: Vec::new(),
            current_shop_x: 0,
            current_shop_y: 0,
            current_shop_model: "".to_string()
        }
    }

    pub fn process_record(&mut self, record: Record) {
        match record {
            Record::CustomUnit(unit) => {
                if self.current_unit_list.len() > 11 {
                    panic!("Trying to add more than 12 units to a shop.");
                }

                let yunit = YarpUnit::new(unit.uid, unit.name, unit.model.trim().to_string());

                self.register_uid(&yunit.uid, &yunit.id_constant);
                self.current_shop_model = yunit.model.to_string();

                self.register_model(Constant(yunit.id_constant.to_string()), &yunit.model);
                self.current_unit_list.push(Constant(yunit.uid.clone()));
                self.units.push(yunit);
            }

            Record::UnitShop(shop) => {
                let old_shop = replace(&mut self.current_unit_list, Vec::new());
                println!("{:?}", old_shop);
                let unit_shop = YarpUnitShop::new(
                    shop.uid,
                    shop.name,
                    shop.model.unwrap_or_else(|| self.current_shop_model.clone()).trim().to_string(),
                    old_shop,
                    self.current_shop_x,
                    self.current_shop_y,
                );
                self.register_uid(&unit_shop.uid, &unit_shop.id_constant);
                self.unit_shops.push(unit_shop);

                self.current_shop_x += 128;

                if self.current_shop_x >= 1024 {
                    self.current_shop_x = 0;
                    self.current_shop_y -= 128;
                }
            }

            Record::CustomBuilding(building) => {
                if self.current_building_list.len() > 10 {
                    panic!("Trying to add more than 11 buildings to a builder.");
                }

                let ybuilding = YarpBuilding::new(building.uid, building.name, building.model.trim().to_string());

                self.register_uid(&ybuilding.uid, &ybuilding.id_constant);

                self.current_building_list.push(Constant(ybuilding.uid.clone()));
                self.buildings.push(ybuilding);
            },

            Record::StockUnit(unit) => {
                if self.current_unit_list.len() > 11 {
                    panic!("Trying to add more than 12 units to a shop.");
                }

                let id_reference = Literal(format!("'{}'", unit.id));
                self.current_unit_list.push(id_reference.clone());
                self.current_shop_model = unit.model;
            },

            Record::CustomBuilder(builder) => {
                if self.current_unit_list.len() > 11 {
                    panic!("Trying to add more than 12 builders to a shop.");
                }

                let old_builder = replace(&mut self.current_building_list, Vec::new());
                let builder = YarpBuilder::new(
                    builder.uid,
                    builder.name,
                    builder.model.unwrap_or_else(||self.current_shop_model.clone()).trim().to_string(),
                    old_builder
                );
                
                self.register_uid(&builder.uid, &builder.id_constant);
                self.current_unit_list.push(Constant(builder.uid.clone()));
                self.builders.push(builder);
            }

            Record::SetShopModel(record) => {
                self.current_shop_model = record.model;
            }

            Record::StockUnitModel(record) => {
                let id_reference = Literal(format!("'{}'", record.id));
                self.register_model(id_reference, &record.model.trim());
            }

            Record::ShopNewLine(record) => {
                self.current_shop_x = 0;
                self.current_shop_y -= 128;
            }
            _ => (),
        }
    }

    pub fn emit(&mut self) {
        let writer = &mut self.writer;
        let registry = &self.uid_registry;
        Self::emit_header(writer);

        self.units
            .iter()
            .for_each(|unit| Self::emit_constant_id(writer, &unit.id_constant));

        self.unit_shops
            .iter()
            .for_each(|shop| Self::emit_constant_id(writer, &shop.id_constant));

        self.buildings
            .iter()
            .for_each(|building| Self::emit_constant_id(writer, &building.id_constant));

        self.builders
            .iter()
            .for_each(|builder| Self::emit_constant_id(writer, &builder.id_constant));

        Self::emit_newline(writer);
        Self::emit_definitions_start(writer);

        self.units
            .iter()
            .for_each(|unit| Self::emit_unit_definition(writer, unit));

        Self::emit_newline(writer);

        self.builders
            .iter()
            .for_each(|builder| Self::emit_builder_definition(writer, registry, builder));

        Self::emit_newline(writer);

        self.unit_shops
            .iter()
            .for_each(|shop| Self::emit_shop_definition(writer, registry, shop));

        Self::emit_newline(writer);

        self.buildings
            .iter()
            .for_each(|building| Self::emit_building_definition(writer, building));

        Self::emit_newline(writer);
        Self::emit_shop_placement_start(writer);

        self.unit_shops
            .iter()
            .for_each(|shop| Self::emit_shop_placement(writer, shop));

        Self::emit_newline(writer);
        Self::emit_registry(writer, registry);
        Self::emit_newline(writer);
        Self::emit_reverse_registry(writer, registry);
        Self::emit_newline(writer);
        Self::emit_shop_registry(writer, &self.unit_shops);
        Self::emit_newline(writer);
        Self::emit_model_registry(writer, &self.model_registry);
    }

    fn register_uid(&mut self, uid: &str, id_constant: &str) {
        if self
            .uid_registry
            .insert(uid.to_string(), id_constant.to_string())
            .is_some()
        {
            panic!("Id collision! {}", uid)
        }
    }

    fn register_model(&mut self, id: IdReference, model: &str) {
        self.model_registry.insert(id, model.to_string());
    }

    fn emit_header(writer: &mut W) {
        writeln!(writer, "package AutoGenerated\nimport public CodegenUtils\n").unwrap();
    }

    fn emit_newline(writer: &mut W) {
        writeln!(writer).unwrap();
    }

    fn emit_constant_id(writer: &mut W, id: &str) {
        writeln!(
            writer,
            "constant {} = compiletime(UNIT_ID_GEN.next())",
            id
        ).unwrap();
    }

    fn emit_definitions_start(writer: &mut W) {
        writeln!(writer, "@compiletime function generate()").unwrap();
    }

    fn emit_unit_definition(writer: &mut W, unit: &YarpUnit) {
        writeln!(
            writer,
            "\tCodegenUtils.customUnit({}, \"{}\", \"{}\")",
            unit.id_constant, unit.model, unit.name
        ).unwrap();
    }

    fn emit_building_definition(writer: &mut W, building: &YarpBuilding) {
        writeln!(
            writer,
            "\tCodegenUtils.customBuilding({}, \"{}\", \"{}\")",
            building.id_constant, building.model, building.name
        ).unwrap();
    }

    fn emit_builder_definition(writer: &mut W, registry: &BTreeMap<String, String>, builder: &YarpBuilder) {
        writeln!(
            writer,
            "\tCodegenUtils.customBuilder({}, \"{}\", \"{}\", {})",
            builder.id_constant,
            builder.model,
            builder.name,
            builder.built_ids
                .iter()
                .map(|s| {
                    match s {
                        IdReference::Constant(s) => registry.get(s),
                        IdReference::Literal(s) => Some(s)
                    }
                })
                .filter_map(|s| s)
                .map(|s| s.to_string() + ".toRawCode()")
                .join(" + \",\" + ")
        );
    }

    fn emit_shop_definition(
        writer: &mut W,
        registry: &BTreeMap<String, String>,
        shop: &YarpUnitShop,
    ) {
        writeln!(
            writer,
            "\tCodegenUtils.unitShop({}, \"{}\", \"{}\", {})",
            shop.id_constant,
            shop.model,
            shop.name,
            shop.unit_ids
                .iter()
                .map(|s| {
                    match s {
                        IdReference::Constant(s) => registry.get(s),
                        IdReference::Literal(s) => Some(s)
                    }
                })
                .filter_map(|s| s)
                .map(|s| s.to_string() + ".toRawCode()")
                .join(" + \",\" + ")
        ).unwrap();
    }

    fn emit_shop_placement_start(writer: &mut W) {
        writeln!(writer, "public function placeShops(real x, real y)").unwrap();
        writeln!(writer, "\tlet offset = vec2(x, y)").unwrap();
        writeln!(writer, "\tlet owner = players[bj_PLAYER_NEUTRAL_EXTRA]").unwrap();
    }

    fn emit_shop_placement(writer: &mut W, shop: &YarpUnitShop) {
        writeln!(
            writer,
            "\tcreateUnit(owner, {}, offset + vec2({}, {}), angle(0))",
            shop.id_constant, shop.x, shop.y
        ).unwrap();
    }

    fn emit_registry(writer: &mut W, registry: &BTreeMap<String, String>) {
        let mut counter = 0;
        registry.iter().chunks(40).into_iter().enumerate().for_each(|(index, chunk)| {
            counter += 1;
            writeln!(
                writer,
                "@noinline\nfunction populateUidRegistry{}()",
                index
            ).unwrap();

            chunk.for_each(|(uid, id_constant)| {
                writeln!(
                    writer,
                    "\tregisterUid(compiletime(\"{}\".getHash()), {})",
                    uid, id_constant
                ).unwrap();
            });

            Self::emit_newline(writer);
        });

        writeln!(
            writer,
            "public function makeUidRegistry()"
        ).unwrap();

        for i in 0..counter {
            writeln!(writer, "\tpopulateUidRegistry{}()", i);
        }
    }

    fn emit_reverse_registry(writer: &mut W, registry: &BTreeMap<String, String>) {
        let mut counter = 0;
        registry.iter().chunks(40).into_iter().enumerate().for_each(|(index, chunk)| {
            counter += 1;
            writeln!(
                writer,
                "@noinline\nfunction populateUidReverseRegistry{}()",
                index
            ).unwrap();

            chunk.for_each(|(uid, id_constant)| {
                writeln!(
                    writer,
                    "\tregisterReverseUid({}, \"{}\")",
                    id_constant, uid
                ).unwrap();
            });

            Self::emit_newline(writer);
        });

        writeln!(
            writer,
            "public function makeUidReverseRegistry()"
        ).unwrap();

        for i in 0..counter {
            writeln!(writer, "\tpopulateUidReverseRegistry{}()", i);
        }
    }

    fn emit_shop_registry(writer: &mut W, unit_shops: &[YarpUnitShop]) {
        writeln!(
            writer,
            "public function makeShopRegistry() returns HashSet<int>"
        ).unwrap();
        writeln!(writer, "\tlet registry = new HashSet<int>").unwrap();

        for shop in unit_shops {
            writeln!(writer, "\tregistry.add({})", shop.id_constant).unwrap();
        }

        writeln!(writer, "\treturn registry").unwrap();
    }

    fn emit_model_registry(writer: &mut W, registry: &BTreeMap<IdReference, String>) {
        let mut counter = 0;
        registry.iter().chunks(40).into_iter().enumerate().for_each(|(index, chunk)| {
            counter += 1;

            writeln!(
                writer,
                "@noinline\nfunction populateModelRegistry{}()",
                index
            ).unwrap();

            chunk.for_each(|(id_reference, model)| {
                writeln!(
                    writer,
                    "\tregisterModel({}, \"{}\")",
                    id_reference.to_string(), model
                ).unwrap();
            });

            Self::emit_newline(writer);
        });

        writeln!(
            writer,
            "public function makeModelRegistry()"
        ).unwrap();

        for i in 0..counter {
            writeln!(writer, "\tpopulateModelRegistry{}()", i);
        }
    }
}
