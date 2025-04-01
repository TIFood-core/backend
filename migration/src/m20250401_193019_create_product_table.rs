use sea_orm_migration::prelude::*;

use crate::m20250401_191153_create_user_table::User;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Product::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Product::Uuid).uuid().primary_key())
                    .col(ColumnDef::new(Product::Name).string().not_null())
                    .col(ColumnDef::new(Product::Image).string())
                    .col(ColumnDef::new(Product::Description).text().not_null())
                    .col(ColumnDef::new(Product::CreatedBy).unsigned().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-product-user-id")
                            .from(Product::Table, Product::CreatedBy)
                            .to(User::Table, User::Id),
                    )
                    .col(
                        ColumnDef::new(Product::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Product::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Product {
    Table,
    Uuid,
    Name,
    Image,
    Description,
    CreatedBy,
    CreatedAt,
}
