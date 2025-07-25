use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use crate::models::DataEntry;

pub type AppState = Arc<Mutex<HashMap<u32, DataEntry>>>;

pub fn new_state() -> AppState {
    Arc::new(Mutex::new(HashMap::new()))
}