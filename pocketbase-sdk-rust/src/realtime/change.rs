use ::serde::de::DeserializeOwned;
use ::serde::Deserialize;
use anyhow::Result;

pub trait Record {
    fn key(&self) -> &str;
}

pub trait OrderedRecord {
    fn next_record_key(&self) -> &str;
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Action {
    Update,
    Create,
    Delete,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Change {
    pub record: Box<serde_json::value::RawValue>,
    pub action: Action,
}

impl Change {
    pub fn record<T: DeserializeOwned>(&self) -> Result<T> {
        let res = serde_json::from_str::<T>(&self.record.as_ref().get())?;
        Ok(res)
    }

    pub fn apply<T>(&self, collection: &mut Vec<T>)
    where
        T: Record + DeserializeOwned
    {
        let record = self.record::<T>().unwrap();
        let record_key = record.key();

        match self.action {
            Action::Update => {
                if let Some(index) = collection.iter().position(|i| i.key() == record_key) {
                    collection[index] = record;
                } else {
                    collection.push(record);
                }
            }
            Action::Create => collection.push(record),
            Action::Delete => collection.retain(|i| i.key() != record_key),
        }
    }
}
