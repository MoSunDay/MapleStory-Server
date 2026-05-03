// WZ binary reader stub - ported from provider/wz/WZFile.java + WZIMGFile.java

use crate::data_types::{DataEntry, DataType, DataValue};
use std::collections::HashMap;
use std::path::Path;

pub struct WzBinaryProvider {
    root: DataEntry,
    entries: HashMap<String, DataEntry>,
}

impl WzBinaryProvider {
    pub fn load<P: AsRef<Path>>(_path: P) -> Result<Self, String> {
        // TODO: Implement WZ binary format parsing
        // The WZ format uses:
        // 1. GMS encryption (key XOR)
        // 2. Zlib/deflate compression
        // 3. WZ-specific IMG sub-format
        let root = DataEntry::new("root", DataType::Property);
        Ok(WzBinaryProvider {
            root,
            entries: HashMap::new(),
        })
    }
}

impl super::DataProvider for WzBinaryProvider {
    fn get_data(&self, path: &str) -> Option<&DataEntry> {
        self.entries.get(path)
    }

    fn get_root(&self) -> &DataEntry {
        &self.root
    }
}

pub struct XmlWzProvider {
    root: DataEntry,
    entries: HashMap<String, DataEntry>,
}

impl XmlWzProvider {
    pub fn load<P: AsRef<Path>>(_path: P) -> Result<Self, String> {
        // TODO: Implement WZ XML format parsing
        // XML format uses the standard MapleStory XML structure:
        // <imgdir name="...">   -> Property + children
        // <int name="..." value="..."/> -> Int
        // <string name="..." value="..."/> -> String
        // etc.
        let root = DataEntry::new("root", DataType::Property);
        Ok(XmlWzProvider {
            root,
            entries: HashMap::new(),
        })
    }
}

impl super::DataProvider for XmlWzProvider {
    fn get_data(&self, path: &str) -> Option<&DataEntry> {
        self.entries.get(path)
    }

    fn get_root(&self) -> &DataEntry {
        &self.root
    }
}

// Factory: auto-detect WZ vs XML format
pub fn open_wz<P: AsRef<Path>>(path: P) -> Result<Box<dyn super::DataProvider>, String> {
    let path_ref = path.as_ref();
    let name = path_ref.file_name().and_then(|n| n.to_str()).unwrap_or("");

    if name.to_lowercase().ends_with(".wz") && path_ref.is_file() {
        Ok(Box::new(WzBinaryProvider::load(path)?))
    } else {
        Ok(Box::new(XmlWzProvider::load(path)?))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data_types::DataProvider;

    #[test]
    fn test_wz_provider_trait() {
        let provider = WzBinaryProvider::load("test.wz").unwrap();
        assert_eq!(provider.get_root().name, "root");
        assert_eq!(provider.get_root().dtype, DataType::Property);
    }
}
