use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .create_table(
                Table::create()
                    .table(UserCredential::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(UserCredential::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(UserCredential::Username)
                            .string()
                            .unique_key()
                            .not_null()
                    )
                    .col(
                        ColumnDef::new(UserCredential::Password)
                            .string()
                            .not_null()
                    )
                    .col(
                        ColumnDef::new(UserCredential::FullName)
                            .string()
                            .not_null()
                    )
                    .col(
                        ColumnDef::new(UserCredential::Email)
                            .string()
                            .unique_key()
                            .not_null()
                    )
                    .col(
                        ColumnDef::new(UserCredential::CreatedAt)
                            .timestamp()
                            .not_null()
                            .default(Expr::current_timestamp())
                    )
                    .col(
                        ColumnDef::new(UserCredential::UpdatedAt)
                            .timestamp()
                            .not_null()
                            .default(Expr::current_timestamp())
                    )
                    .col(
                        ColumnDef::new(UserCredential::Deleted)
                            .boolean()
                            .not_null()
                            .default(false)
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .drop_table(Table::drop().table(Post::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum UserCredential{
    Table,
    Id,
    FullName,
    Username,
    Password,
    Email,
    CreatedAt,
    UpdatedAt,
    Deleted
}

#[derive(DeriveIden)]
enum Post {
    Table,
    Id,
    Title,
    Text,
}
