use std::collections::HashMap;

pub trait Model<'a> {
    type Entity: crate::Entity;
    type Structure: crate::Structure;

    fn new(connection: &'a crate::Connection) -> Self;

    fn default_projection() -> crate::Projection {
        use crate::Structure;

        crate::Projection::new(&Self::Structure::definition())
    }

    fn create_projection() -> crate::Projection {
        Self::default_projection()
    }

    fn create_entity(tuple: &crate::pq::Tuple) -> Self::Entity {
        <Self::Entity as crate::Entity>::from(&tuple)
    }

    fn primary_key(entity: &Self::Entity) -> HashMap<&'static str, &dyn crate::pq::ToSql> {
        use crate::Structure;
        use crate::Entity;

        let mut pk = HashMap::new();

        for field in Self::Structure::primary_key() {
            pk.insert(*field, entity.get(field).unwrap());
        }

        pk
    }
}
