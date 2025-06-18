use serenity::all::{MessageBuilder, UserId};

use crate::Data;

#[derive(sqlx::FromRow, Debug)]
struct ReviveInfo {
    id: i64,
    tokens: i64,
    revive_tokens: i64,
    revive_count: i32,
}

pub async fn handle_revive(data: &Data, user_id: UserId) -> Result<String, sqlx::Error> {
    let res: Option<ReviveInfo> = sqlx::query_as(
        r#"
        select 
            u.id,
            u.tokens,
            u.revive_tokens,
            s.revive_count
        from gamble.users u
        left join gamble.user_stat s
        on u.id = s.id
        where u.id = $1;
        "#,
    )
    .bind(user_id.get() as i64)
    .fetch_optional(&data.dbpool)
    .await?;

    Ok(match res {
        None => MessageBuilder::new()
            .push("You are not registered.\n")
            .push("Use `.register` to get registered.")
            .build(),

        Some(revive_info) => match revive_info.tokens {
            0 => {
                println!("{:?}", revive_info);
                println!("Updating user {:?} in gamble.users", revive_info.id);
                let res = sqlx::query(
                    r#"
                    update gamble.users
                    set 
                        tokens = $1
                    where id = $2
                    "#,
                )
                .bind(revive_info.revive_tokens)
                .bind(revive_info.id)
                .execute(&data.dbpool)
                .await?;

                println!("Affected rows: {}", res.rows_affected());
                println!("Updating user {:?} in gamble.user_stat", revive_info.id);
                let res = sqlx::query(
                    r#"
                    update gamble.user_stat
                    set 
                    revive_count = $1
                    where id = $2
                    "#,
                )
                .bind(revive_info.revive_count + 1)
                .bind(revive_info.id)
                .execute(&data.dbpool)
                .await?;

                println!("Affected rows: {}", res.rows_affected());
                MessageBuilder::new()
                    .push("You")
                    .push_bold_line(" revived.")
                    .push(format!("You have {} tokens.", revive_info.revive_tokens))
                    .build()
            }

            _ => MessageBuilder::new()
                .push("You cannot revive with non-zero tokens.\n")
                .push(format!("You have {} tokens.", revive_info.tokens))
                .build(),
        },
    })
}
