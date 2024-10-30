// use openssl::{pkey::PKey, rsa::Rsa};
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[derive(DeriveIden)]
pub enum Privkey {
    Table,
    PrivkeyId,
    Key,
    CreationDate,
    IsEnabled,
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Privkey::Table)
                    .col(
                        ColumnDef::new(Privkey::PrivkeyId)
                            .integer()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Privkey::Key).text())
                    .col(ColumnDef::new(Privkey::CreationDate).timestamp())
                    .col(ColumnDef::new(Privkey::IsEnabled).boolean())
                    .to_owned(),
            )
            .await
            .unwrap();

        // Generate a keypair
        // let rsa = Rsa::generate(2048).unwrap();
        // let keypair = PKey::from_rsa(rsa).unwrap();

        // // Private key as pem String
        // let prv_key = String::from_utf8(keypair.private_key_to_pem_pkcs8().unwrap()).unwrap();

        // let insert_initial_pkey = Query::insert()
        //     .into_table(Privkey::Table)
        //     .columns([Privkey::Key, Privkey::CreationDate, Privkey::IsEnabled])
        //     .values_panic([
        //         prv_key.into(),
        //         Func::cast_as(
        //             "2024-09-01 00:00:00-5",
        //             Alias::new("TIMESTAMP"),
        //         )
        //         .into(),
        //         true.into(),
        //     ])
        //     .to_owned();

        // manager.exec_stmt(insert_initial_pkey).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Privkey::Table).cascade().to_owned())
            .await
            .unwrap();

        Ok(())
    }
}
