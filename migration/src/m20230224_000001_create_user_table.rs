use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str{
        "m20230224_000001_create_User_table"
    }

}

#[async_trait::async_trait]
impl MigrationTrait for Migration{
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr>{
        manager.create_table(
            Table::create()
            .table(User::Table)
            .col(ColumnDef::new(User::Id)
            .integer()
            .not_null()
            .auto_increment()
            .primary_key()
            )
            .col(ColumnDef::new(User::Username)
            .string_len(64)
            .not_null()
            )
            .col(ColumnDef::new(User::Password)
            .string_len(64)
            .not_null()
            )
            .col(ColumnDef::new(User::DeletedAt)
            .timestamp_with_time_zone()
            )
            .col(ColumnDef::new(User::Token)
            .string()
            )
            .col(ColumnDef::new(User::Salt)
            .string_len(30)
            .not_null()
            )
            .to_owned()
        ).await

        
        
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr>{
        manager.drop_table(Table::drop().table(User::Table).to_owned()).await
    }
}


#[derive(Iden)]
pub enum User{
    Table,
    Id,
    Username,
    Password,
    Salt,
    DeletedAt,
    Token
}

