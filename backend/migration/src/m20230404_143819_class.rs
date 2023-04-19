use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Class::Table)
                    .col(
                        ColumnDef::new(Class::Id)
                            .unsigned()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Class::Name).string_len(5).not_null())
                    .col(ColumnDef::new(Class::TeacherId).unsigned().not_null())
                    .col(ColumnDef::new(Class::CreateAt).date_time().not_null())
                    .col(ColumnDef::new(Class::UpdateAt).date_time().not_null())
                    .index(
                        Index::create()
                            .unique()
                            .name("idx-teacher-id")
                            .col(Class::TeacherId),
                    )
                    .if_not_exists()
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Class::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum Class {
    Table,
    Id,
    Name,
    TeacherId,
    CreateAt,
    UpdateAt,
}
