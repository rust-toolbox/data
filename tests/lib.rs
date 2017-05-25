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
    assert_eq!("test entity 1", entity.name);

    let entity = Dm::<TestEntity>::query(in_memory::QueryAction::Find)
        .by(json!({ "id": id }))
        .one();
    assert_eq!(id, entity.id());
    assert_eq!("test entity 1", entity.name);

    let mut entity = OtherEntity {
        name: "other test entity".to_owned(),
        ..
        OtherEntity::default()
    };
    Dm::create(&mut entity);
    let other_id = entity.id();

    let entity = Dm::<OtherEntity>::find().by(json!({ "id": other_id })).one();
    assert_eq!(other_id, entity.id());
    assert_eq!("other test entity", entity.name);

    let name = "test entity 2";
    let entity = Dm::<TestEntity>::find().by(json!({ "name": name })).one();
    assert_eq!(name, entity.name);

    let entity = OtherEntity {
        name: "other test entity".to_owned(),
        ..
        OtherEntity::default()
    };
    Dm::insert(&entity);

    let name = "other test entity";
    let entity = Dm::<OtherEntity>::find().by(json!({ "name": name })).one();
    assert_eq!(name, entity.name);
}

#[test]
fn find_all()
{
    let mut ids: Vec<Id> = Vec::new();
    let entity = TestEntity {
        id: 0,
        name: "test all entity".to_owned()
    };
    ids.push(Dm::insert(&entity));

    let entity = TestEntity {
        id: 0,
        name: "test all entity".to_owned()
    };
    ids.push(Dm::insert(&entity));

    let entity = OtherEntity {
        name: "other test all entity".to_owned(),
        ..
        OtherEntity::default()
    };
    let other_id = Dm::insert(&entity);

    let name = "test all entity";
    let entities = Dm::<TestEntity>::find().by(json!({ "name": name })).all();
    assert_eq!(2, entities.len());
    for entity in entities {
        assert!(ids.contains(&entity.id));
        assert_eq!(name, entity.name);
    }

    let name = "other test all entity";
    let entities = Dm::<OtherEntity>::find().by(json!({ "name": name })).all();
    assert_eq!(1, entities.len());
    assert_eq!(other_id, entities[0].id);
    assert_eq!(name, entities[0].name);
}

#[test]
fn update()
{
    let entity = TestEntity {
        id: 0,
        name: "test entity".to_owned()
    };
    let id = Dm::insert(&entity);
    assert!(0 < id);

    let updated = Dm::update(&entity);
    assert!(updated == false);

    let name = "new";
    let mut entity = entity;
    entity.set_id(id);
    entity.name = name.to_owned();

    let updated = Dm::update(&entity);
    assert!(updated);

    let entity = Dm::<TestEntity>::find().by(json!({ "id": entity.id() })).one();
    assert_eq!(name, entity.name);

    let entity = TestEntity {
        id: 10100,
        name: "test entity 2".to_owned()
    };
    let updated = Dm::update(&entity);
    assert!(updated == false);
}

#[test]
fn save()
{
    let name = "test entity";
    let mut entity = TestEntity {
        id: 0,
        name: name.to_owned()
    };
    assert!(Dm::save(&mut entity));
    assert!(0 < entity.id());

    let mut entity = Dm::<TestEntity>::find().by(json!({ "id": entity.id() })).one();
    assert_eq!(name, entity.name);

    let name = "new";
    entity.name = name.to_owned();
    assert!(Dm::save(&mut entity));

    let entity = Dm::<TestEntity>::find().by(json!({ "id": entity.id() })).one();
    assert_eq!(name, entity.name);
}

#[test]
fn insert_or_update()
{
    let name = "test entity";
    let entity = TestEntity {
        id: 0,
        name: name.to_owned()
    };
    let id = Dm::insert_or_update(&entity);
    assert!(0 < id);

    let mut entity = Dm::<TestEntity>::find().by(json!({ "id": id })).one();
    assert_eq!(name, entity.name);

    let name = "new";
    entity.name = name.to_owned();
    Dm::insert_or_update(&entity);

    let entity = Dm::<TestEntity>::find().by(json!({ "id": entity.id() })).one();
    assert_eq!(name, entity.name);
}

#[test]
fn delete()
{
    let name = "test del entity";
    let mut entity = TestEntity {
        id: 0,
        name: name.to_owned()
    };
    assert!(Dm::create(&mut entity));
    let id = entity.id();
    assert!(0 < id);

    assert!(Dm::delete(&entity));

    {
        let entity = Dm::<TestEntity>::find().by(json!({ "id": id, "name": name })).one();
        assert_eq!(0, entity.id());
    }

    assert!(Dm::save(&mut entity));
    assert!(id != entity.id());

    let entity = Dm::<TestEntity>::find().by(json!({ "id": entity.id() })).one();
    assert_eq!(name, entity.name);
    assert!(id != entity.id());
}