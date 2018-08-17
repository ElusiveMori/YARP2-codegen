use csv::StringRecord;

#[derive(Debug, Eq, PartialEq)]
pub struct CustomUnitRecord {
    pub uid: String,
    pub name: String,
    pub model: String,
}

impl CustomUnitRecord {
    fn from_record(string_record: &StringRecord) -> Option<CustomUnitRecord> {
        Some(CustomUnitRecord {
            uid: string_record.get(1).unwrap().to_string(),
            name: string_record.get(2).unwrap().to_string(),
            model: string_record.get(3).unwrap().to_string(),
        })
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct StockUnitRecord {}

impl StockUnitRecord {
    fn from_record(string_record: &StringRecord) -> Option<StockUnitRecord> {
        Some(StockUnitRecord {})
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct CustomStructureRecord {}

impl CustomStructureRecord {
    fn from_record(string_record: &StringRecord) -> Option<CustomStructureRecord> {
        Some(CustomStructureRecord {})
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct UnitShopRecord {
    pub uid: String,
    pub name: String,
    pub model: String,
}

impl UnitShopRecord {
    fn from_record(string_record: &StringRecord) -> Option<UnitShopRecord> {
        Some(UnitShopRecord {
            uid: string_record.get(1).unwrap().to_string(),
            name: string_record.get(2).unwrap().to_string(),
            model: string_record.get(3).unwrap().to_string(),
        })
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct StructureBuilderRecord {}

impl StructureBuilderRecord {
    fn from_record(string_record: &StringRecord) -> Option<StructureBuilderRecord> {
        Some(StructureBuilderRecord {})
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct StructureShopRecord {}

impl StructureShopRecord {
    fn from_record(string_record: &StringRecord) -> Option<StructureShopRecord> {
        Some(StructureShopRecord {})
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct AttachmentRecord {}

impl AttachmentRecord {
    fn from_record(string_record: &StringRecord) -> Option<AttachmentRecord> {
        Some(AttachmentRecord {})
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct AttachmentShopRecord {}

impl AttachmentShopRecord {
    fn from_record(string_record: &StringRecord) -> Option<AttachmentShopRecord> {
        Some(AttachmentShopRecord {})
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum Record {
    CustomUnit(CustomUnitRecord),
    StockUnit(StockUnitRecord),
    CustomStructure(CustomStructureRecord),
    UnitShop(UnitShopRecord),
    StructureBuilder(StructureBuilderRecord),
    StructureShop(StructureShopRecord),
    Attachment(AttachmentRecord),
    AttachmentShop(AttachmentShopRecord),
}

impl Record {
    pub fn from_record(string_record: &StringRecord) -> Option<Record> {
        let record_type = string_record.get(0);

        if let Some(record_type) = record_type {
            return match record_type {
                "CustomUnit" => {
                    CustomUnitRecord::from_record(string_record).map(Record::CustomUnit)
                }
                "StockUnit" => StockUnitRecord::from_record(string_record).map(Record::StockUnit),
                "CustomStructure" => {
                    CustomStructureRecord::from_record(string_record).map(Record::CustomStructure)
                }
                "UnitShop" => UnitShopRecord::from_record(string_record).map(Record::UnitShop),
                "StructureBuilder" => {
                    StructureBuilderRecord::from_record(string_record).map(Record::StructureBuilder)
                }
                "StructureShop" => {
                    StructureShopRecord::from_record(string_record).map(Record::StructureShop)
                }
                "Attachment" => {
                    AttachmentRecord::from_record(string_record).map(Record::Attachment)
                }
                "AttachmentShop" => {
                    AttachmentShopRecord::from_record(string_record).map(Record::AttachmentShop)
                }
                _ => None,
            };
        }

        None
    }
}
