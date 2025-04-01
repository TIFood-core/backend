pub use sea_orm_migration::prelude::*;

mod m20250401_191153_create_user_table;
mod m20250401_193019_create_product_table;
mod m20250401_195229_create_campaign_table;
mod m20250401_195729_create_product_campaign_table;
mod m20250401_202011_create_sale_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20250401_191153_create_user_table::Migration),
            Box::new(m20250401_193019_create_product_table::Migration),
            Box::new(m20250401_195229_create_campaign_table::Migration),
            Box::new(m20250401_195729_create_product_campaign_table::Migration),
            Box::new(m20250401_202011_create_sale_table::Migration),
        ]
    }
}
