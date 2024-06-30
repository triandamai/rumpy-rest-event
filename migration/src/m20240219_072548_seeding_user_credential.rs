use sea_orm_migration::prelude::*;
use uuid::uuid;
use crate::sea_orm::EnumIter;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        let insert = Query::insert()
            .into_table(UserCredential::Table)
            .columns([
                UserCredential::Id,
                UserCredential::FullName,
                UserCredential::Username,
                UserCredential::Password,
                UserCredential::Email,
                UserCredential::AuthProvider,
                UserCredential::Status
            ])
            .values_panic([
                uuid::Uuid::new_v4().to_string().into(),
                "Trian damai".into(),
                "triandamai".into(),
                "$2a$12$R0EIvnvgqZe12Gc8C3xQzu313ouJX.CsAJ6d8jZDPsTLEmBjAf6j2".into(),
                "triandamai@gmail.com".into(),
                "BASIC".into(),
                "ACTIVE".into()
            ])
            .to_owned();
        manager.exec_stmt(insert).await?;
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