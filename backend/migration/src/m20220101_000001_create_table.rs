use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let table = Table::create()
            .table(Todo::Table)
            .if_not_exists()
            .col(ColumnDef::new(Todo::Id).uuid().not_null().primary_key())
            .col(ColumnDef::new(Todo::Text).string().not_null())
            .col(
                ColumnDef::new(Todo::Done)
                    .boolean()
                    .not_null()
                    .default(Expr::value(false)),
            )
            .to_owned();

        manager.create_table(table).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Todo::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Todo {
    Table,
    Id,
    Text,
    Done,
}
