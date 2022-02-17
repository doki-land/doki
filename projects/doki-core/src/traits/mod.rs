use serde::{Deserialize, Serialize};
use doki_error::Result;

pub trait ComponentData<'a>: where Self: Serialize + Deserialize<'a> {
    fn dump<T>(&self) -> Result<Vec<u8>> {
        Ok(bincode::serialize(self)?)
    }
    fn load<T>(data: &[u8]) -> Result<Self> {
        Ok(bincode::deserialize(data)?)
    }
    fn dump_debug(&self) -> Result<String> {
        Ok(serde_json::to_string_pretty(self)?)
    }
    fn load_debug(data: &str) -> Result<Self> {
        Ok(serde_json::from_str(data)?)
    }
}
