// WZ data types - ported from provider/wz/MapleDataType.java

use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DataType {
    None,
    Img0x00,
    Short,
    Int,
    Float,
    Double,
    String,
    Extended,
    Property,
    Canvas,
    Vector,
    Convex,
    Sound,
    Uol,
    UnknownType,
    UnknownExtendedType,
}

impl From<u8> for DataType {
    fn from(byte: u8) -> Self {
        match byte {
            0 => DataType::None,
            1 => DataType::Img0x00,
            2 => DataType::Short,
            3 => DataType::Int,
            4 => DataType::Float,
            5 => DataType::Double,
            6 => DataType::String,
            7 => DataType::Extended,
            8 => DataType::Property,
            9 => DataType::Canvas,
            10 => DataType::Vector,
            11 => DataType::Convex,
            12 => DataType::Sound,
            13 => DataType::Uol,
            20 => DataType::UnknownType,
            21 => DataType::UnknownExtendedType,
            _ => DataType::None,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum DataValue {
    None,
    Short(i16),
    Int(i32),
    Float(f32),
    Double(f64),
    Str(String),
    Vector(i32, i32),
    Canvas {
        width: i32,
        height: i32,
        data: Vec<u8>,
    },
    Sound(Vec<u8>),
}

#[derive(Debug, Clone)]
pub struct DataEntry {
    pub name: String,
    pub dtype: DataType,
    pub children: Vec<DataEntry>,
    pub value: DataValue,
}

impl DataEntry {
    pub fn new(name: &str, dtype: DataType) -> Self {
        DataEntry {
            name: name.to_string(),
            dtype,
            children: Vec::new(),
            value: DataValue::None,
        }
    }

    pub fn child_by_path(&self, path: &str) -> Option<&DataEntry> {
        let parts: Vec<&str> = path.split('/').collect();
        let mut current = self;
        for part in parts {
            current = current.children.iter().find(|c| c.name == part)?;
        }
        Some(current)
    }

    pub fn child_by_name(&self, name: &str) -> Option<&DataEntry> {
        self.children.iter().find(|c| c.name == name)
    }
}

pub type DataMap = HashMap<String, DataEntry>;

#[derive(Debug, Clone)]
pub struct WzFileInfo {
    pub name: String,
    pub size: usize,
    pub checksum: u32,
    pub offset: u32,
}

pub trait DataProvider: Send + Sync {
    fn get_data(&self, path: &str) -> Option<&DataEntry>;
    fn get_root(&self) -> &DataEntry;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_data_type_from_u8() {
        assert_eq!(DataType::from(2), DataType::Short);
        assert_eq!(DataType::from(3), DataType::Int);
        assert_eq!(DataType::from(6), DataType::String);
        assert_eq!(DataType::from(13), DataType::Uol);
        assert_eq!(DataType::from(255), DataType::None);
    }

    #[test]
    fn test_child_by_path() {
        let mut root = DataEntry::new("root", DataType::Property);
        let mut child = DataEntry::new("Item", DataType::Property);
        let leaf = DataEntry::new("00020000", DataType::String);
        child.children.push(leaf);
        root.children.push(child);

        assert!(root.child_by_path("Item/00020000").is_some());
        assert!(root.child_by_path("Item/nonexistent").is_none());
    }

    #[test]
    fn test_data_values() {
        assert_eq!(DataValue::Int(42), DataValue::Int(42));
        assert_eq!(DataValue::Str("test".into()), DataValue::Str("test".into()));
        assert_ne!(DataValue::Int(1), DataValue::Int(2));
    }
}
