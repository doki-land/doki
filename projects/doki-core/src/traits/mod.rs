use doki_error::Result;
use serde::{Deserialize, Serialize};

pub trait ComponentData<'i>
where
    Self: Serialize + Deserialize<'i>,
{
    fn dump<T>(&self) -> Result<Vec<u8>> {
        Ok(bincode::serialize(self)?)
    }
    fn load<T>(data: &'i [u8]) -> Result<Self> {
        Ok(bincode::deserialize(data)?)
    }
    fn dump_debug(&self) -> Result<String> {
        Ok(serde_json::to_string_pretty(self)?)
    }
    fn load_debug(data: &'i str) -> Result<Self> {
        Ok(serde_json::from_str(data)?)
    }
}
