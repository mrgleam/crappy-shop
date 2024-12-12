use sea_orm_migration::{prelude::*, schema::*};

use crate::m20241205_083431_create_table_user::User;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Cart::Table)
                    .if_not_exists()
                    .col(pk_auto(Cart::Id))
                    .col(ColumnDef::new(Cart::UserId).integer().not_null())
                    .col(
                        ColumnDef::new(Cart::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(Cart::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(Cart::Table, Cart::UserId)
                            .to(User::Table, User::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .index(
                        Index::create()
                            .name("user_id_key")
                            .col(Cart::UserId)
                            .unique(),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Cart::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Cart {
    Table,
    Id,
    UserId,
    CreatedAt,
    UpdatedAt,
}
