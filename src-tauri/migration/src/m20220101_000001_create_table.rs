use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Peer::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Peer::LocalPeerId)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Peer::PeerId).string().unique_key().not_null())
                    .col(ColumnDef::new(Peer::Name).string())
                    .col(ColumnDef::new(Peer::HostName).string())
                    .col(ColumnDef::new(Peer::Status).small_unsigned().not_null())
                    .col(ColumnDef::new(Peer::LastSeen).date_time().not_null())
                    .to_owned(),
            )
            .await?;
        manager
            .create_table(
                Table::create()
                    .table(Endpoint::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Endpoint::LocalPeerId)
                            .integer()
                            .not_null()
                        ,
                    )
                    .col(ColumnDef::new(Endpoint::Multiaddr).string().not_null())
                    .primary_key(
                        Index::create()
                            .col(Endpoint::LocalPeerId)
                            .col(Endpoint::Multiaddr))
                    .foreign_key(
                        ForeignKey::create()
                            .from(Endpoint::Table, Endpoint::LocalPeerId)
                            .to(Peer::Table, Peer::LocalPeerId)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::NoAction))
                    .to_owned()
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Endpoint::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Peer::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Peer {
    Table,
    LocalPeerId,
    PeerId,
    Name,
    HostName,
    Status,
    LastSeen,
}

#[derive(DeriveIden)]
enum Endpoint {
    Table,
    LocalPeerId,
    Multiaddr,
}