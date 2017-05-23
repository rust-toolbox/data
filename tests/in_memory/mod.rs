extern crate toolbox_data;
extern crate serde;
extern crate serde_json;

#[allow(dead_code, unused_imports)]
use self::toolbox_data as api;
use self::serde::ser::Serialize;

use std::collections::HashMap;
use std::sync::Mutex;
use std::sync::MutexGuard;
use std::ops::Deref;
use std::ops::DerefMut;


pub type Id = u64;

type RawStore = HashMap<Id, String>;

struct InMemoryStorage<'a>(MutexGuard<'a, RawStore>);

impl<'a> Deref for InMemoryStorage<'a> {
    type Target = RawStore;

    fn deref(&self) -> &RawStore {
        &self.0
    }
}

impl<'a> DerefMut for InMemoryStorage<'a> {
    fn deref_mut(&mut self) -> &mut RawStore {
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

pub struct DataMapper<'a, T: 'a + api::Identifiable>(pub &'a mut T);

impl<'a, T: api::Identifiable<ID = Id> + Default + Serialize + Clone> api::DataMapper<'a, Id, Query<T>> for DataMapper<'a, T>
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

    fn create(entity: &mut Self::Entity) -> bool
    {
        let mut store = InMemoryStorage::hold();
        let id = store.next_id();
        entity.set_id(id);
        let json = serde_json::to_string(entity).unwrap();
//        println!("create {}", json);
        store.insert(entity.id(), json);
        true
    }

    fn insert(entity: &Self::Entity) -> Id
    {
        let mut store = InMemoryStorage::hold();
        let id = store.next_id();
        let ref mut persist = entity.clone();
        persist.set_id(id);
        let json = serde_json::to_string(persist).unwrap();
//        println!("insert {}", json);
        store.insert(id, json);
        id
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