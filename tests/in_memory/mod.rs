extern crate toolbox_data;
extern crate serde;
extern crate serde_json;

#[allow(dead_code, unused_imports)]
use self::toolbox_data as api;
use self::serde::Serialize;
use self::serde::de::DeserializeOwned;
use self::serde_json::Value;

use std::collections::HashMap;
use std::sync::Mutex;
use std::sync::MutexGuard;
use std::ops::Deref;
use std::ops::DerefMut;
use std::marker::PhantomData;


pub type Id = u64;

type RawStore = HashMap<Id, String>;

struct InMemoryStorage<'a>(MutexGuard<'a, RawStore>);

impl<'a> Deref for InMemoryStorage<'a>
{
    type Target = RawStore;

    fn deref(&self) -> &RawStore
    {
        &self.0
    }
}

impl<'a> DerefMut for InMemoryStorage<'a>
{
    fn deref_mut(&mut self) -> &mut RawStore
    {
        &mut self.0
    }
}

impl<'a> InMemoryStorage<'a>
{
    fn hold() -> Self
    {
        lazy_static! {
            static ref STORE: Mutex<HashMap<Id, String>> = { Mutex::new(HashMap::new()) };
        }
        InMemoryStorage(STORE.lock().unwrap())
    }

    fn next_id(&self) -> Id
    {
        (self.len() + 1) as Id
    }
}

pub struct Query<T>
{
    criteria: Value,
    phantom: PhantomData<T>
}

#[allow(unused_variables)]
impl<'a, T: api::Identifiable<ID = Id> + Default + DeserializeOwned> api::Query for Query<T>
{
    type Entity = T;
    type Criteria = Value;

    fn from<C: api::Connection>(db: C) -> Self
    {
        unimplemented!()
    }

    fn one(self) -> Self::Entity
    {
        let store = InMemoryStorage::hold();
        if store.is_empty() {
            return Self::Entity::default();
        }

        match self.criteria["by"] {
            Value::Object(ref by) => {
                for (key, value) in store.iter() {
                    let object: Value = serde_json::from_str(value).unwrap();
                    let mut equals = true;
                    if let Value::Object(ref current) = object {
                        for (by_key, by_value) in by.iter() {
                            if let Some(current_value) = current.get(by_key) {
                                if current_value != by_value {
                                    equals = false;
                                    break;
                                }
                            } else {
                                equals = false;
                                break;
                            }
                        }
                    } else {
                        equals = false;
                    }
                    if equals {
                        return serde_json::from_value(object).unwrap();
                    }
                }
                Self::Entity::default()
            },
            _ => {
                for (key, ref value) in store.iter() {
                    if let Result::Ok(entity) = serde_json::from_str::<Self::Entity>(value) {
                        return entity;
                    }
                }
                Self::Entity::default()
            }
        }
    }

    fn all(self) -> Vec<Self::Entity>
    {
        let mut all: Vec<Self::Entity> = Vec::new();
        let store = InMemoryStorage::hold();
        if store.is_empty() {
            return all;
        }

        match self.criteria["by"] {
            Value::Object(ref by) => {
                for (key, value) in store.iter() {
                    let object: Value = serde_json::from_str(value).unwrap();
                    let mut equals = true;
                    if let Value::Object(ref current) = object {
                        for (by_key, by_value) in by.iter() {
                            if let Some(current_value) = current.get(by_key) {
                                if current_value != by_value {
                                    equals = false;
                                    break;
                                }
                            } else {
                                equals = false;
                                break;
                            }
                        }
                    } else {
                        equals = false;
                    }
                    if equals {
                        all.push(serde_json::from_value(object).unwrap());
                    }
                }
                all
            },
            _ => {
                for (key, ref value) in store.iter() {
                    if let Result::Ok(entity) = serde_json::from_str::<Self::Entity>(value) {
                        all.push(entity);
                    }
                }
                all
            }
        }
    }

    fn by(mut self, criteria: Self::Criteria) -> Self
    {
        self.criteria["by"] = criteria;
        self
    }

    fn limit(&self, limit: u32) -> Self
    {
        unimplemented!()
    }

    fn offset(&self, offset: u32) -> Self
    {
        unimplemented!()
    }

    fn count(&self) -> u32
    {
        unimplemented!()
    }

    fn exists(&self) -> bool
    {
        unimplemented!()
    }
}

pub struct DataMapper<'a, T: 'a + api::Identifiable>(pub &'a mut T);

impl<'a, T: api::Identifiable<ID = Id> + Default + Serialize + DeserializeOwned + Clone> api::DataMapper<'a, Id, Query<T>> for DataMapper<'a, T>
{
    type Entity = T;

    fn find() -> Query<T>
    {
        Query { criteria: json!({ "by": null }), phantom: PhantomData }
    }

    fn at(entity: &'a mut Self::Entity) -> Self
    {
        DataMapper::<'a>(entity)
    }

    fn create(entity: &mut Self::Entity) -> bool
    {
        let mut store = InMemoryStorage::hold();
        let id = store.next_id();
        entity.set_id(id);
        let json = serde_json::to_string(entity).unwrap();
        store.insert(entity.id(), json);
        true
    }

    fn insert(entity: &Self::Entity) -> Id
    {
        let mut store = InMemoryStorage::hold();
        let id = store.next_id();
        let ref mut persistent = entity.clone();
        persistent.set_id(id);
        let json = serde_json::to_string(persistent).unwrap();
        store.insert(id, json);
        id
    }

    fn update(entity: &Self::Entity) -> bool
    {
        let mut store = InMemoryStorage::hold();
        if store.contains_key(&entity.id()) {
            let json = serde_json::to_string(entity).unwrap();
            store.insert(entity.id(), json);
            true
        } else {
            false
        }
    }

    fn save(&self) -> bool
    {
        unimplemented!()
    }

    fn delete(&self) -> u32
    {
        unimplemented!()
    }
}