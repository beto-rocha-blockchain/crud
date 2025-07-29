use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Clone, Debug)]

pub struct DataEntry {
    pub func_names: Vec<String>,
    pub bytecode: Vec<u8>,
    pub owner: String,
}