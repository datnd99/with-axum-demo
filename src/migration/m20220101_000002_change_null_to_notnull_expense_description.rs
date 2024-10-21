use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(TExpenseIncome::Table)
                    .modify_column(
                        ColumnDef::new(TExpenseIncome::Description)
                            .not_null(), 
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {


        manager
            .alter_table(
                Table::alter()
                    .table(TExpenseIncome::Table)
                    .modify_column(
                        ColumnDef::new(TExpenseIncome::Description)
                            .null(), 
                    )
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
enum TExpenseIncome {
    Table,
    Description,
}
