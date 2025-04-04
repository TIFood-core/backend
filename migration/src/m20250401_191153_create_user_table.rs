use std::env;

use argon2::{
    Argon2, PasswordHasher,
    password_hash::{SaltString, rand_core::OsRng},
};
use sea_orm_migration::{prelude::*, sea_orm::prelude::Date};

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
                    .col(ColumnDef::new(User::EmailActivation).uuid().unique_key())
                    .col(
                        ColumnDef::new(User::EmailActivatorGenerationDate)
                            .date()
                            .default(Expr::current_date()),
                    )
                    .col(
                        ColumnDef::new(User::IsActive)
                            .boolean()
                            .not_null()
                            .default(true),
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
            .unwrap();

        let db = manager.get_connection();

        let env_admin_email = env::var("ADMIN_EMAIL").expect("ADMIN_EMAIL not found at .env file");
        let env_admin_password =
            env::var("ADMIN_PASSWORD").expect("ADMIN_PASSWORD not found at .env file");

        let password_hash = Argon2::default()
            .hash_password(
                env_admin_password.as_bytes(),
                &SaltString::generate(&mut OsRng),
            )
            .unwrap()
            .to_string();

        db.execute(
            db.get_database_backend().build(
                Query::insert()
                    .into_table(User::Table)
                    .columns([
                        User::Email,
                        User::Password,
                        User::IsAdmin,
                        User::EmailActivatorGenerationDate,
                    ])
                    .values_panic([
                        env_admin_email.into(),
                        password_hash.into(),
                        true.into(),
                        None::<Date>.into(),
                    ]),
            ),
        )
        .await
        .unwrap();

        Ok(())
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
    EmailActivation,
    EmailActivatorGenerationDate,
    IsActive,
    CreatedAt,
}
