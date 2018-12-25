use csv::StringRecord;

#[derive(Debug, Eq, PartialEq)]
pub struct CustomUnitRecord {
    pub uid: String,
    pub name: String,
    pub model: String,
}

impl CustomUnitRecord {
    fn from_record(string_record: &StringRecord) -> Option<CustomUnitRecord> {
        let uid = string_record.get(1)?.to_string();

        Some(CustomUnitRecord {
            uid: string_record.get(1).unwrap().to_string(),
            name: string_record.get(2).unwrap().to_string(),
            model: string_record.get(3).unwrap().to_string(),
        })
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct StockUnitRecord {
    pub id: String,
    pub model: String
}

impl StockUnitRecord {
    fn from_record(string_record: &StringRecord) -> Option<StockUnitRecord> {
        Some(StockUnitRecord {
            id: string_record.get(1).unwrap().to_string(),
            model: string_record.get(2).unwrap().to_string()
        })
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct CustomBuildingRecord {
    pub uid: String,
    pub name: String,
    pub model: String,
}

impl CustomBuildingRecord {
    fn from_record(string_record: &StringRecord) -> Option<CustomBuildingRecord> {
        Some(CustomBuildingRecord {
            uid: string_record.get(1).unwrap().to_string(),
            name: string_record.get(2).unwrap().to_string(),
            model: string_record.get(3).unwrap().to_string(),
        })
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct UnitShopRecord {
    pub uid: String,
    pub name: String,
    pub model: Option<String>,
}

impl UnitShopRecord {
    fn from_record(string_record: &StringRecord) -> Option<UnitShopRecord> {
        Some(UnitShopRecord {
            uid: string_record.get(1).unwrap().to_string(),
            name: string_record.get(2).unwrap().to_string(),
            model: string_record.get(3).map(|f| f.to_string()),
        })
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct CustomBuilderRecord {
    pub uid: String,
    pub name: String,
    pub model: Option<String>,
}

impl CustomBuilderRecord {
    fn from_record(string_record: &StringRecord) -> Option<CustomBuilderRecord> {
        Some(CustomBuilderRecord {
            uid: string_record.get(1).unwrap().to_string(),
            name: string_record.get(2).unwrap().to_string(),
            model: string_record.get(3).map(|f| f.to_string()),
        })
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct BuilderShopRecord {
    pub uid: String,
    pub name: String,
}

impl BuilderShopRecord {
    fn from_record(string_record: &StringRecord) -> Option<BuilderShopRecord> {
        Some(BuilderShopRecord {
            uid: string_record.get(1).unwrap().to_string(),
            name: string_record.get(2).unwrap().to_string(),
        })
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct SetShopModelRecord {
    pub model: String,
}

impl SetShopModelRecord {
    fn from_record(string_record: &StringRecord) -> Option<SetShopModelRecord> {
        Some(SetShopModelRecord {
            model: string_record.get(1).unwrap().to_string()
        })
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct StockUnitModelRecord {
    pub id: String,
    pub model: String,
}

impl StockUnitModelRecord {
    fn from_record(string_record: &StringRecord) -> Option<StockUnitModelRecord> {
        Some(StockUnitModelRecord {
            id: string_record.get(1).unwrap().to_string(),
            model: string_record.get(2).unwrap().to_string()
        })
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct ShopNewLineRecord {
}

impl ShopNewLineRecord {
    fn from_record(string_record: &StringRecord) -> Option<ShopNewLineRecord> {
        Some(ShopNewLineRecord {
        })
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
    CustomBuilding(CustomBuildingRecord),
    UnitShop(UnitShopRecord),
    CustomBuilder(CustomBuilderRecord),
    BuilderShop(BuilderShopRecord),
    SetShopModel(SetShopModelRecord),
    Attachment(AttachmentRecord),
    AttachmentShop(AttachmentShopRecord),
    StockUnitModel(StockUnitModelRecord),
    ShopNewLine(ShopNewLineRecord)
}

impl Record {
    pub fn from_record(string_record: &StringRecord) -> Option<Record> {
        let record_type = string_record.get(0);

        if let Some(record_type) = record_type {
            return match record_type {
                "CustomUnit" => CustomUnitRecord::from_record(string_record).map(Record::CustomUnit),
                "StockUnit" => StockUnitRecord::from_record(string_record).map(Record::StockUnit),
                "CustomBuilding" => CustomBuildingRecord::from_record(string_record).map(Record::CustomBuilding),
                "UnitShop" => UnitShopRecord::from_record(string_record).map(Record::UnitShop),
                "CustomBuilder" => CustomBuilderRecord::from_record(string_record).map(Record::CustomBuilder),
                "BuilderShop" => BuilderShopRecord::from_record(string_record).map(Record::BuilderShop),
                "Attachment" => AttachmentRecord::from_record(string_record).map(Record::Attachment),
                "AttachmentShop" => AttachmentShopRecord::from_record(string_record).map(Record::AttachmentShop),
                "SetShopModel" => SetShopModelRecord::from_record(string_record).map(Record::SetShopModel),
                "StockUnitModel" => StockUnitModelRecord::from_record(string_record).map(Record::StockUnitModel),
                "ShopNewLine" => ShopNewLineRecord::from_record(string_record).map(Record::ShopNewLine),
                _ => None,
            };
        }

        None
    }
}
