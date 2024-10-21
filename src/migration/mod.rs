pub use sea_orm_migration::prelude::*;

mod m20220101_000001_create_expense_income;
mod m20220101_000002_change_null_to_notnull_expense_description;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_create_expense_income::Migration),
            Box::new(m20220101_000002_change_null_to_notnull_expense_description::Migration),
        ]
    }
}
