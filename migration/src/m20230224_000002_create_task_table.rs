use sea_orm_migration::prelude::*;

use crate::m20230224_000001_create_user_table::User;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str{
        "m20230224_000002_create_Task_table"
    }

}

#[async_trait::async_trait]
impl MigrationTrait for Migration{

    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    
        manager.create_table(
            Table::create()
            .table(Task::Table)
            .col(ColumnDef::new(Task::Id)
            .integer()
            .not_null()
            .auto_increment()
            .primary_key()
            )
            .col(ColumnDef::new(Task::Priority)
            .string_len(4)
            .null()
            )
            .col(ColumnDef::new(Task::Title)
                .string_len(255)
                .not_null()
            )
            .col(ColumnDef::new(Task::CompletedAt)
                .timestamp_with_time_zone()
            )
            .col(ColumnDef::new(Task::Description)
                .text()
            )
            .col(ColumnDef::new(Task::DeletedAt)
                .timestamp_with_time_zone()
            )
            .col(ColumnDef::new(Task::IsDefault)
                .boolean()
                .default(false)
            )
            .col(ColumnDef::new(Task::UserId)
                .integer()
            )
            .foreign_key(
                ForeignKey::create()
                .name("fk-task-user_id")
                .from(Task::Table, Task::UserId)
                .to(User::Table, User::Id)
            )
            .to_owned()
        ).await


    }

    async fn down(&self, manager: &SchemaManager) -> Result<(),DbErr>
    {
        manager.drop_table(Table::drop().table(Task::Table).to_owned()).await
    }



}

#[derive(Iden)]
pub enum Task{
    Table,
    Id,
    Priority,
    Title,
    CompletedAt,
    Description,
    DeletedAt,
    UserId,
    IsDefault
}