use std::collections::HashMap;

/**
 * Impl this trait to create a link between an entity and a structure.
 */
pub trait Model<'a> {
    type Entity: crate::Entity;
    type Structure: crate::Structure;

    fn new(connection: &'a crate::Connection) -> Self;

    /**
     * This method creates a projection based on the structure definition of
     * the underlying relation.
     *
     * This method can be used where a projection that sticks to table
     * definition is needed like recursive CTEs. For normal projections, use
     * [`create_projection`] instead.
     *
     * [`create_projection`]: #method.create_projection
     */
    fn default_projection() -> crate::Projection {
        use crate::Structure;

        crate::Projection::new(&Self::Structure::relation(), &Self::Structure::columns())
    }

    /**
     * This is a helper to create a new projection according to the current
     * structure. Overriding this method will change projection for all models.
     */
    fn create_projection() -> crate::Projection {
        Self::default_projection()
    }

    /**
     * Create a new entity.
     */
    fn create_entity(tuple: &crate::Tuple<'_>) -> Self::Entity {
        <Self::Entity as crate::Entity>::from(&tuple)
    }

    fn primary_key(entity: &Self::Entity) -> HashMap<&'static str, &dyn crate::ToSql> {
        use crate::Entity;
        use crate::Structure;

        let mut pk = HashMap::new();

        for field in Self::Structure::primary_key() {
            pk.insert(*field, entity.get(field).unwrap());
        }

        pk
    }
}
