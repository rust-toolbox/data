extern crate toolbox_data;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate lazy_static;

mod in_memory;

#[allow(dead_code, unused_imports)]
use toolbox_data::*;


type Id = in_memory::Id;
type Dm<'a, T> = in_memory::DataMapper<'a, T>;

#[derive(Default, Clone, Serialize, Deserialize, Debug)]
struct TestEntity
{
    id: Id,
    name: String
}

#[derive(Default, Clone, Serialize, Deserialize, Debug)]
struct OtherEntity
{
    id: Id,
    name: String,
    val: i32
}

impl Identifiable for TestEntity
{
    type ID = Id;

    fn id(&self) -> Self::ID
    {
        self.id
    }

    fn set_id(&mut self, id: Self::ID) -> &mut Self
    {
        self.id = id;
        self
    }
}

impl Identifiable for OtherEntity
{
    type ID = Id;

    fn id(&self) -> Self::ID
    {
        self.id
    }

    fn set_id(&mut self, id: Self::ID) -> &mut Self
    {
        self.id = id;
        self
    }
}


#[test]
fn create()
{
    let mut entity = TestEntity {
        id: 0,
        name: "test entity".to_owned()
    };
    Dm::create(&mut entity);
    assert!(0 < entity.id());

    let id = entity.id();
    let mut entity = OtherEntity {
        name: "other test entity".to_owned(),
        ..
        OtherEntity::default()
    };
    Dm::create(&mut entity);
    assert!(id < entity.id());
}

#[test]
fn insert()
{
    let entity = TestEntity {
        id: 0,
        name: "test entity".to_owned()
    };
    let id = Dm::insert(&entity);
    assert!(0 < id);

    let entity = OtherEntity {
        name: "other test entity".to_owned(),
        ..
        OtherEntity::default()
    };
    let other_id = Dm::insert(&entity);
    assert!(id < other_id);
}

#[test]
fn find_one()
{
    let mut entity = TestEntity {
        id: 0,
        name: "test entity 1".to_owned()
    };
    Dm::create(&mut entity);
    let id = entity.id();

    let mut entity = TestEntity {
        id: 0,
        name: "test entity 2".to_owned()
    };
    Dm::create(&mut entity);

    let entity = Dm::<TestEntity>::find().by(json!({ "id": id })).one();
    assert_eq!(id, entity.id());
    assert_eq!("test entity 1".to_owned(), entity.name);

    let mut entity = OtherEntity {
        name: "other test entity".to_owned(),
        ..
        OtherEntity::default()
    };
    Dm::create(&mut entity);
    let other_id = entity.id();

    let entity = Dm::<OtherEntity>::find().by(json!({ "id": other_id })).one();
    assert_eq!(other_id, entity.id());
    assert_eq!("other test entity".to_owned(), entity.name);

    let name = "test entity 2".to_owned();
    let entity = Dm::<TestEntity>::find().by(json!({ "name": name })).one();
    assert_eq!(name, entity.name);
}