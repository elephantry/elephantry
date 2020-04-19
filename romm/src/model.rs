use std::collections::HashMap;

pub trait Model
{
    type Entity: crate::Entity;
    type RowStructure: crate::RowStructure;

    fn default_projection() -> crate::Projection
    {
        use crate::RowStructure;

        crate::Projection::new(&Self::RowStructure::definition())
    }

    fn create_projection() -> crate::Projection
    {
        Self::default_projection()
    }

    fn create_entity(row: &crate::pq::Row) -> Self::Entity
    {
        <Self::Entity as crate::Entity>::from(&row)
    }

    fn primary_key(entity: &Self::Entity) -> HashMap<&'static str, &dyn crate::pq::ToSql>  {
        use crate::Entity;
        use crate::RowStructure;

        let mut pk = HashMap::new();

        for field in Self::RowStructure::primary_key() {
            pk.insert(*field, entity.get(field).unwrap());
        }

        pk
    }
}
