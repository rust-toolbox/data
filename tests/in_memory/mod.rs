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
        unsafe {
            static mut SEQUENCE_VALUE: Id = 0 as Id;
            SEQUENCE_VALUE += 1;
            SEQUENCE_VALUE
        }
    }

    fn persist<T>(&mut self, entity: &mut T) -> Id
        where T: api::Identifiable<ID = Id> + Serialize + DeserializeOwned
    {
        let id = self.next_id();
        entity.set_id(id);
        let json = serde_json::to_string(entity).unwrap();
        self.insert(id, json);
        id
    }

    fn renew<T>(&mut self, entity: &T) -> bool
        where T: api::Identifiable<ID = Id> + Serialize + DeserializeOwned
    {
        if self.contains_key(&entity.id()) {
            let json = serde_json::to_string(entity).unwrap();
            self.insert(entity.id(), json);
            true
        } else {
            false
        }
    }
}

#[allow(dead_code)]
pub enum QueryAction
{
    None,
    Find,
    Insert,
    Update,
    Delete
}

#[allow(dead_code)]
pub struct Query<T>
{
    action: QueryAction,
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
    type Action = QueryAction;

    fn query(action: Self::Action) -> Query<T>
    {
        Query { action: action, criteria: json!({ "by": null }), phantom: PhantomData }
    }

    fn find() -> Query<T>
    {
        Query { action: QueryAction::Find, criteria: json!({ "by": null }), phantom: PhantomData }
    }

    fn create(entity: &'a mut Self::Entity) -> bool
    {
        let mut store = InMemoryStorage::hold();
        store.persist(entity);
        true
    }

    fn insert(entity: &'a Self::Entity) -> Id
    {
        let mut store = InMemoryStorage::hold();
        let ref mut persistent = entity.clone();
        store.persist(persistent)
    }

    fn update(entity: &'a Self::Entity) -> bool
    {
        let mut store = InMemoryStorage::hold();
        store.renew(entity)
    }

    fn save(entity: &'a mut Self::Entity) -> bool
    {
        let mut store = InMemoryStorage::hold();
        if store.contains_key(&entity.id()) {
            store.renew(entity)
        } else {
            store.persist(entity);
            true
        }
    }

    fn insert_or_update(entity: &'a Self::Entity) -> Id
    {
        let mut store = InMemoryStorage::hold();
        if store.contains_key(&entity.id()) {
            store.renew(entity);
            entity.id()
        } else {
            let ref mut persistent = entity.clone();
            store.persist(persistent)
        }
    }

    #[allow(unused_variables)]
    fn delete(entity: &'a Self::Entity) -> bool
    {
        let mut store = InMemoryStorage::hold();
        let result = store.remove(&entity.id());
        if let Some(_) = result {
            true
        } else {
            false
        }
    }
}