use entity::product;
use fake::faker::lorem::en::*;
use fake::Fake;
use sea_orm::{EntityTrait, Set};
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        let mut products = Vec::new();

        for _ in 1..11 {
            let product = product::ActiveModel {
                name: Set(Word().fake()),
                price: Set(((8.0..20.0).fake::<f32>() * 100.0).round() / 100.0),
                description: Set(Paragraph(1..3).fake()),
                ..Default::default()
            };

            products.push(product);
        }

        product::Entity::insert_many(products).exec(db).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        product::Entity::delete_many().exec(db).await?;

        Ok(())
    }
}
