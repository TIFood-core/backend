use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Campaign::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Campaign::Uuid).uuid().primary_key())
                    .col(ColumnDef::new(Campaign::Date).date().not_null())
                    .col(
                        ColumnDef::new(Campaign::CreatedAt)
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
            .drop_table(Table::drop().table(Campaign::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Campaign {
    Table,
    Uuid,
    Date,
    CreatedAt,
}
