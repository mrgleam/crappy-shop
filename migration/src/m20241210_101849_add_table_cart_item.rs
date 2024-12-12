use sea_orm_migration::{prelude::*, schema::*};

use crate::{
    m20241205_084432_create_table_product::Product, m20241210_100816_add_table_cart::Cart,
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(CartItem::Table)
                    .if_not_exists()
                    .col(pk_auto(CartItem::Id))
                    .col(ColumnDef::new(CartItem::CartId).integer().not_null())
                    .col(ColumnDef::new(CartItem::ProductId).integer().not_null())
                    .col(
                        ColumnDef::new(CartItem::Quantity)
                            .small_integer()
                            .not_null()
                            .check(
                                Expr::col(CartItem::Quantity)
                                    .gt(0)
                                    .and(Expr::col(CartItem::Quantity).lt(255)),
                            ),
                    )
                    .col(
                        ColumnDef::new(CartItem::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(CartItem::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(CartItem::Table, CartItem::CartId)
                            .to(Cart::Table, Cart::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(CartItem::Table, CartItem::CartId)
                            .to(Product::Table, Product::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .index(
                        Index::create()
                            .name("cart_id_product_id_key")
                            .col(CartItem::CartId)
                            .col(CartItem::ProductId)
                            .unique(),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(CartItem::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum CartItem {
    Table,
    Id,
    CartId,
    ProductId,
    Quantity,
    CreatedAt,
    UpdatedAt,
}
