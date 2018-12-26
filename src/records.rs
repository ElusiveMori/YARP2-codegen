use csv::StringRecord;

#[derive(Debug, Eq, PartialEq)]
pub struct CustomUnitRecord<'src> {
    pub uid: &'src str,
    pub name: &'src str,
    pub model: &'src str,
}

impl<'src> CustomUnitRecord<'src> {
    fn from_record(string_record: &StringRecord) -> Option<CustomUnitRecord> {
        Some(CustomUnitRecord {
            uid: string_record.get(1).unwrap(),
            name: string_record.get(2).unwrap(),
            model: string_record.get(3).unwrap(),
        })
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct StockUnitRecord<'src> {
    pub id: &'src str,
    pub model: &'src str,
}

impl<'src> StockUnitRecord<'src> {
    fn from_record(string_record: &StringRecord) -> Option<StockUnitRecord> {
        Some(StockUnitRecord {
            id: string_record.get(1).unwrap(),
            model: string_record.get(2).unwrap(),
        })
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct CustomBuildingRecord<'src> {
    pub uid: &'src str,
    pub name: &'src str,
    pub model: &'src str,
}

impl<'src> CustomBuildingRecord<'src> {
    fn from_record(string_record: &StringRecord) -> Option<CustomBuildingRecord> {
        Some(CustomBuildingRecord {
            uid: string_record.get(1).unwrap(),
            name: string_record.get(2).unwrap(),
            model: string_record.get(3).unwrap(),
        })
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct UnitShopRecord<'src> {
    pub uid: &'src str,
    pub name: &'src str,
    pub model: Option<&'src str>,
}

impl<'src> UnitShopRecord<'src> {
    fn from_record(string_record: &StringRecord) -> Option<UnitShopRecord> {
        Some(UnitShopRecord {
            uid: string_record.get(1).unwrap(),
            name: string_record.get(2).unwrap(),
            model: string_record.get(3).map(|f| f),
        })
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct CustomBuilderRecord<'src> {
    pub uid: &'src str,
    pub name: &'src str,
    pub model: Option<&'src str>,
}

impl<'src> CustomBuilderRecord<'src> {
    fn from_record(string_record: &StringRecord) -> Option<CustomBuilderRecord> {
        Some(CustomBuilderRecord {
            uid: string_record.get(1).unwrap(),
            name: string_record.get(2).unwrap(),
            model: string_record.get(3).map(|f| f),
        })
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct BuilderShopRecord<'src> {
    pub uid: &'src str,
    pub name: &'src str,
}

impl<'src> BuilderShopRecord<'src> {
    fn from_record(string_record: &StringRecord) -> Option<BuilderShopRecord> {
        Some(BuilderShopRecord {
            uid: string_record.get(1).unwrap(),
            name: string_record.get(2).unwrap(),
        })
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct SetShopModelRecord<'src> {
    pub model: &'src str,
}

impl<'src> SetShopModelRecord<'src> {
    fn from_record(string_record: &StringRecord) -> Option<SetShopModelRecord> {
        Some(SetShopModelRecord {
            model: string_record.get(1).unwrap(),
        })
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct StockUnitModelRecord<'src> {
    pub id: &'src str,
    pub model: &'src str,
}

impl<'src> StockUnitModelRecord<'src> {
    fn from_record(string_record: &StringRecord) -> Option<StockUnitModelRecord> {
        Some(StockUnitModelRecord {
            id: string_record.get(1).unwrap(),
            model: string_record.get(2).unwrap(),
        })
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct SetDefaultIconRecord<'src> {
    pub icon: &'src str,
}

impl<'src> SetDefaultIconRecord<'src> {
    fn from_record(string_record: &StringRecord) -> Option<SetDefaultIconRecord> {
        Some(SetDefaultIconRecord {
            icon: string_record.get(1).unwrap(),
        })
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum Record<'src> {
    CustomUnit(CustomUnitRecord<'src>),
    StockUnit(StockUnitRecord<'src>),
    CustomBuilding(CustomBuildingRecord<'src>),
    UnitShop(UnitShopRecord<'src>),
    CustomBuilder(CustomBuilderRecord<'src>),
    BuilderShop(BuilderShopRecord<'src>),
    SetShopModel(SetShopModelRecord<'src>),
    StockUnitModel(StockUnitModelRecord<'src>),
    SetDefaultIcon(SetDefaultIconRecord<'src>),
}

impl<'src> Record<'src> {
    pub fn from_record(string_record: &StringRecord) -> Option<Record> {
        let record_type = string_record.get(0);

        if let Some(record_type) = record_type {
            return match record_type {
                "CustomUnit" => {
                    CustomUnitRecord::from_record(string_record).map(Record::CustomUnit)
                }
                "StockUnit" => StockUnitRecord::from_record(string_record).map(Record::StockUnit),
                "CustomBuilding" => {
                    CustomBuildingRecord::from_record(string_record).map(Record::CustomBuilding)
                }
                "UnitShop" => UnitShopRecord::from_record(string_record).map(Record::UnitShop),
                "CustomBuilder" => {
                    CustomBuilderRecord::from_record(string_record).map(Record::CustomBuilder)
                }
                "BuilderShop" => {
                    BuilderShopRecord::from_record(string_record).map(Record::BuilderShop)
                }
                "SetShopModel" => {
                    SetShopModelRecord::from_record(string_record).map(Record::SetShopModel)
                }
                "StockUnitModel" => {
                    StockUnitModelRecord::from_record(string_record).map(Record::StockUnitModel)
                }
                "SetDefaultIcon" => {
                    SetDefaultIconRecord::from_record(string_record).map(Record::SetDefaultIcon)
                }
                _ => None,
            };
        }

        None
    }
}
