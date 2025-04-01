use sea_orm_migration::prelude::*;

use crate::{
    m20250401_193019_create_product_table::Product,
    m20250401_195229_create_campaign_table::Campaign,
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(ProductCampaign::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(ProductCampaign::Uuid).uuid().primary_key())
                    .col(ColumnDef::new(ProductCampaign::Price).decimal().not_null())
                    .col(ColumnDef::new(ProductCampaign::Units).unsigned().not_null())
                    .col(
                        ColumnDef::new(ProductCampaign::ExtraUnits)
                            .unsigned()
                            .not_null(),
                    )
                    .col(ColumnDef::new(ProductCampaign::IdProduct).uuid().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-product_campaign-product-uuid")
                            .from(ProductCampaign::Table, ProductCampaign::IdProduct)
                            .to(Product::Table, Product::Uuid),
                    )
                    .col(
                        ColumnDef::new(ProductCampaign::IdCampaign)
                            .uuid()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-product_campaign-campaign-uuid")
                            .from(ProductCampaign::Table, ProductCampaign::IdCampaign)
                            .to(Campaign::Table, Campaign::Uuid),
                    )
                    .col(
                        ColumnDef::new(ProductCampaign::CreatedAt)
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
            .drop_table(Table::drop().table(ProductCampaign::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum ProductCampaign {
    Table,
    Uuid,
    Price,
    Units,
    ExtraUnits,
    IdProduct,
    IdCampaign,
    CreatedAt,
}
