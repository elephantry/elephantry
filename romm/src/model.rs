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

    fn create_entity(row: postgres::rows::Row<'_>) -> Self::Entity
    {
        let projection = Self::create_projection();
        let mut data = std::collections::HashMap::<&'static str, (postgres::types::Type, Vec<u8>)>::new();

        for (name, crate::Row {ty, .. }) in projection.fields {
            if let Some(bytes) = row.get_bytes(name) {
                data.insert(name, (ty, bytes.to_vec()));
            }
        }

        <Self::Entity as crate::Entity>::from(&data)
    }
}
