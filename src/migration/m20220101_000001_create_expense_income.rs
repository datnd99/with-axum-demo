use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
 

        manager
            .create_table(
                Table::create()
                    .table(TExpenseIncome::Table)
                    .col(
                        ColumnDef::new(TExpenseIncome::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key()
                            .comment("Unique identifier for each record"), 
                    )
                    .col(
                        ColumnDef::new(TExpenseIncome::Category)
                            .integer()  
                            .not_null()
                            .default(0)
                            .comment("0 for Khoản Chi (Expense), 1 for Khoản Thu (Income)"), 
                    )
                    .col(
                        ColumnDef::new(TExpenseIncome::UnitPrice)
                            .decimal()
                            .not_null()
                            .comment("Price per unit"), 
                    )
                    .col(
                        ColumnDef::new(TExpenseIncome::Description)
                            .string_len(255)
                            .comment("Description of the expense or income"), 
                    )
                    .col(
                        ColumnDef::new(TExpenseIncome::Quantity)
                            .integer()
                            .not_null()
                            .comment("Quantity of items involved"), 
                    )
                    .col(
                        ColumnDef::new(TExpenseIncome::CreatedBy)
                            .string_len(100)
                            .not_null()
                            .comment("Name of the person who created the record"), 
                    )
                    .col(
                        ColumnDef::new(TExpenseIncome::CreatedAt)
                            .date_time()
                            .not_null()
                            .comment("Timestamp of when the record was created"), 
                    )
                    .col(
                        ColumnDef::new(TExpenseIncome::ApplicableTo)
                            .string_len(100)
                            .comment("Applicable to which entity or category"), 
                    )
                    .col(
                        ColumnDef::new(TExpenseIncome::Status)
                            .string_len(20)
                            .not_null()
                            .comment("Status of the record"), 
                    )
                    .col(
                        ColumnDef::new(TExpenseIncome::Note)
                            .string_len(255)
                            .comment("Additional notes about the record"), 
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {


        manager
            .drop_table(Table::drop().table(TExpenseIncome::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum TExpenseIncome {
    Table,
    Id,
    Category,
    UnitPrice,
    Description,
    Quantity,
    CreatedBy,
    CreatedAt,
    ApplicableTo,
    Status,
    Note,
}
