use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Articles::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Articles::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Articles::Title).string_len(180).not_null())
                    .col(
                        ColumnDef::new(Articles::Slug)
                            .string_len(220)
                            .not_null()
                            .unique_key(),
                    )
                    .col(ColumnDef::new(Articles::BodyMarkdown).text().not_null())
                    .col(ColumnDef::new(Articles::BodyHtml).text().not_null())
                    .col(ColumnDef::new(Articles::Excerpt).text().not_null())
                    .col(ColumnDef::new(Articles::Description).text().not_null())
                    .col(ColumnDef::new(Articles::PlainText).text().not_null())
                    .col(ColumnDef::new(Articles::AuthorId).big_integer().not_null())
                    .col(ColumnDef::new(Articles::Status).string_len(32).not_null())
                    .col(ColumnDef::new(Articles::Source).string_len(32).not_null())
                    .col(
                        ColumnDef::new(Articles::Category)
                            .string_len(120)
                            .not_null(),
                    )
                    .col(ColumnDef::new(Articles::Tags).text().not_null())
                    .col(
                        ColumnDef::new(Articles::HasCode)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(
                        ColumnDef::new(Articles::HasMath)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(ColumnDef::new(Articles::PublishedAt).timestamp().null())
                    .col(
                        ColumnDef::new(Articles::CreatedAt)
                            .timestamp()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(Articles::UpdatedAt)
                            .timestamp()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_articles_status_published")
                    .table(Articles::Table)
                    .col(Articles::Status)
                    .col(Articles::PublishedAt)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_articles_author")
                    .table(Articles::Table)
                    .col(Articles::AuthorId)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Articles::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Articles {
    Table,
    Id,
    Title,
    Slug,
    BodyMarkdown,
    BodyHtml,
    Excerpt,
    Description,
    PlainText,
    AuthorId,
    Status,
    Source,
    Category,
    Tags,
    HasCode,
    HasMath,
    PublishedAt,
    CreatedAt,
    UpdatedAt,
}
