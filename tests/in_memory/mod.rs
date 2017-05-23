extern crate toolbox_data;
extern crate serde;
extern crate serde_json;

#[allow(dead_code, unused_imports)]
use self::toolbox_data as api;
use self::serde::ser::Serialize;

use std::collections::HashMap;
use std::sync::Mutex;
use std::sync::MutexGuard;

pub type Id = u64;

pub struct Query<T: api::Identifiable<ID = Id> + Default>(T);

#[allow(unused_variables)]
impl<T: api::Identifiable<ID = Id> + Default> api::Query for Query<T>
{
    type Entity = T;

    fn from<C: api::Connection>(db: C) -> Self
    {
        unimplemented!()
    }

    fn one(&mut self) -> Self::Entity
    {
        //let Query::<T>(ref mut old) = *self;
        //old.set_id(0u64);
        let new = Self::Entity::default();
        println!("one: {}", new.id());
        new
    }

    fn all(&self) -> Vec<Self::Entity>
    {
        unimplemented!()
    }

    fn by(&mut self) -> &Self
    {
        {
            let Query::<T>(ref mut old) = *self;
            old.set_id(2 as Id);
            println!("by: {}", old.id());
        }
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

pub struct InMemoryStorage;

impl InMemoryStorage
{
    fn store<'a>() -> MutexGuard<'a, HashMap<Id, String>>
    {
        lazy_static! {
            static ref STORE: Mutex<HashMap<Id, String>> = { Mutex::new(HashMap::new()) };
        }
        STORE.lock().unwrap()
    }
}

pub struct DataMapper<'a, T: 'a + api::Identifiable>(pub &'a mut T);

impl<'a, T: api::Identifiable<ID = Id> + Default + Serialize> api::DataMapper<'a, Id, Query<T>> for DataMapper<'a, T>
{
    type Entity = T;

    fn find() -> Query<T>
    {
        println!("find.");
        Query(T::default())
    }

    fn at(entity: &'a mut Self::Entity) -> Self
    {
        DataMapper::<'a>(entity)
    }

    fn create(&mut self/*entity: &mut Self::Entity*/) -> bool
    {
        let DataMapper(ref mut entity) = *self;
        let id = (InMemoryStorage::store().len() + 1) as Id;
        entity.set_id(id);
        InMemoryStorage::store().insert(entity.id(), serde_json::to_string(entity).unwrap());
        true
    }

    fn update(&self) -> u32
    {
        unimplemented!()
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