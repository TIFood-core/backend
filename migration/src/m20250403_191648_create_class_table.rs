use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Class::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Class::Uuid).uuid().primary_key())
                    .col(ColumnDef::new(Class::Name).string().not_null())
                    .col(
                        ColumnDef::new(Class::CreatedAt)
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
            .drop_table(Table::drop().table(Class::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Class {
    Table,
    Uuid,
    Name,
    CreatedAt,
}
