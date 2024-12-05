pub use sea_orm_migration::prelude::*;

mod m20241205_083431_create_table_user;
mod m20241205_084432_create_table_product;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20241205_083431_create_table_user::Migration),
            Box::new(m20241205_084432_create_table_product::Migration),
        ]
    }
}
