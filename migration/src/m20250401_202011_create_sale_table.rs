use sea_orm_migration::prelude::*;

use crate::{m20250401_191153_create_user_table::User, m20250403_191648_create_class_table::Class};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Sale::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Sale::Uuid).uuid().primary_key())
                    .col(ColumnDef::new(Sale::IdMercadoPago).unsigned())
                    .col(ColumnDef::new(Sale::TotalPrice).decimal().not_null())
                    .col(ColumnDef::new(Sale::Username).string().not_null())
                    .col(ColumnDef::new(Sale::IdUserClass).uuid().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-sale-class-uuid")
                            .from(Sale::Table, Sale::IdUserClass)
                            .to(Class::Table, Class::Uuid),
                    )
                    .col(ColumnDef::new(Sale::IdDeliveryman).unsigned())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-sale-user-id")
                            .from(Sale::Table, Sale::IdDeliveryman)
                            .to(User::Table, User::Id),
                    )
                    .col(
                        ColumnDef::new(Sale::MoneyKept)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(
                        ColumnDef::new(Sale::RequiredRefunded)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(
                        ColumnDef::new(Sale::WasRefunded)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(
                        ColumnDef::new(Sale::CreatedAt)
                            .date()
                            .not_null()
                            .default(Expr::current_date()),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Sale::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Sale {
    Table,
    Uuid,
    IdMercadoPago,
    TotalPrice,
    Username,
    IdUserClass,
    IdDeliveryman,
    MoneyKept,
    RequiredRefunded,
    WasRefunded,
    CreatedAt,
}
