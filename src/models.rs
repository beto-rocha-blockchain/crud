use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Clone, Debug)]

pub struct DataEntry {
    pub data1: Vec<String>,
    pub data2: Vec<u8>,
}