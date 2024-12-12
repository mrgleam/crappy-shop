pub use sea_orm_migration::prelude::*;

mod m20241205_083431_create_table_user;
mod m20241205_084432_create_table_product;
mod m20241208_041250_seed_example_products;
mod m20241210_100816_add_table_cart;
mod m20241210_101849_add_table_cart_item;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20241205_083431_create_table_user::Migration),
            Box::new(m20241205_084432_create_table_product::Migration),
            Box::new(m20241208_041250_seed_example_products::Migration),
            Box::new(m20241210_100816_add_table_cart::Migration),
            Box::new(m20241210_101849_add_table_cart_item::Migration),
        ]
    }
}
