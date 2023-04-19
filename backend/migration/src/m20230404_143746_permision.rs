use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Permision::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Permision::Id)
                            .unsigned()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(Permision::Permision)
                            .string_len(40)
                            .not_null(),
                    )
                    // .index(Index::create().if_not_exists().name("idx-permision_id").col(Permision::Id))
                    .to_owned(),
            )
            .await
            .unwrap();
        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx-permision_id")
                    .col(Permision::Id)
                    .table(Permision::Table)
                    .to_owned(),
            )
            .await
            .unwrap();
        let insert = Query::insert()
            .into_table(Permision::Table)
            .columns([Permision::Permision])
            .values_panic(["root".into()])
            .values_panic(["admin".into()])
            .values_panic(["teacher".into()])
            .values_panic(["student".into()])
            .values_panic(["parent".into()])
            .to_owned();
        manager.exec_stmt(insert).await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_index(
                Index::drop()
                    .table(Permision::Table)
                    .name("idx-permision_id")
                    .to_owned(),
            )
            .await?;
        manager
            .drop_table(Table::drop().table(Permision::Table).to_owned())
            .await?;
        Ok(())
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
pub(crate) enum Permision {
    Table,
    Id,
    Permision,
}
