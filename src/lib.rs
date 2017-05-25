
pub trait Identifiable
{
    type ID;

    fn id(&self) -> Self::ID;
    fn set_id(&mut self, id: Self::ID) -> &mut Self;
}

pub trait Connection
{

}

pub trait Query
{
    type Entity: Identifiable;
    type Criteria;

    fn from<C: Connection>(db: C) -> Self;
    fn one(self) -> Self::Entity;
    fn all(self) -> Vec<Self::Entity>;
    fn by(self, criteria: Self::Criteria) -> Self;
    fn limit(&self, limit: u32) -> Self;
    fn offset(&self, offset: u32) -> Self;
    fn count(&self) -> u32;
    fn exists(&self) -> bool;
}

pub trait DataMapper<'a, ID, Q: Query>
{
    type Entity: Identifiable<ID = ID>;
    type Action;

    fn query(action: Self::Action) -> Q;
    fn find() -> Q;
    fn create(entity: &'a mut Self::Entity) -> bool;
    fn insert(entity: &'a Self::Entity) -> ID;
    fn update(entity: &'a Self::Entity) -> bool;
    fn save(entity: &'a mut Self::Entity) -> bool;
    fn insert_or_update(entity: &'a Self::Entity) -> ID;
    fn delete(entity: &'a Self::Entity) -> bool;
}