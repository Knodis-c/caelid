use serde::{Serialize, Deserialize};
use serde_json::{self, Result as SerdeResult};

pub type JSONString = String;
pub type JSONStr = str;

pub trait JSON<'a>: Serialize + Deserialize<'a> {
    fn to_json(&self) -> SerdeResult<JSONString> {
       serde_json::to_string(self)
    }
}

