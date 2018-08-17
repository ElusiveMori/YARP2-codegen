use std::collections::BTreeMap;
use std::collections::HashMap;
use std::io::Write;

use heck::*;
use records::*;
use std::mem::replace;

use itertools::Itertools;

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

struct YarpUnitShop {
    uid: String,
    id_constant: String,
    name: String,
    model: String,
    unit_ids: Vec<String>,
    x: i32,
    y: i32,
}

impl YarpUnitShop {
    fn new(
        uid: String,
        name: String,
        model: String,
        unit_ids: Vec<String>,
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
    uid_registry: BTreeMap<String, String>,
    current_unit_list: Vec<String>,
    unit_shops: Vec<YarpUnitShop>,
    current_shop_x: i32,
    current_shop_y: i32,
}

impl<W: Write> CodeGenerator<W> {
    pub fn new(writer: W) -> CodeGenerator<W> {
        CodeGenerator {
            writer,
            units: Vec::new(),
            current_unit_list: Vec::new(),
            uid_registry: BTreeMap::new(),
            unit_shops: Vec::new(),
            current_shop_x: 0,
            current_shop_y: 0,
        }
    }

    pub fn process_record(&mut self, record: Record) {
        match record {
            Record::CustomUnit(unit) => {
                if self.current_unit_list.len() > 10 {
                    panic!("Trying to add more than 11 units to a shop.");
                }

                let yunit = YarpUnit::new(unit.uid, unit.name, unit.model);

                self.register_uid(&yunit.uid, &yunit.id_constant);

                self.current_unit_list.push(yunit.uid.clone());
                self.units.push(yunit);
            }

            Record::UnitShop(shop) => {
                let old_shop = replace(&mut self.current_unit_list, Vec::new());
                let unit_shop = YarpUnitShop::new(
                    shop.uid,
                    shop.name,
                    shop.model,
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
            _ => (),
        }
    }

    pub fn emit(&mut self) {
        let writer = &mut self.writer;
        let registry = &self.uid_registry;
        Self::emit_header(writer);

        self.units
            .iter()
            .for_each(|unit| Self::emit_unit_id(writer, &unit.id_constant));

        self.unit_shops
            .iter()
            .for_each(|shop| Self::emit_unit_id(writer, &shop.id_constant));

        Self::emit_newline(writer);
        Self::emit_definitions_start(writer);

        self.units
            .iter()
            .for_each(|unit| Self::emit_unit_definition(writer, unit));

        Self::emit_newline(writer);

        self.unit_shops
            .iter()
            .for_each(|shop| Self::emit_shop_definition(writer, registry, shop));

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

    fn emit_header(writer: &mut W) {
        writeln!(writer, "package AutoGenerated\nimport CodegenUtils\n").unwrap();
    }

    fn emit_newline(writer: &mut W) {
        writeln!(writer).unwrap();
    }

    fn emit_unit_id(writer: &mut W, id: &str) {
        writeln!(
            writer,
            "public constant {} = compiletime(UNIT_ID_GEN.next())",
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
                .map(|s| registry.get(s))
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
        writeln!(
            writer,
            "public function makeUidRegistry() returns HashMap<int, int>"
        ).unwrap();
        writeln!(writer, "\tlet registry = new HashMap<int, int>").unwrap();

        for (uid, id_constant) in registry {
            writeln!(
                writer,
                "\tregistry.put(compiletime(\"{}\".getHash()), {})",
                uid, id_constant
            ).unwrap();
        }

        writeln!(writer, "\treturn registry").unwrap();
    }

    fn emit_reverse_registry(writer: &mut W, registry: &BTreeMap<String, String>) {
        writeln!(
            writer,
            "public function makeUidReverseRegistry() returns HashMap<int, int>"
        ).unwrap();
        writeln!(writer, "\tlet registry = new HashMap<int, int>").unwrap();

        for (uid, id_constant) in registry {
            writeln!(
                writer,
                "\tregistry.put({}, compiletime(\"{}\".getHash()))",
                id_constant, uid
            ).unwrap();
        }

        writeln!(writer, "\treturn registry").unwrap();
    }

    fn emit_shop_registry(writer: &mut W, unit_shops: &Vec<YarpUnitShop>) {
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
}
