use borsh::{BorshDeserialize, BorshSerialize};
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use wincode::{SchemaRead, SchemaWrite};
use wincode::config::Configuration;
use std::error::Error;
use std::fmt;
use std::marker::PhantomData;

pub trait Serializer<T> {
    type Error: Error;

    fn from_bytes(&self, data: &[u8]) -> Result<T, Self::Error>;
    fn to_bytes(&self, data: &T) -> Result<Vec<u8>, Self::Error>;
}

pub struct Json;
pub struct Borsh;
pub struct Wincode;

impl<T> Serializer<T> for Json
where
    T: Serialize + DeserializeOwned,
{
    type Error = serde_json::Error;

    fn from_bytes(&self, data: &[u8]) -> Result<T, Self::Error> {
        serde_json::from_slice(data)
    }

    fn to_bytes(&self, data: &T) -> Result<Vec<u8>, Self::Error> {
        serde_json::to_vec(data)
    }
}

impl<T> Serializer<T> for Borsh
where
    T: BorshSerialize + BorshDeserialize,
{
    type Error = borsh::io::Error;

    fn from_bytes(&self, data: &[u8]) -> Result<T, Self::Error> {
        borsh::from_slice(data)
    }

    fn to_bytes(&self, data: &T) -> Result<Vec<u8>, Self::Error> {
        borsh::to_vec(data)
    }
}

#[derive(Debug)]
pub enum WincodeError {
    Read(wincode::ReadError),
    Write(wincode::WriteError),
}

impl fmt::Display for WincodeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            WincodeError::Read(e) => write!(f, "Wincode read error: {}", e),
            WincodeError::Write(e) => write!(f, "Wincode write error: {}", e),
        }
    }
}

impl Error for WincodeError {}

impl From<wincode::ReadError> for WincodeError {
    fn from(value: wincode::ReadError) -> Self {
        WincodeError::Read(value)
    }
}

impl From<wincode::WriteError> for WincodeError {
    fn from(value: wincode::WriteError) -> Self {
        WincodeError::Write(value)
    }
}

impl<T> Serializer<T> for Wincode
where
    T: SchemaWrite<Configuration, Src = T> + for<'a> SchemaRead<'a, Configuration, Dst = T>,
{
    type Error = WincodeError;

    fn from_bytes(&self, data: &[u8]) -> Result<T, Self::Error> {
        Ok(wincode::deserialize(data)?)
    }

    fn to_bytes(&self, data: &T) -> Result<Vec<u8>, Self::Error> {
        Ok(wincode::serialize(data)?)
    }
}

pub struct Storage<T, S>
where
    S: Serializer<T>,
{
    serializer: S,
    data: Option<Vec<u8>>,
    _marker: PhantomData<T>,
}

impl<T, S> Storage<T, S>
where
    S: Serializer<T>,
{
    pub fn new(serializer: S) -> Self {
        Self {
            serializer,
            data: None,
            _marker: PhantomData,
        }
    }

    pub fn save(&mut self, value: &T) -> Result<(), S::Error> {
        let bytes = self.serializer.to_bytes(value)?;
        self.data = Some(bytes);
        Ok(())
    }

    pub fn load(&self) -> Result<T, S::Error> {
        let bytes = self.data.as_ref().expect("No data stored in Storage");
        self.serializer.from_bytes(bytes)
    }

    pub fn has_data(&self) -> bool {
        self.data.is_some()
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize, BorshSerialize, BorshDeserialize, SchemaRead, SchemaWrite)]
pub struct Person {
    pub name: String,
    pub age: u32,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_person() -> Person {
        Person {
            name: "Andre".to_string(),
            age: 30,
        }
    }

    #[test]
    fn json_save_and_load() {
        let person = sample_person();
        let mut storage = Storage::<Person, Json>::new(Json);

        storage.save(&person).unwrap();
        assert!(storage.has_data());

        let loaded = storage.load().unwrap();
        assert_eq!(person, loaded);
    }

    #[test]
    fn borsh_save_and_load() {
        let person = sample_person();
        let mut storage = Storage::<Person, Borsh>::new(Borsh);

        storage.save(&person).unwrap();
        assert!(storage.has_data());

        let loaded = storage.load().unwrap();
        assert_eq!(person, loaded);
    }

    #[test]
    fn wincode_save_and_load() {
        let person = sample_person();
        let mut storage = Storage::<Person, Wincode>::new(Wincode);

        storage.save(&person).unwrap();
        assert!(storage.has_data());

        let loaded = storage.load().unwrap();
        assert_eq!(person, loaded);
    }
}
