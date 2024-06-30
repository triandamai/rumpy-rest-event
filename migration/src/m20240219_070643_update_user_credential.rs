use sea_orm_migration::prelude::*;
use sea_orm_migration::prelude::extension::postgres::Type;
use sea_orm_migration::sea_orm::{EnumIter, Iterable};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .create_type(
                Type::create()
                    .as_enum(AuthProvider::Table)
                    .values(AuthProvider::iter().skip(1))
                    .to_owned()
            ).await?;

        manager
            .create_type(
                Type::create()
                    .as_enum(UserStatus::Table)
                    .values(UserStatus::iter().skip(1))
                    .to_owned()
            ).await?;


        manager
            .alter_table(
                Table::alter()
                    .table(UserCredential::Table)
                    .add_column_if_not_exists(
                        ColumnDef::new(AuthProvider::Table)
                            .not_null()
                            .enumeration(AuthProvider::Table, AuthProvider::iter().skip(1))
                    ).to_owned()
            ).await?;

        manager
            .alter_table(
                Table::alter()
                    .table(UserCredential::Table)
                    .add_column_if_not_exists(
                        ColumnDef::new(UserStatus::Table)
                            .not_null()
                            .enumeration(UserStatus::Table, UserStatus::iter().skip(1))
                    ).to_owned()
            ).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        Ok(())
    }
}


#[derive(DeriveIden)]
enum UserCredential {
    Table,
    Id,
    FullName,
    Username,
    Password,
    Email,
    AuthProvider,
    Status,
    CreatedAt,
    UpdatedAt,
    Deleted,
}

#[derive(Iden, EnumIter)]
enum AuthProvider {
    Table,
    #[iden = "GOOGLE"]
    GOOGLE,
    #[iden = "BASIC"]
    BASIC,
    #[iden = "FACEBOOK"]
    FACEBOOK,
    #[iden = "APPLE"]
    APPLE,
    #[iden = "GITHUB"]
    GITHUB,
}

#[derive(Iden, EnumIter)]
enum UserStatus {
    Table,
    #[iden = "ACTIVE"]
    ACTIVE,
    #[iden = "WAITING_CONFIRMATION"]
    WAITING_CONFIRMATION,
    #[iden = "INACTIVE"]
    INACTIVE,
    #[iden = "SUSPENDED"]
    SUSPENDED,
    #[iden = "LOCK"]
    LOCK,
}