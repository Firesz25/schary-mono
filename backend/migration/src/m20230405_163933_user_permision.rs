use crate::m20230404_143746_permision::Permision;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(UserPermision::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(UserPermision::UserId)
                            .unsigned()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(UserPermision::PermisionId)
                            .unsigned()
                            .not_null(),
                    )
                    // .index(
                    //     Index::create()
                    //         .name("idx-user_permision-permision_id")
                    //         .col(UserPermision::PermisionId)
                    //         .table(UserPermision::Table),
                    // )
                    // .index(
                    //     Index::create()
                    //         .name("idx-user_permision-user_id")
                    //         .col(UserPermision::UserId)
                    //         .table(UserPermision::Table),
                    // )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-user_permision-")
                            .from(UserPermision::Table, UserPermision::PermisionId)
                            .to(Permision::Table, Permision::Id),
                    )
                    .to_owned(),
            )
            .await?;
        manager
            .create_index(
                Index::create()
                    .name("idx-user_permision-permision_id")
                    .col(UserPermision::PermisionId)
                    .table(UserPermision::Table)
                    .to_owned(),
            )
            .await?;
        // manager
        //     .create_index(
        //         Index::create()
        //             .name("idx-user_permision-user_id")
        //             .col(UserPermision::UserId)
        //             .table(UserPermision::Table)
        //             .to_owned(),
        //     )
        //     .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // manager
        //     .drop_index(
        //         Index::drop()
        //             .table(UserPermision::Table)
        //             .name("idx-user_permision-permision_id")
        //             .to_owned(),
        //     )
        //     .await?;
        manager
            .drop_index(
                Index::drop()
                    .table(UserPermision::Table)
                    .name("idx-user_permision-user_id")
                    .to_owned(),
            )
            .await?;
        manager
            .drop_table(Table::drop().table(UserPermision::Table).to_owned())
            .await?;
        Ok(())
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
pub(crate) enum UserPermision {
    Table,
    UserId,
    PermisionId,
}
