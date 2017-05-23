extern crate toolbox_data;
#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate lazy_static;

mod in_memory;

#[allow(dead_code, unused_imports)]
use toolbox_data::*;


type Id = in_memory::Id;
type Dm<'a, T> = in_memory::DataMapper<'a, T>;

#[derive(Default, Serialize, Deserialize)]
struct TestEntity
{
    id: Id,
    name: String
}

#[derive(Default, Serialize, Deserialize)]
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
    Dm::at(&mut entity).create();
    //Dm::create(&mut entity);
    assert_eq!(1, entity.id());

    let mut entity = OtherEntity {
        name: "other test entity".to_owned(),
        ..
        OtherEntity::default()
    };
    Dm::at(&mut entity).create();
    //Dm::create(&mut entity);
    assert_eq!(2, entity.id());
}

/*
#[test]
fn find_one()
{
    let mut entity = TestEntity {
        id: 0,
        name: "test entity 1".to_owned()
    };
    Dm::at(&mut entity).create();
    assert_eq!(1, entity.id());

    let mut entity = TestEntity {
        id: 0,
        name: "test entity 2".to_owned()
    };
    Dm::at(&mut entity).create();
    assert_eq!(2, entity.id());

    let entity = Dm::<TestEntity>::find().by(/*id = 1*/).one();
    assert_eq!(1, entity.id());
    assert_eq!("test entity 1".to_owned(), entity.name);

    let mut entity = OtherEntity {
        name: "other test entity".to_owned(),
        ..
        OtherEntity::default()
    };
    Dm::at(&mut entity).create();
    assert_eq!(3, entity.id());

    let entity = Dm::<OtherEntity>::find().by(/*id = 3*/).one();
    assert_eq!(3, entity.id());
    assert_eq!("other test entity".to_owned(), entity.name);
}
*/