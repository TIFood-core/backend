use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(User::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(User::Id)
                            .unsigned()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(User::Email).string().unique_key().not_null())
                    .col(ColumnDef::new(User::Password).string().not_null())
                    .col(
                        ColumnDef::new(User::IsAdmin)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(ColumnDef::new(User::Activation).uuid().unique_key())
                    .col(
                        ColumnDef::new(User::ActivatorGenerationDate)
                            .date()
                            .default(Expr::current_date()),
                    )
                    .col(
                        ColumnDef::new(User::IsBanned)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(
                        ColumnDef::new(User::CreatedAt)
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
            .drop_table(Table::drop().table(User::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum User {
    Table,
    Id,
    Email,
    Password,
    IsAdmin,
    Activation,
    ActivatorGenerationDate,
    IsBanned,
    CreatedAt,
}
