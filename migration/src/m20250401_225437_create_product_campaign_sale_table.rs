use sea_orm_migration::prelude::*;

use crate::{
    m20250401_195729_create_product_campaign_table::ProductCampaign,
    m20250401_202011_create_sale_table::Sale,
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(ProductCampaignSale::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(ProductCampaignSale::Id)
                            .unsigned()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(ProductCampaignSale::Price)
                            .decimal()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(ProductCampaignSale::Amount)
                            .unsigned()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(ProductCampaignSale::IdProductCampaign)
                            .uuid()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-product_campaign_sale-product_campaing-uuid")
                            .from(
                                ProductCampaignSale::Table,
                                ProductCampaignSale::IdProductCampaign,
                            )
                            .to(ProductCampaign::Table, ProductCampaign::Uuid),
                    )
                    .col(
                        ColumnDef::new(ProductCampaignSale::IdSale)
                            .uuid()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-product_campaign_sale-sale-uuid")
                            .from(ProductCampaignSale::Table, ProductCampaignSale::IdSale)
                            .to(Sale::Table, Sale::Uuid),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(ProductCampaignSale::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum ProductCampaignSale {
    Table,
    Id,
    Price,
    Amount,
    IdProductCampaign,
    IdSale,
}
