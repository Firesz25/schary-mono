use crate::m20230405_163933_user_permision::UserPermision;
use sea_orm_migration::prelude::*;
#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(User::Table)
                    .col(
                        ColumnDef::new(User::Id)
                            .unsigned()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(User::Login).string_len(10).not_null())
                    .col(ColumnDef::new(User::Name).string_len(35).not_null())
                    .col(ColumnDef::new(User::Surname).string_len(35).not_null())
                    .col(ColumnDef::new(User::Password).string_len(255).not_null())
                    .col(ColumnDef::new(User::Phone).string_len(12).not_null())
                    .col(ColumnDef::new(User::Email).string_len(40).not_null())
                    .col(ColumnDef::new(User::Active).boolean().not_null())
                    .col(ColumnDef::new(User::CreateAt).date_time().not_null())
                    .col(ColumnDef::new(User::UpdateAt).date_time().not_null())
                    .index(
                        Index::create()
                            .unique()
                            .name("idx-user-login")
                            .col(User::Login),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-user_id-user_permision_user_id")
                            .from(User::Table, User::Id)
                            .to(UserPermision::Table, UserPermision::UserId),
                    )
                    .if_not_exists()
                    .to_owned(),
            )
            .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(User::Table).to_owned())
            .await?;
        Ok(())
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
pub(crate) enum User {
    Table,
    Id,
    Login,
    Name,
    Surname,
    Password,
    Phone,
    Email,
    Active,
    CreateAt,
    UpdateAt,
}
